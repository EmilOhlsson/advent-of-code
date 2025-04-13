local M = {}



function M.diff(a,b)
end

local function is_disjoint(set1, set2)
    assert(type(set1) == 'table' and type(set2) == 'table')
    for k, _ in pairs(set1) do
        if set2[k] then
            return false
        end
    end
    for k, _ in pairs(set2) do
        if set1[k] then
            return false
        end
    end
    return true
end

local function get_set_size(set)
    local count = 0
    for _, _ in pairs(set) do
        count = count + 1
    end
    return count
end

local function set_to_string(set)
    local list = {}
    for x, _ in pairs(set) do
        table.insert(list, x)
    end
    table.sort(list)
    local result = ""
    if #list > 0 then
        result = result .. list[1]
        for i = 2, #list do
            result = result .. ',' .. list[i]
        end
    end
    return result
end

local function string_to_set(str)
    local result = {}
    for _, e in ipairs(util.split_string(str, ',')) do
        result[e] = true
    end
    return result
end

return M

-- vim: set et ts=4 sw=4 ss=4 tw=100 :
