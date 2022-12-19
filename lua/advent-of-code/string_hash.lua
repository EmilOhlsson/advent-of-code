local M = {}

function M.new()
    local table = {}
    setmetatable(table, {
        __index = function(tbl, k)
            return rawget(tbl, tostring(k))
        end,
        __newindex = function(tbl, k, v)
            rawset(tbl, tostring(k), v)
        end,
    })
    return table
end

return M

-- vim: set et ts=4 sw=4 ss=4 tw=100 :
