local M = {}

local util = require('advent-of-code.utils')
local iterators = require('advent-of-code.iterators')

-- You need to either manually keep track of number of keys
-- or count them manually.
local function count_keys(tbl)
    local n = 0
    for _ in pairs(tbl) do
        n = n + 1
    end
    return n
end

function M.solve_p1(lines)
    local chars = util.string_to_array(lines[1])
    local pos = 1
    for window in iterators.window { size = 4, list = chars } do
        local set = {}
        for _, ch in ipairs(window) do
            set[ch] = true
        end
        if count_keys(set) == 4 then
            return pos + 3 -- 3 for rest of header
        end
        pos = pos + 1
    end
end

function M.solve_p2(lines)
    local chars = util.string_to_array(lines[1])
    local pos = 1
    for window in iterators.window { size = 14, list = chars } do
        local set = {}
        for _, ch in ipairs(window) do
            set[ch] = true
        end
        if count_keys(set) == 14 then
            return pos + 13 -- 3 for rest of header
        end
        pos = pos + 1
    end
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
