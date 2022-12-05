local M = {}

local stack = require('advent-of-code.stack')

local function parse(lines)
    local sum = 0
    local amounts = {}
    local push_sum = function()
        table.insert(amounts, sum)
        sum = 0;
    end
    for _, line in ipairs(lines) do
        if line == '' then
            push_sum()
        else
            sum = sum + tonumber(line)
        end
    end
    push_sum()
    return amounts
end

function M.solve_p1(lines)
    local calories = parse(lines)
    table.sort(calories)
    return calories[#calories]
end

function M.solve_p2(lines)
    local calories = parse(lines)
    table.sort(calories)
    return calories[#calories] + calories[#calories - 1] + calories[#calories - 2]
end

function M.solve(lines, part)
    local parts = {
        M.solve_p1,
        M.solve_p2,
    }

    return parts[part](lines)
end

function M.test()
end

return M

-- vim: set et ts=4 sw=4 ss=4 tw=100 :
