local M = {}

local util = require('advent-of-code.utils')
local iter = require('advent-of-code.iterators')
local func = require('advent-of-code.functional')

local function parse(lines)
    return func.map(function(line)
        local a_lo, a_hi, b_lo, b_hi = line:match('(%d+)-(%d+),(%d+)-(%d+)')
        return tonumber(a_lo), tonumber(a_hi), tonumber(b_lo), tonumber(b_hi)
    end, lines)
end

function M.solve_p1(lines)
    local count = 0;
    for a_lo, a_hi, b_lo, b_hi in parse(lines) do
        if (a_lo <= b_lo and a_hi >= b_hi) or
            (b_lo <= a_lo and b_hi >= a_hi) then
            count = count + 1
        end
    end
    return count
end

function M.solve_p2(lines)
    local count = 0;
    for a_lo, a_hi, b_lo, b_hi in parse(lines) do
        if (a_lo <= b_hi and a_lo >= b_lo) or
            (a_hi <= b_hi and a_hi >= b_lo) or
            (a_lo <= b_lo and a_hi >= b_hi) or
            (b_lo <= a_lo and b_hi >= a_hi) then
            count = count + 1
        end
    end
    -- 776 is too low
    return count
end

function M.solve(lines, part)
    local parts = {
        M.solve_p1,
        M.solve_p2,
    }

    return parts[part](lines)
end

function M.test()
    for v in iter.range(1, 1) do
        util.printf('range: %d', v)
    end

    for r, c in iter.zip_n(iter.range(1, 3), iter.range(3, 1)) do
        util.printf('zip: %d %d', r, c)
    end
end

return M

-- vim: set et ts=4 sw=4 ss=4 tw=100 :
