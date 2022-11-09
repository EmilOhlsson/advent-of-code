local M = {}

local util = require("advent-of-code.utils")
local iterators = require('advent-of-code.iterators')

function M.solve_p1(lines)
end

function M.solve_p2(lines)
end

function M.solve(lines, part)
    local parts = {
        M.solve_p1,
        M.solve_p2,
    }

    return parts[part](lines)
end

function M.test()
    PQueue = require('advent-of-code.priority_queue')
    local series = { 7, 5, 3, 6, 4, 5, 10, 1, 9, 2 }
    local sorted = { unpack(series) }
    table.sort(sorted)
    local my_queue = PQueue.new()

    for value in iterators.list_iterator(series) do
        PQueue.push(my_queue, value)
    end

    assert(PQueue.getn(my_queue) == #sorted)

    for i in iterators.list_iterator(sorted) do
        assert(i == PQueue.pop(my_queue))
    end
    assert(PQueue.getn(my_queue) == 0)

    for _ = 1, 100 do
        local n = math.random(50, 150)
        local random_series = {}
        for _ = 1, n do
            table.insert(random_series, math.random(1000))
        end

        for value in iterators.list_iterator(random_series) do
            PQueue.push(my_queue, value)
        end
        table.sort(random_series)
        for _, expected in ipairs(random_series) do
            local value = PQueue.pop(my_queue)
            assert(expected == value, string.format('%d != %d', expected, value))
        end
    end

    local functional = require('advent-of-code.functional')
    local sum = functional.reduce(function(x, y) return x + y end,
        iterators.list_iterator({ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 }))
    assert(sum == 55)

    local sq_series = { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 }
    local squares = functional.map(function(x) return x * x end, iterators.list_iterator(sq_series))
    for val, square in iterators.zip(iterators.list_iterator(sq_series), squares) do
        assert(val * val, square)
    end

    print("All test show OK")
end

return M

-- vim: set et ts=4 sw=4 ss=4 tw=100 :
