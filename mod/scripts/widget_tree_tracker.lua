local url = "http://127.0.0.1:45754/sync"

local WidgetNode = {
    id = nil, -- string
    parent_id = nil, -- string
}

function WidgetNode.Init(id)
    return {
        id = id
    }
end

local WidgetTreeTracker = {
    next_id = 1,
    widgets_ids = setmetatable({}, { __mode = "k" }),
    widget_nodes = {},
}

function WidgetTreeTracker:RegisterWidget(widget, parent)
    local id = self.widgets_ids[widget]
    if id == nil then
        id = tostring(self.next_id)
        self.widgets_ids[widget] = id
        self.widget_nodes[id] = WidgetNode.Init(id)

        self.next_id = self.next_id + 1
    end

    if parent ~= nil then
        self.widget_nodes[id].parent_id = self.widgets_ids[parent]
    end
end

function WidgetTreeTracker:RemoveChild(child)
    local child_id = self.widgets_ids[child]
    if child_id and self.widget_nodes[child_id] then
        self.widget_nodes[child_id].parent_id = nil
    end
end

function WidgetTreeTracker:Remove(widget)
    local id = self.widgets_ids[widget]
    if id ~= nil then
        self.widgets_ids[widget] = nil
        self.widget_nodes[id] = nil
    end
end

function WidgetTreeTracker:OnResponse(body, successful, status_code)
    print(body, successful, status_code)
end

function WidgetTreeTracker:Start()
    local payload = {
        widget_nodes = self.widget_nodes,
    }

    TheSim:QueryServer(url, function(...)
        self:OnResponse(...)
    end, "POST", json.encode_compliant(payload))
end

return WidgetTreeTracker
