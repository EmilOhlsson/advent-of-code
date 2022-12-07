local M = {}

local util = require('advent-of-code.utils')
local iter = require('advent-of-code.iterators')

local function parse(lines)
    local function parse_impl(line_iter)
        local dir = {}

        while true do
            local line = line_iter()
            if line == nil or line == '$ cd ..' then
                return dir
            end

            -- Try file
            local size, file = line:match('(%d+) ([%a.]+)')
            if size ~= nil and file ~= nil then
                dir[file] = tonumber(size)
            end

            -- Try cd
            local dest = line:match('$ cd (%a+)')
            if dest ~= nil then
                dir[dest] = {
                    files = parse_impl(line_iter)
                }
            end
        end
    end

    local line_iter = iter.values(lines)
    line_iter() -- Discard first line
    return parse_impl(iter.values(lines))
end

-- Calculate directory sizes
local function calculate_size(tree)
    local sum = 0
    for _, node in pairs(tree) do
        if type(node) == 'number' then
            sum = sum + node
        else
            node.size = calculate_size(node.files)
            sum = sum + node.size
        end
    end
    tree.size = sum
    return sum
end

-- Calculate sizes for part 1
local function filtered_size(tree)
    local sum = 0
    for _, node in pairs(tree) do
        if type(node) == 'table' then
            if node.size <= 100000 then
                sum = sum + node.size
            end
            sum = sum + filtered_size(node.files)
        end
    end
    return sum
end

-- Find candidates for part 2
local function find_candidates(tree, total, candidates)
    for _, node in pairs(tree) do
        if type(node) == 'table' then
            if 70000000 - (total - node.size) >= 30000000 then
                table.insert(candidates, node.size)
            end
            find_candidates(node.files, total, candidates)
        end
    end
end

function M.solve_p1(lines)
    local tree = parse(lines)
    calculate_size(tree)
    return filtered_size(tree)
end

function M.solve_p2(lines)
    local tree = parse(lines)
    calculate_size(tree)
    local candidates = {}
    find_candidates(tree, tree.size, candidates)
    table.sort(candidates)
    return candidates[1]
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
