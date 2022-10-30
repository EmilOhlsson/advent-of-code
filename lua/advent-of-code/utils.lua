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
function M.window_iterator(arg)
    local i = arg.size
    local n = #arg.list + 1
    assert(i >= 1, "Zero sized window")
    assert(i <= n, "Window larger than table")
    return function () 
        i = i + 1
        if i <= n then
            return {unpack(arg.list, i - arg.size, i - 1)}
        end
    end
end

-- Formatted print
function M.printf(fmt, ...)
    print(string.format(fmt, ...))
end

-- TODO: Create fold method
--function M.fold(lst, op)
--    for k, v 
--end

return M

-- vim: set et ts=4 sw=4 ss=4 tw=100 :
