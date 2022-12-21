local M = {}

-- Formatted print
function M.printf(fmt, ...)
    print(string.format(fmt, ...))
end

-- Check if output from `iterator` is sorted
function M.is_sorted(iterator)
    local prev
    for value in iterator do
        if prev == nil then
            prev = value
        elseif prev > value then
            return false
        end
    end
    return true
end

-- Collect output from an `iterator` into a table
function M.collect(iterator)
    local result = {}
    for value in iterator do
        table.insert(result, value)
    end
    return result
end

-- Joind a list of elements using concat
function M.join(tbl, sep)
    if #tbl > 0 then
        local res = tostring(tbl[1])
        for i = 2, #tbl do
            res = res .. sep .. tbl[i]
        end
        return res
    else
        return ""
    end
end

-- Create an array of character strings based on a string
function M.string_to_array(str)
    local tbl = {}
    str:gsub('.', function(char)
        table.insert(tbl, char)
    end)
    return tbl
end

-- Character iterator
function M.chars(str)
    local i = 0
    local n = #str
    return function()
        i = i + 1
        if i <= n then
            return str:sub(i, i)
        end
    end
end

-- Split a string, on a set of characters
function M.split_string(str, split)
    assert(str ~= nil and split ~= nil)
    return M.collect(str:gmatch('([^' .. split .. ']+)'))
end

-- Trim white spaces from before and aftter string
function M.trim_string(str)
    return str:match('^%s*(.-)%s*$')
end

function M.copy(tbl)
    if type(tbl) == 'table' then
        local tbl_new = {}
        for k, v in pairs(tbl) do
            tbl_new[k] = M.copy(v)
        end
        setmetatable(tbl_new, getmetatable(tbl))
        return tbl_new
    else
        return tbl
    end
end

return M

-- vim: set et ts=4 sw=4 ss=4 tw=100 :
