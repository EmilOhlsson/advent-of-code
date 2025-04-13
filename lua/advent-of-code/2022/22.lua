local M = {}

local util = require('advent-of-code.utils')
local iter = require('advent-of-code.iterators')
local func = require('advent-of-code.functional')
local pts = require('advent-of-code.points')

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
    local instructions = {}
    local starting_position
    -- Create, and clear namespace, and then assign colors to highlight groups
    local done_with_map = false
    local max_row, max_col = 0, 0
    for row, line in ipairs(lines) do
        if line == '' then
            done_with_map = true
        elseif done_with_map then
            -- Add spaces to easily be able to split string
            line = line:gsub('R', ' R '):gsub('L', ' L ')
            for _, instr in ipairs(util.split_string(line, ' ')) do
                --print('Instruction: ' .. instr)
                if instr == 'L' or instr == 'R' then
                    table.insert(instructions, instr)
                else
                    table.insert(instructions, tonumber(instr))
                end
            end
        else
            --print('Working with map')
            local chars = util.string_to_array(line)
            for col, char in ipairs(chars) do
                -- Set stating position if not already set
                if starting_position == nil and char == '.' then
                    starting_position = pts.create { row, col }
                end

                if char == '.' or char == '#' then
                    map[pts.create { row, col }] = char
                end
                max_row, max_col = math.max(max_row, row), math.max(max_col, col)
            end
        end
    end
    --print('Instructions: ' .. vim.inspect(instructions))
    return {
        size = { max_row, max_col },
        side = math.max(max_row, max_col)  / 4,
        position = starting_position,
        direction = pts.create { 0, 1 }, -- Facing right
        map = map,
        instructions = instructions,
        turn = function(self, dir)
            --print('Turning ' .. dir .. ' ' .. tostring(self.direction))
            if dir == 'L' then
                self.direction = pts.create { -self.direction[2], self.direction[1] }
            elseif dir == 'R' then
                self.direction = pts.create { self.direction[2], -self.direction[1] }
            else
                assert(false)
            end
            if self.direction[1] == -0 then self.direction[1] = 0 end
            if self.direction[2] == -0 then self.direction[2] = 0 end
            --print(' ' .. tostring(self.direction))
        end,
        step = function(self, steps)
            --print('stepping ' .. steps .. ' from ' .. tostring(self.position))
            for _ = 1, steps do
                local new_point = self.position + self.direction
                local slot = self.map[new_point]
                if slot == '.' then
                    self.position = new_point
                elseif slot == '#' then
                    -- do nothing
                elseif slot == nil then
                    -- Find wrapped position
                    --print(' wrapping from ' .. tostring(self.position))
                    local wrapped_pos = self.position
                    while self.map[wrapped_pos - self.direction] ~= nil do
                        wrapped_pos = wrapped_pos - self.direction
                    end
                    --print(' found ' .. tostring(wrapped_pos))
                    if self.map[wrapped_pos] == '.' then
                        --print('  updating position')
                        self.position = wrapped_pos
                    elseif self.map[wrapped_pos] == '#' then
                        --print('  Wall in wrapped position')
                    end
                else
                    --print(' WAT?! ' .. tostring(slot))
                    assert(false)
                end
            end
            --print(' ' .. tostring(self.position))
        end,
        get_password = function(self)
            local values = {}
            setmetatable(values, mt)
            values[pts.create { 0, 1 }] = 0
            values[pts.create { 1, 0 }] = 1
            values[pts.create { 0, -1 }] = 2
            values[pts.create { -1, 0 }] = 3
            --print(tostring(pts.create { 0, 1 }) ..
            --    ' == ' .. tostring(self.direction) .. ' => ' .. tostring(pts.create { 1, 0 } == self.direction))
            return 1000 * self.position[1] + 4 * self.position[2] + values[self.direction]
        end,
    }
end

function M.solve_p1(lines)
    local map = parse(lines)
    for _, instr in ipairs(map.instructions) do
        if type(instr) == 'string' then
            map:turn(instr)
        elseif type(instr) == 'number' then
            map:step(instr)
        else
            assert(false)
        end
    end
    return map:get_password()
end

function M.solve_p2(lines)
    local map = parse(lines)
    print(vim.inspect(map.size) .. ' ' .. map.side)
    for _, instr in ipairs(map.instructions) do
        if type(instr) == 'string' then
            map:turn(instr)
        elseif type(instr) == 'number' then
            map:step(instr)
        else
            assert(false)
        end
    end
    return map:get_password()
end

function M.test()
end

M[1] = M.solve_p1
M[2] = M.solve_p2

return M

-- vim: set et ts=4 sw=4 ss=4 tw=100 :
