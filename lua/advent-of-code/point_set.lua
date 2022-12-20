local M = {}

function M.new()
    return {}
end

function M.set(points, p)
    if points[p[1]] == nil then
        points[p[1]] = {}
    end
    if points[p[1]][p[2]] == nil then
        points[p[1]][p[2]] = {}
    end
    points[p[1]][p[2]][p[3]] = true
end

function M.get(points, p)
    local x = points[p[1]] or {}
    local y = x[p[2]] or {}
    return y[p[3]] or false
end

return M

-- vim: set et ts=4 sw=4 ss=4 tw=100 :
