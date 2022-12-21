local M = {}

local util = require('advent-of-code.utils')
local iter = require('advent-of-code.iterators')
local func = require('advent-of-code.functional')
local pts = require('advent-of-code.points')

function M.solve_p1(lines)
    -- The list of numbers
    local numbers = func.map_table(tonumber, lines)

    -- Used to map position to index in nums
    local indices = util.collect(iter.range(1, #numbers))

    -- Inverse of above, maps index in num to specific position
    local positions = util.collect(iter.range(1, #numbers))

    local zero_idx = func.find_pos(function(x) return x == 0 end, numbers)

    -- Swap numbers at two positions
    local function swap(a, b)
        indices[a], indices[b] = indices[b], indices[a]
        positions[indices[a]] = a
        positions[indices[b]] = b
    end

    local N = #numbers
    local function index(i)
        return (i - 1) % N + 1
    end

    for i = 1, N do
        -- Subtract and add 1 to keep 1-indexed
        local num = numbers[i]
        local pos = positions[i]
        local pos_new = (pos + num - 1) % (N - 1) + 1
        for ps in iter.window { size = 2, list = iter.range(pos, pos_new) } do
            swap(ps[1], ps[2])
        end
    end

    local z_pos = positions[zero_idx]
    local a = numbers[indices[index(z_pos + 1000)]]
    local b = numbers[indices[index(z_pos + 2000)]]
    local c = numbers[indices[index(z_pos + 3000)]]
    return a + b + c
end

function M.solve_p2(lines)
    -- The list of numbers
    local numbers = func.map_table(function(v)
        return tonumber(v) * 811589153
    end, lines)
    local N = #numbers

    -- Used to map position to index in nums
    local indices = util.collect(iter.range(1, N))

    -- Inverse of above, maps index in num to specific position
    local positions = util.collect(iter.range(1, N))

    -- Swap numbers at two positions
    local function swap(a, b)
        indices[a], indices[b] = indices[b], indices[a]
        positions[indices[a]] = a
        positions[indices[b]] = b
    end

    for _ = 1, 10 do
        for i = 1, N do
            -- Subtract and add 1 to keep 1-indexed
            local num = numbers[i]
            local pos = positions[i]
            local pos_new = (pos + num - 1) % (N - 1) + 1
            for ps in iter.window { size = 2, list = iter.range(pos, pos_new) } do
                swap(ps[1], ps[2])
            end
        end
    end

    local function index(i)
        return (i - 1) % N + 1
    end

    -- Find zero value, and calculate
    local zero_idx = func.find_pos(function(x) return x == 0 end, numbers)
    local z_pos = positions[zero_idx]
    local a = numbers[indices[index(z_pos + 1000)]]
    local b = numbers[indices[index(z_pos + 2000)]]
    local c = numbers[indices[index(z_pos + 3000)]]
    return a + b + c
end

function M.test()

end

M[1] = M.solve_p1
M[2] = M.solve_p2

return M

-- vim: set et ts=4 sw=4 ss=4 tw=100 :
