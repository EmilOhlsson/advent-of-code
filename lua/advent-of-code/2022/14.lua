local M = {}

local util = require('advent-of-code.utils')
local iter = require('advent-of-code.iterators')
local func = require('advent-of-code.functional')
local ps = require('advent-of-code.points')

local grid_mt = {
    __index = function(table, key)
        local result = rawget(table, key[1] .. ',' .. key[2])
        return result or '.'
    end,
    __newindex = function(table, key, value)
        rawset(table, key[1] .. ',' .. key[2], value)
    end,
}

local function parse(lines)
    local limit = 0
    local grid = {}
    setmetatable(grid, grid_mt)

    for _, line in ipairs(lines) do
        local points = util.split_string(line, ' %-> ')
        for point_pair in iter.window { list = points, size = 2 } do
            local start = func.map_table(tonumber, util.split_string(point_pair[1], ','))
            local stop = func.map_table(tonumber, util.split_string(point_pair[2], ','))
            if start[1] == stop[1] then
                local x = start[1]
                limit = math.max(limit, start[2], stop[2])
                for y in iter.range(start[2], stop[2]) do
                    grid[{ x, y }] = '#'
                end
            else
                local y = start[2]
                limit = math.max(limit, y)
                for x in iter.range(start[1], stop[1]) do
                    grid[{ x, y }] = '#'
                end
            end
        end
    end
    return grid, limit
end

function M.solve_p1(lines)
    local grid, limit = parse(lines)
    local left = ps.create { -1, 0 }
    local down = ps.create { 0, 1 }
    local right = ps.create { 1, 0 }
    local count = 0

    while true do
        local grain = ps.create { 500, 0 }

        while true do
            local grain_next = grain + down
            if grain[2] >= limit then
                return count
            end
            if grid[grain_next] ~= '.' then
                if grid[grain_next + left] == '.' then
                    grain = grain_next + left
                elseif grid[grain_next + right] == '.' then
                    grain = grain_next + right
                else
                    grid[grain] = 'o'
                    count = count + 1
                    break;
                end
            else
                grain = grain_next
            end
        end
    end
end

function M.solve_p2(lines)
    local grid, limit = parse(lines)
    local left = ps.create { -1, 0 }
    local down = ps.create { 0, 1 }
    local right = ps.create { 1, 0 }
    local count = 0
    local stop = ps.create { 500, 0 }

    limit = limit + 2
    local grid_mt = {
        __index = function(table, key)
            if key[2] >= limit then return '#' end
            local result = rawget(table, key[1] .. ',' .. key[2])
            return result or '.'
        end,
        __newindex = function(table, key, value)
            rawset(table, key[1] .. ',' .. key[2], value)
        end,
    }
    setmetatable(grid, grid_mt)

    while true do
        local grain = ps.create { 500, 0 }

        while true do
            local grain_next = grain + down
            if grid[grain_next] ~= '.' then
                if grid[grain_next + left] == '.' then
                    grain = grain_next + left
                elseif grid[grain_next + right] == '.' then
                    grain = grain_next + right
                else
                    grid[grain] = 'o'
                    count = count + 1
                    if grain == stop then
                        return count
                    end
                    break;
                end
            else
                grain = grain_next
            end
        end
    end
end

function M.test() end

M = {
    M.solve_p1,
    M.solve_p2,
}

return M

-- vim: set et ts=4 sw=4 ss=4 tw=100 :
