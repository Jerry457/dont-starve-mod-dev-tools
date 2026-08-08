local AddSimPostInit = AddSimPostInit
local AddGamePostInit = AddGamePostInit
local AddClassPostConstruct = AddClassPostConstruct
GLOBAL.setfenv(1, GLOBAL)

local Widget = require("widgets/widget")

WidgetTreeTracker = require("widget_tree_tracker")
local WidgetTreeTracker = WidgetTreeTracker

AddClassPostConstruct("widgets/widget", function(widget)
    WidgetTreeTracker:RegisterWidget(widget)
end)

local _AddChild = Widget.AddChild
function Widget:AddChild(child, ...)
    WidgetTreeTracker:RegisterWidget(self)
    WidgetTreeTracker:RegisterWidget(child, self)
    return _AddChild(self, child, ...)
end

local _RemoveChild = Widget.RemoveChild
function Widget:RemoveChild(child, ...)
    WidgetTreeTracker:RemoveChild(child)
    return _RemoveChild(self, child, ...)
end

local _Kill = Widget.Kill
function Widget:Kill(...)
    return _Kill(self, ...)
end

local function StartTracker()
    WidgetTreeTracker:Start()
end
AddGamePostInit(StartTracker)
AddSimPostInit(StartTracker)
