local M = {}

local iter = require('advent-of-code.iterators')

-- Map a function, `fn` onto a `iterator`, and `yield` result
function M.map(fn, iterator)
    if type(iterator) == 'table' then -- Allow table input
        iterator = iter.values(iterator)
    end
    return coroutine.wrap(function()
        for value in iterator do
            coroutine.yield(fn(value))
        end
    end)
end

-- Map a function `fn` onto a table, and return resulting table
function M.map_table(fn, tbl)
    local result = {}
    for i, v in ipairs(tbl) do
        result[i] = fn(v)
    end
    return result
end

-- fold output of an `iterator` using `fn`, with a given `init`
-- initial value
function M.fold(fn, init, iterator)
    for v in iterator do
        init = fn(init, v)
    end
    return init
end

-- Reduce `iterator` using `fn`
function M.reduce(fn, iterator)
    local value = iterator()
    for v in iterator do
        value = fn(value, v)
    end
    return value
end

return M

-- vim: set et ts=4 sw=4 ss=4 tw=100 :
