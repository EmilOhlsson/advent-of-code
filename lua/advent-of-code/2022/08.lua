local M = {}

local util = require('advent-of-code.utils')
local iter = require('advent-of-code.iterators')
local func = require('advent-of-code.functional')

local function parse(lines)
    local trees = {}
    for i, line in ipairs(lines) do
        local nums = util.string_to_array(line)
        trees[i] = func.map_table(tonumber, nums)
    end
    return trees
end

local function is_visible(trees, row, col)
    local height = trees[row][col]
    local visible = 4 -- Assume visible from 4 directions

    -- check left
    for c = 1, col - 1 do
        if trees[row][c] >= height then
            visible = visible - 1
            break
        end
    end

    -- check right
    for c = col + 1, #trees[row] do
        if trees[row][c] >= height then
            visible = visible - 1
            break
        end
    end

    -- check top
    for r = 1, row - 1 do
        if trees[r][col] >= height then
            visible = visible - 1
            break
        end
    end

    -- check bottom
    for r = row + 1, #trees do
        if trees[r][col] >= height then
            visible = visible - 1
            break
        end
    end

    if visible > 0 then
        return 1
    else
        return 0
    end
end

local function scenic_score(trees, row, col)
    local directions = {
        iter.zip(iter.range(row - 1, 1), iter.repeated(col)), -- look up
        iter.zip(iter.repeated(row), iter.range(col - 1, 1)), -- look left
        iter.zip(iter.repeated(row), iter.range(col + 1, #trees[row])), -- look right
        iter.zip(iter.range(row + 1, #trees), iter.repeated(col)), -- look down
    }

    local score = 1
    local h_stop = trees[row][col]

    for dir in iter.values(directions) do
        -- local h_max = -1
        local count = 0
        for r, c in dir do
            count = count + 1
            if trees[r][c] >= h_stop then
                break
            end
        end
        score = score * count
    end

    return score
end

function M.solve_p1(lines)
    local trees = parse(lines)
    local visible = 0
    for r, treeline in ipairs(trees) do
        for c, _ in ipairs(treeline) do
            visible = visible + is_visible(trees, r, c)
        end
    end
    return visible
end

function M.solve_p2(lines)
    local trees = parse(lines)
    local scores = {}
    local rows, cols = #trees, #trees[1] -- Assume all rows are same length
    for row in iter.range(2, rows - 1) do -- skip outer trees
        for col in iter.range(2, cols - 1) do
            table.insert(scores, scenic_score(trees, row, col))
        end
    end
    table.sort(scores)
    return scores[#scores]
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
