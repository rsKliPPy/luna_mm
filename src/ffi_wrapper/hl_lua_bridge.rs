use std::collections::HashMap;
use std::os::raw::{c_int, c_float};
use crate::meta_ffi::types::{
  Edict,
  EntVars,
  EngineStringHandle,
};
use super::{entvars_of_edict, string_from_handle, handle_from_string};

#[derive(Clone)]
pub struct EntityHandle {
  edict: *mut Edict,
  serial_number: c_int,
}

impl EntityHandle {
  pub fn new(edict: &Edict) -> Self {
    let edict = edict as *const Edict as *mut Edict;
    let serial_number = unsafe {
      edict.as_ref().map(Edict::serial_number).unwrap_or(0)
    };

    EntityHandle {
      edict,
      serial_number,
    }
  }

  pub fn get<'a>(&self) -> Option<&'a Edict> {
    let e = self.edict;
    let serial = self.serial_number;
    unsafe {
      if e.is_null() || (*e).serial_number() != serial || (*e).is_free() {
        None
      } else {
        Some(&*self.edict)
      }
    }
  }

  pub fn is_valid(&self) -> bool {
    self.get().is_some()
  }
}

impl Default for EntityHandle {
  fn default() -> Self {
    EntityHandle {
      edict: std::ptr::null_mut(),
      serial_number: 0,
    }
  }
}

impl rlua::UserData for EntityHandle {
  fn add_methods<'lua, M: rlua::UserDataMethods<'lua, Self>>(m: &mut M) {
    m.add_method("EntVars", |_, this: &Self, ()| {
      match this.is_valid() {
        true => Ok(EntVarsHandle::new(this.clone())),
        false => Err(rlua::Error::RuntimeError("Invalid entity".into())),
      }
    });
  }
}

unsafe impl Send for EntityHandle { }

#[derive(Clone)]
pub struct EntVarsHandle {
  entity_handle: EntityHandle,
}

impl EntVarsHandle {
  pub fn new(entity_handle: EntityHandle) -> Self {
    EntVarsHandle {
      entity_handle,
    }
  }
}

enum EntVarType {
  Int,
  Float,
  Bool,
  Byte,
  String,
  Vector3,
  EdictPtr,
}

