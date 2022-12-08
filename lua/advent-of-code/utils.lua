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

-- Create an array of character strings based on a string
function M.string_to_array(str)
    local tbl = {}
    str:gsub('.', function(char)
        table.insert(tbl, char)
    end)
    return tbl
end

return M

-- vim: set et ts=4 sw=4 ss=4 tw=100 :
