local Core = require 'Luna/Core'
local P2 = require 'Test1NS/Test2Plugin'

Core.PrintToConsole('=== Hello from Lua! I am ' .. Plugin:GetIdentifier() .. ' ===')
P2.CrossPluginCommunication('Yay!')