lazy_static! {
  static ref ENTVARS: HashMap<&'static str, (usize, EntVarType)> = {
    let mut map = HashMap::new();
    map.insert("classname", (offset_of!(EntVars, classname), EntVarType::String));
    map.insert("globalname", (offset_of!(EntVars, globalname), EntVarType::String));
    map.insert("origin", (offset_of!(EntVars, origin), EntVarType::Vector3));
    map.insert("oldorigin", (offset_of!(EntVars, oldorigin), EntVarType::Vector3));
    map.insert("velocity", (offset_of!(EntVars, velocity), EntVarType::Vector3));
    map.insert("basevelocity", (offset_of!(EntVars, basevelocity), EntVarType::Vector3));
    map.insert("clbasevelocity", (offset_of!(EntVars, clbasevelocity), EntVarType::Vector3));
    map.insert("movedir", (offset_of!(EntVars, movedir), EntVarType::Vector3));
    map.insert("angles", (offset_of!(EntVars, angles), EntVarType::Vector3));
    map.insert("avelocity", (offset_of!(EntVars, avelocity), EntVarType::Vector3));
    map.insert("punchangle", (offset_of!(EntVars, punchangle), EntVarType::Vector3));
    map.insert("v_angle", (offset_of!(EntVars, v_angle), EntVarType::Vector3));
    map.insert("endpos", (offset_of!(EntVars, endpos), EntVarType::Vector3));
    map.insert("startpos", (offset_of!(EntVars, startpos), EntVarType::Vector3));
    map.insert("impacttime", (offset_of!(EntVars, impacttime), EntVarType::Float));
    map.insert("starttime", (offset_of!(EntVars, starttime), EntVarType::Float));
    map.insert("fixangle", (offset_of!(EntVars, fixangle), EntVarType::Int));
    map.insert("idealpitch", (offset_of!(EntVars, idealpitch), EntVarType::Float));
    map.insert("pitch_speed", (offset_of!(EntVars, pitch_speed), EntVarType::Float));
    map.insert("ideal_yaw", (offset_of!(EntVars, ideal_yaw), EntVarType::Float));
    map.insert("yaw_speed", (offset_of!(EntVars, yaw_speed), EntVarType::Float));
    map.insert("modelindex", (offset_of!(EntVars, modelindex), EntVarType::Int));
    map.insert("model", (offset_of!(EntVars, model), EntVarType::String));
    map.insert("viewmodel", (offset_of!(EntVars, viewmodel), EntVarType::Int));
    map.insert("weaponmodel", (offset_of!(EntVars, weaponmodel), EntVarType::Int));
    map.insert("absmin", (offset_of!(EntVars, absmin), EntVarType::Vector3));
    map.insert("absmax", (offset_of!(EntVars, absmax), EntVarType::Vector3));
    map.insert("mins", (offset_of!(EntVars, mins), EntVarType::Vector3));
    map.insert("maxs", (offset_of!(EntVars, maxs), EntVarType::Vector3));
    map.insert("size", (offset_of!(EntVars, size), EntVarType::Vector3));
    map.insert("ltime", (offset_of!(EntVars, ltime), EntVarType::Float));
    map.insert("nextthink", (offset_of!(EntVars, nextthink), EntVarType::Float));
    map.insert("movetype", (offset_of!(EntVars, movetype), EntVarType::Int));
    map.insert("solid", (offset_of!(EntVars, solid), EntVarType::Int));
    map.insert("skin", (offset_of!(EntVars, skin), EntVarType::Int));
    map.insert("body", (offset_of!(EntVars, body), EntVarType::Int));
    map.insert("effects", (offset_of!(EntVars, effects), EntVarType::Int));
    map.insert("gravity", (offset_of!(EntVars, gravity), EntVarType::Float));
    map.insert("friction", (offset_of!(EntVars, friction), EntVarType::Float));
    map.insert("light_level", (offset_of!(EntVars, light_level), EntVarType::Int));
    map.insert("sequence", (offset_of!(EntVars, sequence), EntVarType::Int));
    map.insert("gaitsequence", (offset_of!(EntVars, gaitsequence), EntVarType::Int));
    map.insert("frame", (offset_of!(EntVars, frame), EntVarType::Float));
    map.insert("animtime", (offset_of!(EntVars, animtime), EntVarType::Float));
    map.insert("framerate", (offset_of!(EntVars, framerate), EntVarType::Float));
    map.insert("controller0", (offset_of!(EntVars, controller[0]), EntVarType::Byte));
    map.insert("controller1", (offset_of!(EntVars, controller[1]), EntVarType::Byte));
    map.insert("controller2", (offset_of!(EntVars, controller[2]), EntVarType::Byte));
    map.insert("controller3", (offset_of!(EntVars, controller[3]), EntVarType::Byte));
    map.insert("blending0", (offset_of!(EntVars, blending[0]), EntVarType::Byte));
    map.insert("blending1", (offset_of!(EntVars, blending[1]), EntVarType::Byte));
    map.insert("scale", (offset_of!(EntVars, scale), EntVarType::Float));
    map.insert("rendermode", (offset_of!(EntVars, rendermode), EntVarType::Int));
    map.insert("renderamount", (offset_of!(EntVars, renderamount), EntVarType::Float));
    map.insert("rendercolor", (offset_of!(EntVars, rendercolor), EntVarType::Vector3));
    map.insert("renderfx", (offset_of!(EntVars, renderfx), EntVarType::Int));
    map.insert("health", (offset_of!(EntVars, health), EntVarType::Float));
    map.insert("frags", (offset_of!(EntVars, frags), EntVarType::Float));
    map.insert("weapons", (offset_of!(EntVars, weapons), EntVarType::Int));
    map.insert("takedamage", (offset_of!(EntVars, takedamage), EntVarType::Float));
    map.insert("deadflag", (offset_of!(EntVars, deadflag), EntVarType::Int));
    map.insert("view_ofs", (offset_of!(EntVars, view_ofs), EntVarType::Vector3));
    map.insert("button", (offset_of!(EntVars, button), EntVarType::Int));
    map.insert("impulse", (offset_of!(EntVars, impulse), EntVarType::Int));
    map.insert("chain", (offset_of!(EntVars, chain), EntVarType::EdictPtr));
    map.insert("dmg_inflictor", (offset_of!(EntVars, dmg_inflictor), EntVarType::EdictPtr));
    map.insert("enemy", (offset_of!(EntVars, enemy), EntVarType::EdictPtr));
    map.insert("aiment", (offset_of!(EntVars, aiment), EntVarType::EdictPtr));
    map.insert("owner", (offset_of!(EntVars, owner), EntVarType::EdictPtr));
    map.insert("groundentity", (offset_of!(EntVars, groundentity), EntVarType::EdictPtr));
    map.insert("spawnflags", (offset_of!(EntVars, spawnflags), EntVarType::Int));
    map.insert("flags", (offset_of!(EntVars, flags), EntVarType::Int));
    map.insert("colormap", (offset_of!(EntVars, colormap), EntVarType::Int));
    map.insert("team", (offset_of!(EntVars, team), EntVarType::Int));
    map.insert("max_health", (offset_of!(EntVars, max_health), EntVarType::Float));
    map.insert("teleport_time", (offset_of!(EntVars, teleport_time), EntVarType::Float));
    map.insert("armortype", (offset_of!(EntVars, armortype), EntVarType::Float));
    map.insert("armorvalue", (offset_of!(EntVars, armorvalue), EntVarType::Float));
    map.insert("waterlevel", (offset_of!(EntVars, waterlevel), EntVarType::Int));
    map.insert("watertype", (offset_of!(EntVars, watertype), EntVarType::Int));
    map.insert("target", (offset_of!(EntVars, target), EntVarType::String));
    map.insert("targetname", (offset_of!(EntVars, targetname), EntVarType::String));
    map.insert("netname", (offset_of!(EntVars, netname), EntVarType::String));
    map.insert("message", (offset_of!(EntVars, message), EntVarType::String));
    map.insert("dmg_take", (offset_of!(EntVars, dmg_take), EntVarType::Float));
    map.insert("dmg_save", (offset_of!(EntVars, dmg_save), EntVarType::Float));
    map.insert("dmg", (offset_of!(EntVars, dmg), EntVarType::Float));
    map.insert("dmgtime", (offset_of!(EntVars, dmgtime), EntVarType::Float));
    map.insert("noise", (offset_of!(EntVars, noise), EntVarType::String));
    map.insert("noise1", (offset_of!(EntVars, noise1), EntVarType::String));
    map.insert("noise2", (offset_of!(EntVars, noise2), EntVarType::String));
    map.insert("noise3", (offset_of!(EntVars, noise3), EntVarType::String));
    map.insert("speed", (offset_of!(EntVars, speed), EntVarType::Float));
    map.insert("air_finished", (offset_of!(EntVars, air_finished), EntVarType::Float));
    map.insert("pain_finished", (offset_of!(EntVars, pain_finished), EntVarType::Float));
    map.insert("radsuit_finished", (offset_of!(EntVars, radsuit_finished), EntVarType::Float));
    map.insert("containing_entity", (offset_of!(EntVars, containing_entity), EntVarType::EdictPtr));
    map.insert("playerclass", (offset_of!(EntVars, playerclass), EntVarType::Int));
    map.insert("maxspeed", (offset_of!(EntVars, maxspeed), EntVarType::Float));
    map.insert("fov", (offset_of!(EntVars, fov), EntVarType::Float));
    map.insert("weaponanim", (offset_of!(EntVars, weaponanim), EntVarType::Int));
    map.insert("pushmsec", (offset_of!(EntVars, pushmsec), EntVarType::Int));
    map.insert("bInDuck", (offset_of!(EntVars, in_duck), EntVarType::Bool));
    map.insert("flTimeStepSound", (offset_of!(EntVars, time_step_sound), EntVarType::Int));
    map.insert("flSwimTime", (offset_of!(EntVars, swim_time), EntVarType::Int));
    map.insert("flDuckTime", (offset_of!(EntVars, duck_time), EntVarType::Int));
    map.insert("iStepLeft", (offset_of!(EntVars, step_left), EntVarType::Int));
    map.insert("flFallVelocity", (offset_of!(EntVars, fall_velocity), EntVarType::Float));
    map.insert("gamestate", (offset_of!(EntVars, gamestate), EntVarType::Int));
    map.insert("oldbuttons", (offset_of!(EntVars, oldbuttons), EntVarType::Int));
    map.insert("groupinfo", (offset_of!(EntVars, groupinfo), EntVarType::Int));
    map.insert("iuser1", (offset_of!(EntVars, iuser1), EntVarType::Int));
    map.insert("iuser2", (offset_of!(EntVars, iuser2), EntVarType::Int));
    map.insert("iuser3", (offset_of!(EntVars, iuser3), EntVarType::Int));
    map.insert("iuser4", (offset_of!(EntVars, iuser4), EntVarType::Int));
    map.insert("fuser1", (offset_of!(EntVars, fuser1), EntVarType::Float));
    map.insert("fuser2", (offset_of!(EntVars, fuser2), EntVarType::Float));
    map.insert("fuser3", (offset_of!(EntVars, fuser3), EntVarType::Float));
    map.insert("fuser4", (offset_of!(EntVars, fuser4), EntVarType::Float));
    map.insert("vuser1", (offset_of!(EntVars, vuser1), EntVarType::Vector3));
    map.insert("vuser2", (offset_of!(EntVars, vuser2), EntVarType::Vector3));
    map.insert("vuser3", (offset_of!(EntVars, vuser3), EntVarType::Vector3));
    map.insert("vuser4", (offset_of!(EntVars, vuser4), EntVarType::Vector3));
    map.insert("euser1", (offset_of!(EntVars, euser1), EntVarType::EdictPtr));
    map.insert("euser2", (offset_of!(EntVars, euser2), EntVarType::EdictPtr));
    map.insert("euser3", (offset_of!(EntVars, euser3), EntVarType::EdictPtr));
    map.insert("euser4", (offset_of!(EntVars, euser4), EntVarType::EdictPtr));
    map
  };
}

