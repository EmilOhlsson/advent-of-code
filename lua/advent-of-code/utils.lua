local M = {}

-- Map a function, `fn` onto a table, and return the result
function M.map(tbl, fn)
    assert(tbl, "Missing table")
    assert(fn, "Missing function")
    local tbl_new = {}
    for k, v in pairs(tbl) do
        tbl_new[k] = fn(v)
    end
    return tbl_new
end

-- Construct a window iterator using arg.list and arg.size
-- Call using util.window_iterator{size=<size>, list=<list}
function M.window_iterator(arg)
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

-- Create an iterator over items in `list`
function M.list_iterator(list)
    local i = 0;
    local n = #list
    return function ()
        i = i + 1
        if i <= n then
            return list[i]
        end
    end
end

-- Formatted print
function M.printf(fmt, ...)
    print(string.format(fmt, ...))
end

-- fold output of an `iterator` using `fn`, with a given `init`
-- initial value
function M.fold(fn, init, iterator)
    for v in iterator do
        init = fn(init, v)
    end
    return init
end

return M

-- vim: set et ts=4 sw=4 ss=4 tw=100 :
