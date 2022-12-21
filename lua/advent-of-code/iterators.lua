local util = require('advent-of-code.utils')

local M = {}

-- Create an iterator over values in `list`
function M.values(list)
    local i = 0;
    local n = #list
    return function()
        i = i + 1
        if i <= n then
            return list[i]
        end
    end
end

-- Iterate from start to stop, inclusive
function M.range(start, stop)
    if start <= stop then
        return function()
            if start <= stop then
                local val = start
                --util.printf(' returning %u', val)
                start = start + 1
                return val
            end
        end
    else
        return function()
            if start >= stop then
                local val = start
                start = start - 1
                return val
            end
        end
    end
end

-- Construct a window iterator using arg.list and arg.size
-- Call using iterators.window{size=<size>, list=<list}
function M.window(arg)
    if type(arg.list) == 'function' then
        arg.list = util.collect(arg.list)
    end
    local i = arg.size
    local n = #arg.list + 1
    assert(i >= 1, "Zero sized window")
    assert(i <= n, "Window larger than table")
    return function()
        i = i + 1
        if i <= n then
            -- Create a new list from the slice using `unpack`
            return { unpack(arg.list, i - arg.size, i - 1) }
        end
    end
end

-- Return chunks of a table using arg.list and arg.size
-- call using iterators.chunks{size=<size>, list=<list>}
function M.chunks(arg)
    local i = 0
    local n = #arg.list
    return function()
        i = i + arg.size
        if i <= n then
            return { unpack(arg.list, i - arg.size + 1, i) }
        end
    end
end

-- Zip two iterators into one returning tuples
function M.zip(itr1, itr2)
    return function()
        local a, b = itr1(), itr2()
        if a ~= nil and b ~= nil then
            return a, b
        end
    end
end

-- Zip multiple iterators together
function M.zip_n(...)
    local args = { ... }
    return function()
        local result = {}
        for i, v in ipairs(args) do
            result[i] = v()
            if result[i] == nil then
                return nil
            end
        end
        return unpack(result)
    end
end

-- Repeate value
function M.repeated(value)
    return function()
        return value
    end
end

-- Return tables from an iterator by split
function M.split(iter, split)
    assert(iter ~= nil and split ~= nil)
    -- Allow taking arrays as parameter
    if type(iter) == 'table' then
        iter = M.values(iter)
    end
    local done = false
    return function()
        local result = {}
        for entry in iter do
            if entry == split then
                return result
            else
                table.insert(result, entry)
            end
        end
        if not done then
            done = true
            return result
        end
    end
end

return M

-- vim: set et ts=4 sw=4 ss=4 tw=100 :