impl rlua::UserData for EntVarsHandle {
  fn add_methods<'lua, M: rlua::UserDataMethods<'lua, Self>>(m: &mut M) {
    m.add_meta_method(rlua::MetaMethod::Index, |ctx, this: &Self, key: String| {
      use rlua::ToLua;

      match this.entity_handle.get() {
        None => Err(rlua::Error::RuntimeError("Invalid entity".into())),
        Some(edict) => {
          match ENTVARS.get::<&str>(&key.as_str()) {
            Some((offset, ev_type)) => {
              let entvars = entvars_of_edict(edict);
              let ev_offset = entvars as *const EntVars as usize + offset;

              match ev_type {
                EntVarType::Int => unsafe {
                  (*(ev_offset as *const c_int)).to_lua(ctx)
                }
                EntVarType::Float => unsafe {
                  (*(ev_offset as *const c_float)).to_lua(ctx)
                }
                EntVarType::Bool => unsafe {
                  (*(ev_offset as *const c_int) != 0).to_lua(ctx)
                }
                EntVarType::String => unsafe {
                  (*(ev_offset as *const EngineStringHandle)).to_lua(ctx)
                }
                EntVarType::EdictPtr => unsafe {
                  (ev_offset as *const Edict)
                    .as_ref()
                    .map_or_else(EntityHandle::default, EntityHandle::new)
                    .to_lua(ctx)
                }
                _ => unimplemented!("Unsupported type"),
              }
            }
            None => {
              Err(rlua::Error::RuntimeError(
                format!("Invalid entvar \"{}\"", key),
              ))
            }
          }
        }
      }
    });

    m.add_meta_method(rlua::MetaMethod::NewIndex, |ctx, this: &Self, (key, value): (String, rlua::Value)| {
      use rlua::FromLua;

      match this.entity_handle.get() {
        None => Err(rlua::Error::RuntimeError("Invalid entity".into())),
        Some(edict) => {
          match ENTVARS.get::<&str>(&key.as_str()) {
            Some((offset, ev_type)) => {
              let entvars = entvars_of_edict(edict);
              let ev_offset = entvars as *const EntVars as usize + offset;

              match ev_type {
                EntVarType::Int => unsafe {
                  (*(ev_offset as *mut c_int)) = c_int::from_lua(value, ctx)?;
                }
                EntVarType::Float => unsafe {
                  (*(ev_offset as *mut c_float)) = c_float::from_lua(value, ctx)?;
                }
                EntVarType::Bool => unsafe {
                  (*(ev_offset as *mut c_int)) = bool::from_lua(value, ctx)? as c_int;
                }
                EntVarType::String => unsafe {
                  (*(ev_offset as *mut EngineStringHandle)) = EngineStringHandle::from_lua(value, ctx)?;
                }
                EntVarType::EdictPtr => unsafe {
                  *(ev_offset as *mut *mut Edict) = EntityHandle::from_lua(value, ctx)?
                    .get()
                    .map(|r| r as *const Edict as *mut Edict)
                    .ok_or_else(
                      || rlua::Error::RuntimeError("Invalid entity".into())
                    )?;
                }
                _ => unimplemented!("Unsupported type"),
              };

              Ok(())
            }
            None => {
              Err(rlua::Error::RuntimeError(
                format!("Invalid entvar \"{}\"", key),
              ))
            }
          }
        }
      }
    });
  }
}

unsafe impl Send for EntVarsHandle { }

impl<'lua> rlua::ToLua<'lua> for EngineStringHandle {
  fn to_lua(self, ctx: rlua::Context) -> rlua::Result<rlua::Value> {
    ctx
      .create_string(&string_from_handle(self))
      .map(|s| rlua::Value::String(s))
  }
}

impl<'lua> rlua::FromLua<'lua> for EngineStringHandle {
    fn from_lua(value: rlua::Value<'lua>, _: rlua::Context<'lua>) -> rlua::Result<Self> {
      match value {
        rlua::Value::Integer(value) => Ok(EngineStringHandle(value as i32)),
        rlua::Value::String(value) => Ok(handle_from_string(value.to_str()?)),
        _ => Err(rlua::Error::RuntimeError("Expected Integer or String".into())),
      }
    }
}
