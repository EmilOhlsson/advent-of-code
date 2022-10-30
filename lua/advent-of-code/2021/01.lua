local M = {}

local util = require("advent-of-code.utils")

function M.solve_p1(depths)
    local count = 0
    local prev

    for _, depth in ipairs(depths) do
        if prev and depth > prev then
            count = count + 1
        end
        prev = depth
    end
    return count
end

function M.solve_p2(depths)
    local count = 0
    local prev
    for win in util.window_iterator{list=depths, size=3} do
        sum = win[1] + win[2] + win[3]
        if prev and sum > prev then
            count = count + 1
        end
        prev = sum
    end
    return count
end

function M.solve(lines, part)
    local parts = {
        M.solve_p1,
        M.solve_p2,
    }

    local depths =  util.map(lines, tonumber)
    return parts[part](depths)
end

return M

-- vim: set et ts=4 sw=4 ss=4 tw=100 :
