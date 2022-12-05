local M = {}

local stack = require('advent-of-code.stack')

local function parse(lines)
    local parsing_header = true
    local stacks = {}
    stacks.instructions = {}
    for _, line in ipairs(lines) do
        if parsing_header then
            -- Loop over characters
            for i = 1, line:len() do
                local ch = line:sub(i, i)
                -- Check if character is alphabetic, that is box name
                if ch:match('%a') then
                    local s = math.ceil(i / 4)
                    if stacks[s] == nil then
                        stacks[s] = {}
                    end
                    table.insert(stacks[s], 1, ch)
                end
            end
        else
            -- Parse instructions and store as tables
            local amount, from, to = line:match("move (%d+) from (%d+) to (%d+)")
            table.insert(stacks.instructions, {
                amount = tonumber(amount),
                from = tonumber(from),
                to = tonumber(to)
            })
        end

        -- Toggle header parsing
        if line == "" then
            parsing_header = false
        end
    end
    return stacks
end

function M.solve_p1(lines)
    local cargo = parse(lines)
    for _, inst in ipairs(cargo.instructions) do
        for _ = 1, inst.amount do
            stack.push(cargo[inst.to], stack.pop(cargo[inst.from]))
        end
    end
    local top = ''
    for _, pile in ipairs(cargo) do
        top = top .. stack.peek(pile)
    end
    return top
end

function M.solve_p2(lines)
    local cargo = parse(lines)
    for _, inst in ipairs(cargo.instructions) do
        local N = #cargo[inst.from] - inst.amount + 1
        for _ = 1, inst.amount do
            -- Bit lazy, but it works. Likley not efficient
            table.insert(cargo[inst.to], table.remove(cargo[inst.from], N))
        end
    end
    local top = ''
    for _, pile in ipairs(cargo) do
        top = top .. stack.peek(pile)
    end
    return top
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
