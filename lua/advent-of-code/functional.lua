local M = {}

-- Map a function, `fn` onto a `iterator`, and `yield` result
function M.map(fn, iterator)
    return coroutine.wrap(function()
        for value in iterator do
            coroutine.yield(fn(value))
        end
    end)
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
