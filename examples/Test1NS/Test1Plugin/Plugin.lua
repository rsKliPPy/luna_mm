local Core = require 'Luna/Core'
local Listeners = require 'Luna/Listeners'
local P2 = require 'Test1NS/Test2Plugin'
local OF = require './OtherFile'

Core.PrintToConsole('=== Hello from Lua! I am ' .. Plugin:GetIdentifier() .. ' ===')
Core.PrintToConsole(Plugin == OF.PluginHandle and 'Equal' or 'Not Equal')

Listeners.On(Listeners.Events.ClientConnected, function()
  Core.PrintToConsole('=== Client connected! ===');
end)

Listeners.On(Listeners.Events.PluginsLoaded, function()
  Core.PrintToConsole('All plugins loaded')

  -- Cross-plugin communication is only safe after all plugins are loaded
  P2.CrossPluginCommunication('Yay!')
end)

Listeners.On(Listeners.Events.PluginsWillUnload, function()
  Core.PrintToConsole('Plugins will unload')
end)

Listeners.On(Listeners.Events.PluginsUnloading, function()
  Core.PrintToConsole('Plugins unloading')
end)
