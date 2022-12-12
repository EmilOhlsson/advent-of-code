local M = {}

local util = require('advent-of-code.utils')
local iter = require('advent-of-code.iterators')
local func = require('advent-of-code.functional')
local ps = require('advent-of-code.points')

-- Metatable that allows hashing based on string representation
local mt = {
    __index = function(table, key)
        return rawget(table, tostring(key))
    end,
    __newindex = function(table, key, value)
        rawset(table, tostring(key), value)
    end,
}

local function parse(lines)
    local map = {}
    setmetatable(map, mt)
    local start, stop

    local ns = vim.api.nvim_create_namespace('aoc')
    vim.api.nvim_buf_clear_namespace(0, ns, 0, -1)
    for h = 97, 106 do
        vim.cmd(string.format('highlight AoC%d guifg=#00%02x00', h, 50 + (h - 97) * 20))
    end
    for h = 107, 122 do
        vim.cmd(string.format('highlight AoC%d guifg=#7f%02x00', h, 50 + (h - 107) * 10))
    end

    for row, line in ipairs(lines) do
        local chars = util.string_to_array(line)
        for col, char in ipairs(chars) do
            local height = char:byte()
            if char == 'S' then
                height = string.byte('a')
                start = ps.create { row, col }
            elseif char == 'E' then
                height = string.byte('z')
                stop = ps.create { row, col }
            end
            assert(height ~= nil)
            map[ps.create { row, col }] = height

            -- Funny highlighting
            vim.api.nvim_buf_add_highlight(0, ns, 'AoC' .. height, row - 1, col - 1, col)
            --vim.api.nvim_
        end
    end
    return start, stop, map
end

local MOVES = {
    ps.create { -1, 0 }, -- up
    ps.create { 0, -1 }, -- left
    ps.create { 0, 1 }, -- right
    ps.create { 1, 0 }, -- down
}
local function get_next(map, position)
    local result = {}
    local height = map[position]
    for _, move in ipairs(MOVES) do
        local position_new = position + move
        local height_new = map[position_new]
        if height_new ~= nil and height_new <= height + 1 then
            table.insert(result, position_new)
        end
    end
    return result
end

function M.solve_p1(lines)
    local start, stop, map = parse(lines)
    local distance_map = {} -- Closest distance to given point
    distance_map[start] = 0
    setmetatable(distance_map, mt)

    local queue = { start }
    while #queue > 0 do
        local position = table.remove(queue, 1)
        local next = get_next(map, position)
        local dist = distance_map[position]
        for _, pos_next in ipairs(next) do
            if pos_next == stop then
                return dist + 1
            end
            local dist_next = distance_map[pos_next]
            if dist_next == nil then
                distance_map[pos_next] = dist + 1
                table.insert(queue, pos_next)
            end
        end
    end
    assert(false, "Did not find path")
end

local function get_next_v2(map, position)
    local result = {}
    local height = map[position]
    for _, move in ipairs(MOVES) do
        local position_new = position + move
        local height_new = map[position_new]
        if height_new ~= nil and height_new >= height - 1 then
            table.insert(result, position_new)
        end
    end
    return result
end

function M.solve_p2(lines)
    local _, start, map = parse(lines)
    local distance_map = {} -- Closest distance to given point
    distance_map[start] = 0
    setmetatable(distance_map, mt)

    local queue = { start }
    while #queue > 0 do
        local position = table.remove(queue, 1)
        local next = get_next_v2(map, position)
        local dist = distance_map[position]
        for _, pos_next in ipairs(next) do
            if map[pos_next] == string.byte('a') then
                return dist + 1
            end
            local dist_next = distance_map[pos_next]
            if dist_next == nil then
                distance_map[pos_next] = dist + 1
                table.insert(queue, pos_next)
            end
        end
    end
    assert(false, "Did not find path")
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
