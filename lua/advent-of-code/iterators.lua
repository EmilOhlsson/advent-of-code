local M = {}

-- Create an iterator over values in `list`
function M.values(list)
    local i = 0;
    local n = #list
    return function ()
        i = i + 1
        if i <= n then
            return list[i]
        end
    end
end

-- Construct a window iterator using arg.list and arg.size
-- Call using iterators.window{size=<size>, list=<list}
function M.window(arg)
    local i = arg.size
    local n = #arg.list + 1
    assert(i >= 1, "Zero sized window")
    assert(i <= n, "Window larger than table")
    return function ()
        i = i + 1
        if i <= n then
            -- Create a new list from the slice using `unpack`
            return {unpack(arg.list, i - arg.size, i - 1)}
        end
    end
end

-- Return chunks of a table using arg.list and arg.size
-- call using iterators.chunks{size=<size>, list=<list>}
function M.chunks(arg)
    local i = 0
    local n = #arg.list
    return function ()
        i = i + arg.size
        if i <= n then
            return {unpack(arg.list, i - arg.size + 1, i)}
        end
    end
end


-- Zip two iterators into one returning tuples
function M.zip(itr1, itr2)
    return coroutine.wrap(function()
        coroutine.yield(itr1(), itr2())
    end)
end

return M

-- vim: set et ts=4 sw=4 ss=4 tw=100 :
