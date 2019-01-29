pub fn map_funcs<'lua>(
  old_table: &rlua::Table<'lua>,
  new_table: &rlua::Table<'lua>,
  old_keys: &[&str],
  new_keys: &[&str],
) {
  old_keys.iter().zip(new_keys.iter()).for_each(|(old, new)| {
    new_table.raw_set(
      *new,
      old_table.raw_get::<_, rlua::Value>(*old).unwrap()
    ).unwrap();
  });
}
