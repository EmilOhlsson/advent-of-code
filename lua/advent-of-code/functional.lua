local M = {}

local iter = require('advent-of-code.iterators')
local util = require('advent-of-code.utils')

-- Handy binary operations
M.ops = {
    mul = function(x, y) return x * y end,
    add = function(x, y) return x + y end,
    neq = function(x, y) return x ~= y end,
    eq = function(x, y) return x == y end,
}

M.gen = {
    -- Create a unary function that performs binary op with `v`
    bin = function(op, v)
        return function(x)
            return op(v, x)
        end
    end,
}

function M.sum(iterator)
    if type(iterator) == 'table' then
        local sum = 0
        for _, v in ipairs(iterator) do
            sum = sum + v
        end
        return sum
    else
        local sum = 0
        for v in iterator do
            sum = sum + v
        end
        return sum
    end
end

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
    if type(tbl) == 'function' then
        tbl = util.collect(tbl)
    end
    for i, v in ipairs(tbl) do
        result[i] = fn(v)
    end
    return result
end

-- Recursively map `fn` onto table in place
function M.ri_map(fn, tbl)
    for _, v in ipairs(tbl) do
        if type(v) == 'table' then
            M.ri_map(fn, v)
        else
            fn(v)
        end
    end
end

-- Iterate only over items that are truthy by `pred`
function M.filter(pred, iterator)
    if type(iter) == 'table' then
        iterator = iter.values(iterator)
    end
    return coroutine.wrap(function()
        for value in iterator do
            if pred(value) then
                coroutine.yield(value)
            end
        end
    end)
end

-- fold output of an `iterator` using `fn`, with a given `init`
-- initial value
function M.fold(fn, init, iterator)
    if type(iterator) == 'table' then
        iterator = M.values(iterator)
    end
    for v in iterator do
        init = fn(init, v)
    end
    return init
end

-- Reduce `iterator` using `fn`
function M.reduce(fn, iterator)
    if type(iterator) == 'table' then
        iterator = iter.values(iterator)
    end
    local value = iterator()
    for v in iterator do
        value = fn(value, v)
    end
    return value
end

return M

-- vim: set et ts=4 sw=4 ss=4 tw=100 :
