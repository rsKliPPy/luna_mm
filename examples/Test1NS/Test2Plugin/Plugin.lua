local Core = require 'Luna/Core'

local function PrintStuff(message)
  Core.PrintToConsole('PrintStuff from ' .. Plugin:GetIdentifier() .. '. Message: ' .. message)
end

return {
  CrossPluginCommunication = PrintStuff,
}
