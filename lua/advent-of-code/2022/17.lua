local M = {}

local util = require('advent-of-code.utils')
local iter = require('advent-of-code.iterators')
local func = require('advent-of-code.functional')
local ps = require('advent-of-code.points')
local hash = require('advent-of-code.string_hash')

-- origin of piece is it's bottom left most point
local pieces = {
    {
        -- horiz line piece
        ps.create { 0, 0 },
        ps.create { 1, 0 },
        ps.create { 2, 0 },
        ps.create { 3, 0 },
    }, {
        ps.create { 0, 1 },
        ps.create { 1, 0 },
        ps.create { 1, 1 },
        ps.create { 1, 2 },
        ps.create { 2, 1 },
    }, {
        ps.create { 0, 0 },
        ps.create { 1, 0 },
        ps.create { 2, 0 },
        ps.create { 2, 1 },
        ps.create { 2, 2 },
    }, {
        ps.create { 0, 0 },
        ps.create { 0, 1 },
        ps.create { 0, 2 },
        ps.create { 0, 3 },
    }, {
        ps.create { 0, 0 },
        ps.create { 0, 1 },
        ps.create { 1, 0 },
        ps.create { 1, 1 },
    }
}

local down = ps.create { 0, -1 }
local left = ps.create { -1, 0 }
local right = ps.create { 1, 0 }

local tower = {
    height = 0,
    blocks = hash.new(),
    offset = ps.create { 3, 4 },
    block = 1
}

function tower:try_collision(offset, place)
    offset = self.offset + offset
    local collision = false
    for _, p in ipairs(pieces[self.block]) do
        local pn = p + offset
        collision = collision or self.blocks[pn]
        collision = collision or pn[1] > 7
        collision = collision or pn[1] <= 0
        collision = collision or pn[2] <= 0
    end
    if collision and place then
        for _, p in ipairs(pieces[self.block]) do
            local pn = p + self.offset
            self.blocks[pn] = true
            self.height = math.max(self.height, pn[2])
        end
    elseif not collision then
        self.offset = offset
    end
    return collision
end

function tower:fall()
    return tower:try_collision(down, true)
end

function tower:left()
    return tower:try_collision(left, false)
end

function tower:right()
    return tower:try_collision(right, false)
end

function tower:next_piece()
    self.offset = ps.create { 3, 4 + self.height }
    self.block = self.block + 1
    if self.block > #pieces then
        self.block = 1
    end
end

function tower:line_string(y)
    local str = '|'
    for x = 1, 7 do
        if self.blocks[ps.create { x, y }] then
            str = str .. '#'
        else
            str = str .. ' '
        end
    end
    str = str .. '|'
    return str
end

function tower:top_30_liens()
    local str = ''
    for y in iter.range(self.height, math.max(self.height - 30, 1)) do
        str = str .. tower:line_string(y)
    end
    return str
end

function M.solve_p1(lines)
    local count = 0
    while true do
        for ch in util.chars(lines[1]) do
            if ch == '<' then
                tower:left()
            elseif ch == '>' then
                tower:right()
            end
            if tower:fall() then
                tower:next_piece()
                count = count + 1
                if count >= 2022 then
                    return tower.height
                end
            end
        end
    end
end

function M.solve_p2(lines)
    local count = 0
    local seen = {}
    while true do
        for ch in util.chars(lines[1]) do
            if ch == '<' then
                tower:left()
            elseif ch == '>' then
                tower:right()
            end
            if tower:fall() then
                tower:next_piece()
                count = count + 1
                local fingerprint = tower:top_30_liens()
                local prev = seen[fingerprint]
                if prev ~= nil then -- Found cycle
                    local height = tower.height - prev.height
                    local steps = count - prev.count
                    local cycles = math.floor((1000000000000 - count) / steps)
                    local lft = 1000000000000 - (cycles * steps + count)
                    for _, params in pairs(seen) do
                        if prev.count + lft == params.count then
                            return tower.height + height * cycles + (params.height - prev.height)
                        end
                    end
                end
                seen[fingerprint] = { count = count, height = tower.height }
            end
        end
    end
end

function M.test()

end

M[1] = M.solve_p1
M[2] = M.solve_p2

return M

-- vim: set et ts=4 sw=4 ss=4 tw=100 :
