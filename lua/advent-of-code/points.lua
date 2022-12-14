local M = {
    mt = {} -- Shared metatable for points
}

-- Allow point addition
function M.mt.__add(a, b)
    local result = {}
    assert(#a == #b)
    for i = 1, #a do
        result[i] = a[i] + b[i]
    end
    return M.create(result)
end

-- Allow point comparison
function M.mt.__eq(a, b)
    if #a ~= #b then
        return false
    end
    for i = 1, #a do
        if a[i] ~= b[i] then
            return false
        end
    end
    return true
end

-- Allow point printing
function M.mt.__tostring(point)
    local result = '(' .. point[1]
    for i = 2, #point do
        result = result .. ', ' .. point[i]
    end
    result = result .. ')'
    return result
end

-- Create point from table
function M.create(point)
    assert(type(point) == 'table')
    assert(#point >= 1)
    setmetatable(point, M.mt)
    return point
end

return M

-- vim: set et ts=4 sw=4 ss=4 tw=100 :