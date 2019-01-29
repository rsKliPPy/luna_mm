local Core = require 'Luna/Core'
local Listeners = require 'Luna/Listeners'
local P2 = require 'Test1NS/Test2Plugin'
local OF = require './OtherFile'

Core.PrintToConsole('=== Hello from Lua! I am ' .. Plugin:GetIdentifier() .. ' ===')
P2.CrossPluginCommunication('Yay!')
Core.PrintToConsole(Plugin == OF.PluginHandle and "Equal" or "Not Equal")

Listeners.On(Listeners.Events.ClientConnected, function(player)
  Core.PrintToConsole("=== Client connected! ===");
end)
