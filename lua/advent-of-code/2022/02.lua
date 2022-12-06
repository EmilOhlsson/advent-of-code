local M = {}

local util = require('advent-of-code.utils')

local function parse(lines)
    local tbl = {}
    for _, line in ipairs(lines) do
        local entry = {
            opponent = line:sub(1, 1),
            me = line:sub(3, 3),
        }
        table.insert(tbl, entry)
    end
    return tbl
end

function M.solve_p1(lines)
    local scores = {
        X = 1, -- Rock
        Y = 2, -- Paper
        Z = 3, -- Scissor
    }
    local win_over = {
        X = 'C', -- Rock beats scissor,
        Y = 'A', -- Paper beats scissor,
        Z = 'B', -- Scissor beats paper
    }
    local lose_over = {
        X = 'B', -- Rocks is beaten by paper
        Y = 'C', -- Paper is beaten by scissor
        Z = 'A', -- Scissor is beaten by rock
    }
    local strategy = parse(lines)
    local score = 0
    for _, move in ipairs(strategy) do
        if move.opponent == win_over[move.me] then
            score = score + 6 + scores[move.me]
        elseif move.opponent == lose_over[move.me] then
            score = score + scores[move.me]
        else -- draw
            score = score + 3 + scores[move.me]
        end
    end
    return score
end

function M.solve_p2(lines)
    local scores = {
        A = 1, -- Rock
        B = 2, -- Paper
        C = 3, -- Scissor
    }
    local lose_over = {
        A = 'C', -- Rock beats scissor,
        B = 'A', -- Paper beats scissor,
        C = 'B', -- Scissor beats paper
    }
    local win_over = {
        A = 'B', -- Rocks is beaten by paper
        B = 'C', -- Paper is beaten by scissor
        C = 'A', -- Scissor is beaten by rock
    }
    local strategy = parse(lines)
    local score = 0
    for _, move in ipairs(strategy) do
        if move.me == 'X' then -- Lose
            score = score  + scores[lose_over[move.opponent]]
        elseif move.me == 'Y' then -- Draw
            score = score + 3 + scores[move.opponent]
        else -- 'Z' -- Win
            score = score + 6 + scores[win_over[move.opponent]]
        end
    end
    return score
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
