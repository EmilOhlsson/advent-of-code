local M = {}

local util = require('advent-of-code.utils')
local iterators = require('advent-of-code.iterators')

-- instruction iterator, each call produces structs
-- with `instruction` and `distance` fields
local function create_instruction_iterator(lines)
    -- Co-routine just to try it out
    return coroutine.wrap(function()
        for line in iterators.values(lines) do
            local instr, dist = string.match(line, "(%l+) (%d+)")
            coroutine.yield { instruction = instr, distance = tonumber(dist) }
        end
    end)
end

function M.solve_p1(instruction_iterator)
    local state = {
        depth = 0,
        dist = 0,
        forward = function(self, amount) self.dist = self.dist + amount end,
        up = function(self, amount) self.depth = self.depth - amount end,
        down = function(self, amount) self.depth = self.depth + amount end,
    }
    for cmd in instruction_iterator do
        state[cmd.instruction](state, cmd.distance)
    end
    return state.depth * state.dist
end

function M.solve_p2(instruction_iterator)
    local state = {
        aim = 0,
        depth = 0,
        dist = 0,
        forward = function(self, amount)
            self.dist = self.dist + amount
            self.depth = self.depth + self.aim * amount
        end,
        up = function(self, amount) self.aim = self.aim - amount end,
        down = function(self, amount) self.aim = self.aim + amount end,
    }
    for cmd in instruction_iterator do
        state[cmd.instruction](state, cmd.distance)
    end
    return state.depth * state.dist
end

function M.solve(lines, part)
    local parts = {
        M.solve_p1,
        M.solve_p2,
    }

    local instructions = create_instruction_iterator(lines)
    return parts[part](instructions)
end

function M.test()
    local list = { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 }
    assert(util.fold(function(x, y) return x + y end,
        0, util.list_iterator(list)) == 55,
        "fold doesn't work")
    print("All test show OK")
end

return M

-- vim: set et ts=4 sw=4 ss=4 tw=100 :
