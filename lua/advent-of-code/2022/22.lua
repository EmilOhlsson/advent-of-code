local M = {}

local utils = require('advent-of-code.utils')
local iter = require('advent-of-code.iterators')
local func = require('advent-of-code.functional')
local pts = require('advent-of-code.points')
local gcd = require('advent-of-code.math').gcd
local logger = require('advent-of-code.logger')

---@class CubeMap
---@field private rows number
---@field private cols number
---@field private position Point
---@field private direction Point
---@field private map Map
---@field private stitch Map
---@field instructions table
---@field private folding Map|nil
---@field private logger Logger
---@field fold_cube fun(self)
---@field turn fun(self, dir: string)
---@field step fun(self, steps: number)
---@field get_password fun(self): number

---@return CubeMap
local function parse(lines, log)
    local map = utils.create_map()
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
            for _, instr in ipairs(utils.split_string(line, ' ')) do
                --print('Instruction: ' .. instr)
                if instr == 'L' or instr == 'R' then
                    table.insert(instructions, instr)
                else
                    table.insert(instructions, tonumber(instr))
                end
            end
        else
            --print('Working with map')
            local chars = utils.string_to_array(line)
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
        rows=max_row,
        cols=max_col,
        position = starting_position,
        direction = pts.create { 0, 1 }, -- Facing right
        map = map,
        stitch = utils.create_map(),
        instructions = instructions,
        folding = nil,
        logger = log,
        log = function(self, msg, ...)
            if self.logger ~= nil then
                self.logger:log(msg, ...)
            end
        end,

        fold_cube = function(self)
            self.fold = true
            local side = gcd(self.rows, self.cols)
            local dirs = {
                pts.create{-1,0},  -- up
                pts.create{0,1},   -- right
                pts.create{1,0},   -- down
                pts.create{0,-1},  -- left
            }

            local rot_l = function(pt) return pts.create{-pt[2], pt[1]} end
            local rot_r = function(pt) return pts.create{pt[2], -pt[1]} end
            local rev   = function(pt) return pts.create{-pt[1], -pt[2]} end

            -- Track which (pos, outward_dir) are already paired
            local known = utils.create_map()
            local queue = {}

            local function add_pair(p1, d1, p2, d2)
                local key1 = pts.create{p1, d1}
                if known[key1] then return end
                known[key1] = true
                known[pts.create{p2, d2}] = true

                -- Only create stitch entries for non-adjacent pairs
                if not self.map[p1 + d1] then
                    self.stitch[pts.create{p1 + d1, d1}] = pts.create{p2, rev(d2)}
                    self.stitch[pts.create{p2 + d2, d2}] = pts.create{p1, rev(d1)}
                end

                table.insert(queue, {p1, d1, p2, d2})
            end

            -- Seed: all directly adjacent cells on different faces
            for row = 1, self.rows do
                for col = 1, self.cols do
                    local p = pts.create{row, col}
                    if self.map[p] then
                        for _, d in ipairs(dirs) do
                            local nb = p + d
                            if self.map[nb] then
                                -- Different face? (different side×side block)
                                local f1r = math.ceil(row / side)
                                local f1c = math.ceil(col / side)
                                local f2r = math.ceil(nb[1] / side)
                                local f2c = math.ceil(nb[2] / side)
                                if f1r ~= f2r or f1c ~= f2c then
                                    add_pair(p, d, nb, rev(d))
                                end
                            end
                        end
                    end
                end
            end

            -- BFS: propagate around cube vertices
            local qi = 1
            while qi <= #queue do
                local p1, d1, p2, d2 = table.unpack(queue[qi])
                qi = qi + 1

                -- Try both perpendicular directions along the edge
                for _, rot in ipairs({rot_r, rot_l}) do
                    local anti = (rot == rot_r) and rot_l or rot_r
                    local step1 = rot(d1)
                    local step2 = anti(d2)  -- opposite rotation for the other face

                    local n1, nd1 = p1 + step1, d1
                    local n2, nd2 = p2 + step2, d2
                    local turn1 = not self.map[n1]
                    local turn2 = not self.map[n2]

                    if turn1 and turn2 then
                        -- Both hit face corners at same vertex; skip
                    else
                        if turn1 then n1, nd1 = p1, step1 end  -- corner turn
                        if turn2 then n2, nd2 = p2, step2 end
                        add_pair(n1, nd1, n2, nd2)
                    end
                end
            end
        end,

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
            self:log('stepping ' .. steps .. ' from ' .. tostring(self.position))
            for _ = 1, steps do
                local new_point = self.position + self.direction
                local slot = self.map[new_point]
                if slot == '.' then
                    self.position = new_point
                elseif slot == '#' then
                    -- do nothing
                elseif slot == nil then
                    if self.fold then
                        local result = self.stitch[pts.create{new_point, self.direction}]
                        assert(result)
                        local new_pos, new_dir = table.unpack(result)
                        if self.map[new_pos] == '.' then
                            self.position = new_pos
                            self.direction = new_dir
                        end
                    else
                        -- Find wrapped position
                        --self:log(' wrapping from ' .. tostring(self.position))
                        local wrapped_pos = self.position
                        -- look back until finding other edge
                        while self.map[wrapped_pos - self.direction] ~= nil do
                            wrapped_pos = wrapped_pos - self.direction
                        end
                        --self:log(' found ' .. tostring(wrapped_pos))
                        if self.map[wrapped_pos] == '.' then
                            --self:log('  updating position')
                            self.position = wrapped_pos
                        elseif self.map[wrapped_pos] == '#' then
                            --self:log('  Wall in wrapped position')
                        end
                    end
                else
                    --self:log(' WAT?! ' .. tostring(slot))
                    assert(false)
                end
            end
            --print(' ' .. tostring(self.position))
        end,

        get_password = function(self)
            local values = utils.create_map()
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

---@return number
function M.solve_p2(lines)
    local log = logger.create_logger(true, "aoc-22-2.log", 'w')
    local map = parse(lines, log)
    map:fold_cube()
    -- log:log("portals: " .. vim.inspect(map.stitch))
    for _, instr in ipairs(map.instructions) do
        if type(instr) == 'string' then
            map:turn(instr)
        elseif type(instr) == 'number' then
            map:step(instr)
        else
            assert(false)
        end
    end
    local password = map:get_password()
    log:log("result = %d", password)
    return password
end

function M.test()
end

M[1] = M.solve_p1
M[2] = M.solve_p2

return M

-- vim: set et ts=4 sw=4 ss=4 tw=100 :
