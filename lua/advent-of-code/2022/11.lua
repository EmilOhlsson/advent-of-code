local M = {}

local util = require('advent-of-code.utils')
local iter = require('advent-of-code.iterators')
local func = require('advent-of-code.functional')

local function parse(lines)
    return func.map_table(function(group)
        local items = group[2]:sub(#'  Starting items: ' + 1)
        local operation = group[3]:sub(#'  Operation: new = old ' + 1)
        local operand = operation:sub(3)
        if operation:sub(1, 1) == '*' then
            if operand == 'old' then
                operation = function(v) return v * v end
            else
                operand = tonumber(operand)
                operation = function(v) return v * operand end
            end
        elseif operation:sub(1, 1) == '+' then
            if operand == 'old' then
                operation = function(v) return v + v end
            else
                operand = tonumber(operand)
                operation = function(v) return v + operand end
            end
        else
            assert(false)
        end
        local div = tonumber(group[4]:sub(#'  Test: divisible by ' + 1))
        -- Because Lua is 1 indexed we need to add 1 to destination
        local true_dest = tonumber(group[5]:sub(#'    If true: throw to monkey ' + 1)) + 1
        local false_dest = tonumber(group[6]:sub(#'    If false: throw to monkey ' + 1)) + 1
        return {
            count = 0,
            items = util.collect(func.map(tonumber, util.split_string(items, ','))),
            div = div,
            destination = function(v)
                if v % div == 0 then
                    return true_dest
                else
                    return false_dest
                end
            end,
            operation = operation,
        }
    end, iter.split(lines, ''))
end

function M.solve_p1(lines)
    local monkeys = parse(lines)
    for _ = 1, 20 do
        for _, monkey in ipairs(monkeys) do
            for _, item in ipairs(monkey.items) do
                monkey.count = monkey.count + 1
                local worry_level = monkey.operation(item)
                worry_level = math.floor(worry_level / 3)
                local dest = monkey.destination(worry_level)
                table.insert(monkeys[dest].items, worry_level)
            end
            monkey.items = {} -- No monkey throws to itself
        end
    end
    local counts = func.map_table(function(m)
        return m.count
    end, monkeys)
    table.sort(counts)
    return counts[#counts] * counts[#counts - 1]
end

function M.solve_p2(lines)
    local monkeys = parse(lines)
    local get_attribute = function(attr)
        local attribs = func.map_table(function(m)
            return m[attr]
        end, monkeys)
        return attribs
    end
    local div = func.reduce(func.ops.mul, get_attribute('div'))
    for _ = 1, 10000 do
        for _, monkey in ipairs(monkeys) do
            for _, item in ipairs(monkey.items) do
                monkey.count = monkey.count + 1
                local worry_level = monkey.operation(item) % div
                local dest = monkey.destination(worry_level)
                table.insert(monkeys[dest].items, worry_level)
            end
            monkey.items = {} -- No monkey throws to itself
        end
    end
    local counts = get_attribute('count')
    table.sort(counts)
    return counts[#counts] * counts[#counts - 1]
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
