local M = {}

local util = require('advent-of-code.utils')
local func = require('advent-of-code.functional')
local iter = require('advent-of-code.iterators')

local function parse(lines)
    local lss = iter.values(lines)
    return func.map(util.string_to_array, lss)
end

local function set_from_table(tbl)
    local set = {}
    for _, item in ipairs(tbl) do
        set[item] = true
    end
    return set
end

local function isupper(str)
    return str == string.upper(str)
end

local function char_to_prio(str)
    if isupper(str) then
        return string.byte(str) - string.byte('A') + 1 + 26
    else
        return string.byte(str) - string.byte('a') + 1
    end
end

function M.solve_p1(lines)
    local sum = 0
    for rucksack in parse(lines) do
        local n_items = #rucksack
        assert(n_items % 2 == 0)
        local all = set_from_table(rucksack)
        local first = set_from_table { unpack(rucksack, 1, n_items / 2) }
        local second = set_from_table { unpack(rucksack, n_items / 2 + 1) }

        -- Go through items, and find those present in both
        -- compartments of the rucksack
        for item, _ in pairs(all) do
            if first[item] and second[item] then
                sum = sum + char_to_prio(item)
            end
        end
    end

    return sum
end

function M.solve_p2(lines)
    local sum = 0
    local rucksacks = util.collect(parse(lines))
    local n = 0
    for rucksack_group in iter.chunks { size = 3, list = rucksacks } do
        n = n + 1
        assert(#rucksack_group == 3)
        util.printf('%s', vim.inspect(rucksack_group))
        local sets = {
            set_from_table(rucksack_group[1]),
            set_from_table(rucksack_group[2]),
            set_from_table(rucksack_group[3]),
        }

        local all = {}
        for _, set in ipairs(sets) do
            for item, _ in pairs(set) do
                all[item] = true
            end
        end
        for item, _ in pairs(all) do
            if sets[1][item] and sets[2][item] and sets[3][item] then
                sum = sum + char_to_prio(item)
            end
        end
    end
    util.printf("Number of chunks %d", n)

    return sum
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
