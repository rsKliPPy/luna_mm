local Core = require 'Luna/Core'
local Listeners = require 'Luna/Listeners'
local P2 = require 'Test1NS/Test2Plugin'
local OF = require './OtherFile'

Core.PrintToConsole('--- Hello from Lua! I am ' .. Plugin:GetIdentifier())

Listeners.On(Listeners.Events.PluginsLoaded, function()
  Core.PrintToConsole('All plugins loaded')
  --OF.ErroringFunc()

  -- Cross-plugin communication is only safe after all plugins are loaded
  P2.CrossPluginCommunication('Yay!')
end)

Listeners.On(Listeners.Events.ClientConnect, function()
  Core.PrintToConsole('--- ClientConnect ---')
end)

Listeners.On(Listeners.Events.ClientPutInServer, function(entity)
  local entvars = entity:EntVars()
  Core.PrintToConsole('--- PutInServer ' .. entvars['netname'])
  entvars['netname'] = 'Klipicica'
  Core.PrintToConsole('--- PutInServer ' .. entvars['netname'])
  Core.PrintToConsole('--- Classname ' .. entvars['classname'])
end)
