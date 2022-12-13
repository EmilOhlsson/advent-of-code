local M = {}

local util = require('advent-of-code.utils')
local iter = require('advent-of-code.iterators')
local func = require('advent-of-code.functional')
local ps = require('advent-of-code.points')

-- Parse, by converting arrays to Lua code
local function parse_line(line)
    line = line:gsub('%[', '{')
    line = line:gsub(']', '}')
    return loadstring('return ' .. line)()
end

local function parse(lines)
    return func.map_table(function(pair)
        return func.map_table(parse_line, pair)
    end, iter.split(lines, ''))
end

local function parse_v2(lines)
    -- Filter away empty lines, and parse lines as json
    local signal_iter = func.filter(func.gen.bin(func.ops.neq, ''), lines)
    return func.map_table(vim.json.decode, signal_iter)
end

local mt = {
    __tostring = vim.json.encode
}

function mt.__eq(l, r)
    if type(l) == 'number' and type(r) == 'number' then
        return l == r
    elseif type(l) == 'number' then
        return mt.__eq({ l }, r)
    elseif type(r) == 'number' then
        return mt.__eq(l, { r })
    else
        local n = math.max(#l, #r)
        for i = 1, n do
            if l[i] == nil or r[i] == nil then
                return false
            elseif not mt.__eq(l[i], r[i]) then
                return false
            end
        end
        return true
    end
end

function mt.__lt(l, r)
    -- Need to explicitly use `__lt` to make sure correct method is actually used
    if type(l) == 'number' and type(r) == 'number' then
        return l < r
    elseif type(l) == 'number' then
        return mt.__lt({ l }, r)
    elseif type(r) == 'number' then
        return mt.__lt(l, { r })
    else
        local n = math.max(#l, #r)
        for i = 1, n do
            if l[i] == nil then
                return true
            elseif r[i] == nil then
                return false
            elseif not mt.__eq(l[i], r[i]) then
                return mt.__lt(l[i], r[i])
            end
        end
        return false
    end
end

function M.solve_p1(lines)
    local sum = 0
    for i, pair in ipairs(parse(lines)) do
        setmetatable(pair[1], mt)
        setmetatable(pair[2], mt)
        if pair[1] < pair[2] then
            sum = sum + i
        end
    end
    return sum
end

function M.solve_p2(lines)
    local signals = parse_v2(lines)
    local decoder_keys = {
        { { 2 } },
        { { 6 } },
    }
    table.insert(signals, decoder_keys[1])
    table.insert(signals, decoder_keys[2])
    for i = 1, #signals do
        setmetatable(signals[i], mt) -- Make sure signals are comparable with `<`
    end
    table.sort(signals)
    local prod = 1
    for i, v in ipairs(signals) do
        print(v)
        if v == decoder_keys[1] or v == decoder_keys[2] then
            prod = prod * i
        end
    end
    return prod
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
