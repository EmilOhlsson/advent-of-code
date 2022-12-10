local M = {}

local util = require('advent-of-code.utils')
local iter = require('advent-of-code.iterators')
local func = require('advent-of-code.functional')

function M.solve_p1(lines)
    local signal = 1
    local signal_sum = 0

    local cycle = 1
    for _, instruction in ipairs(lines) do
        local cycles
        local increment
        if instruction == 'noop' then
            cycles = 1
            increment = 0
        else
            local val = tonumber(instruction:match('addx (-?%d+)'))
            cycles = 2
            increment = val
        end

        for pc = cycle, cycle + cycles - 1 do
            if pc >= 20 and (pc - 20) % 40 == 0 then
                signal_sum = signal_sum + signal * pc
            end
        end

        cycle = cycle + cycles
        signal = signal + increment
    end

    return signal_sum
end

function M.solve_p2(lines)
    local signal = 1
    local display = ""

    local cycle = 1
    for _, instruction in ipairs(lines) do
        local cycles
        local increment
        if instruction == 'noop' then
            cycles = 1
            increment = 0
        else
            local val = tonumber(instruction:match('addx (-?%d+)'))
            cycles = 2
            increment = val
        end

        for pc = cycle, cycle + cycles - 1 do
            local sprite_pos = pc % 40
            if sprite_pos >= signal and sprite_pos <= signal + 2 then
                display = display .. '#'
            else
                display = display .. '.'
            end
            if pc >= 40 and pc % 40 == 0 then
                display = display .. '\n'
            end
        end

        cycle = cycle + cycles
        signal = signal + increment
    end
    util.printf('Result:\n%s', display)
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
