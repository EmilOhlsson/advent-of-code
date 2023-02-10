local M = {}

local util = require('advent-of-code.utils')
local iter = require('advent-of-code.iterators')
local func = require('advent-of-code.functional')
local pts = require('advent-of-code.points')

local function parse_v1(lines)
    local monkeys = {}
    for i = 1, #lines do
        local splits = util.split_string(lines[i], ': ')
        --print('splits: ' .. vim.inspect(splits))
        if #splits == 2 then -- just a number
            monkeys[splits[1]] = tonumber(splits[2])
        else
            monkeys[splits[1]] = {
                l = splits[2],
                op = splits[3],
                r = splits[4],
            }
        end
    end
    return {
        monkeys = monkeys,
        cache = {},
        resolve = function(self, expr)
            if type(expr) == 'number' then
                return expr
            else
                if expr.op == '+' then
                    return self:evaluate(expr.l) + self:evaluate(expr.r)
                elseif expr.op == '-' then
                    return self:evaluate(expr.l) - self:evaluate(expr.r)
                elseif expr.op == '*' then
                    return self:evaluate(expr.l) * self:evaluate(expr.r)
                elseif expr.op == '/' then
                    return self:evaluate(expr.l) / self:evaluate(expr.r)
                end
            end
        end,
        evaluate = function(self, node)
            if self.cache[node] ~= nil then
                return self.cache[node]
            else
                local result = self:resolve(self.monkeys[node])
                self.cache[node] = result
                return result
            end
        end,
    }
end

function M.solve_p1(lines)
    local monkeys = parse_v1(lines)
    return monkeys:evaluate('root')
end

local function parse_v2(lines)
    local monkeys = {}
    for i = 1, #lines do
        local splits = util.split_string(lines[i], ': ')
        --print('splits: ' .. vim.inspect(splits))
        if #splits == 2 then -- just a number
            monkeys[splits[1]] = tonumber(splits[2])
        else
            monkeys[splits[1]] = {
                l = splits[2],
                op = splits[3],
                r = splits[4],
            }
        end
    end
    return {
        monkeys = monkeys,
        cache = {},

        resolve = function(self, expr)
            if type(expr) == 'number' then
                return expr
            else
                local lhs = self:evaluate(expr.l)
                local rhs = self:evaluate(expr.r)
                -- If either term is based on 'humn' value
                -- then this value is unknown for now
                if lhs == nil or rhs == nil then
                    return nil
                end
                if expr.op == '+' then
                    return lhs + rhs
                elseif expr.op == '-' then
                    return lhs - rhs
                elseif expr.op == '*' then
                    return lhs * rhs
                elseif expr.op == '/' then
                    return lhs / rhs
                end
            end
        end,

        evaluate = function(self, node)
            -- Consider 'humn' an unknown value, making sure
            -- values based on 'humn' are not being added to
            -- the cache
            if node == 'humn' then
                return nil
            end
            if self.cache[node] ~= nil then
                return self.cache[node]
            else
                local result = self:resolve(self.monkeys[node])
                self.cache[node] = result
                return result
            end
        end,

        solve = function(self, node, expected_value)
            -- We have found the expected value of humn
            if node == 'humn' then
                return expected_value
            end

            local expr = self.monkeys[node]
            local lhs = self.cache[expr.l]
            local rhs = self.cache[expr.r]

            -- Solve for unknown
            if lhs == nil then
                if expr.op == '+' then
                    return self:solve(expr.l, expected_value - rhs)
                elseif expr.op == '-' then
                    return self:solve(expr.l, expected_value + rhs)
                elseif expr.op == '/' then
                    return self:solve(expr.l, expected_value * rhs)
                elseif expr.op == '*' then
                    return self:solve(expr.l, expected_value / rhs)
                end
            elseif rhs == nil then
                if expr.op == '+' then
                    return self:solve(expr.r, expected_value - lhs)
                elseif expr.op == '-' then
                    return self:solve(expr.r, lhs - expected_value)
                elseif expr.op == '/' then
                    return self:solve(expr.r, lhs / expected_value)
                elseif expr.op == '*' then
                    return self:solve(expr.r, expected_value / lhs)
                end
            else
                assert(false)
            end
        end,
    }
end

function M.solve_p2(lines)
    local monkeys = parse_v2(lines)
    local left = monkeys.monkeys['root'].l
    local right = monkeys.monkeys['root'].r

    local lhs = monkeys:evaluate(left)
    local rhs = monkeys:evaluate(right)
    if lhs == nil then
        return monkeys:solve(left, rhs)
    else
        return monkeys:solve(left, lhs)
    end
end

function M.test()
end

M[1] = M.solve_p1
M[2] = M.solve_p2

return M

-- vim: set et ts=4 sw=4 ss=4 tw=100 :
