local M = {}

local util = require('advent-of-code.utils')
local iter = require('advent-of-code.iterators')
local func = require('advent-of-code.functional')

local function parse(lines)
    return func.map(function(line)
        local dir, steps = line:match('(%a) (%d+)')
        return dir, tonumber(steps)
    end, lines)
end

local function clamp(val, lo, hi)
    if val < lo then return lo end
    if val > hi then return hi end
    return val
end

local function create_point()
    return {
        row = 1,
        col = 1,

        is_adjacent = function(self, other)
            return math.abs(self.row - other.row) <= 1 and
                math.abs(self.col - other.col) <= 1
        end,

        move = function(self, dir)
            if dir == 'U' then
                self.row = self.row - 1
            elseif dir == 'D' then
                self.row = self.row + 1
            elseif dir == 'L' then
                self.col = self.col - 1
            elseif dir == 'R' then
                self.col = self.col + 1
            else
                assert(false, "No such movementt")
            end
        end,

        move_towards = function(self, other)
            if not self:is_adjacent(other) then
                if self.col ~= other.col then
                    self.col = self.col + clamp(other.col - self.col, -1, 1)
                end
                if self.row ~= other.row then
                    self.row = self.row + clamp(other.row - self.row, -1, 1)
                end
            end
        end,

        str = function(self)
            return string.format('%d,%d', self.row, self.col)
        end,
    }
end

local function create_map()
    return {
        map = {},

        visit = function(self, point)
            local key = string.format('%d,%d', point.row, point.col)
            if self.map[key] == nil then
                self.map[key] = 0
            end
            self.map[key] = self.map[key] + 1
        end,

        count_visited = function(self)
            local count = 0
            for _, _ in pairs(self.map) do
                count = count + 1
            end
            return count
        end,
    }
end

function M.solve_p1(lines)
    local tail = create_point()
    local head = create_point()
    local map = create_map()

    map:visit(tail)

    for dir, steps in parse(lines) do
        for _ = 1, steps do
            head:move(dir)
            tail:move_towards(head)
            map:visit(tail)
            assert(tail:is_adjacent(head), "broken logic")
        end
    end
    return map:count_visited()
end

function M.solve_p2(lines)
    local map = create_map()
    local knots = {}
    for i = 1, 10 do
        knots[i] = create_point()
    end

    map:visit(knots[#knots])

    for dir, steps in parse(lines) do
        for _ = 1, steps do
            knots[1]:move(dir)
            for i = 2, #knots do
                knots[i]:move_towards(knots[i - 1])
                assert(knots[i]:is_adjacent(knots[i - 1]), "broken logic")
            end
            map:visit(knots[#knots])
        end
    end
    return map:count_visited()
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
