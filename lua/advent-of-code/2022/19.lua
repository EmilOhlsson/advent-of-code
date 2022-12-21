local M = {}

local util = require('advent-of-code.utils')
local iter = require('advent-of-code.iterators')
local func = require('advent-of-code.functional')
local ps = require('advent-of-code.points')

local blob_ops = {}

local function blob(arg)
    local b = {
        ore = arg.ore or 0,
        clay = arg.clay or 0,
        obsidian = arg.obsidian or 0,
        geode = arg.geode or 0,
    }
    setmetatable(b, blob_ops)
    return b
end

function blob_ops.__sub(self, other)
    local b = {}
    for k, v in pairs(self) do
        b[k] = v - other[k]
    end
    return blob(b)
end

function blob_ops.__add(self, other)
    local b = {}
    for k, v in pairs(self) do
        b[k] = v + other[k]
    end
    return blob(b)
end

function blob_ops.__le(self, other)
    for k, v in pairs(self) do
        if v > other[k] then
            return false
        end
    end
    return true
end

function blob_ops.__tostring(self)
    return 'ore: ' .. self.ore ..
        ', clay: ' .. self.clay ..
        ', obsidian: ' .. self.obsidian ..
        ', geode: ' .. self.geode
end

local function parse(lines)
    local blueprints = {}
    for bp, line in ipairs(lines) do
        local ore, clay, ob_ore, ob_clay, geod_ore, geod_ob = line:match('^Blueprint %d+: Each ore robot costs (%d+) ore. Each clay robot costs (%d+) ore. Each obsidian robot costs (%d+) ore and (%d+) clay. Each geode robot costs (%d+) ore and (%d+) obsidian.$')
        assert(ore ~= nil and clay ~= nil and ob_ore ~= nil and ob_clay ~= nil and geod_ore ~= nil and geod_ob ~= nil,
            string.format('line %s does not match', line))
        --
        blueprints[bp] = {
            ore = blob { ore = tonumber(ore) },
            clay = blob { ore = tonumber(clay) },
            obsidian = blob { ore = tonumber(ob_ore), clay = tonumber(ob_clay) },
            geode = blob { ore = tonumber(geod_ore), obsidian = tonumber(geod_ob) },
        }
    end
    return blueprints
end

-- Find number of geodes you can build within remaining time
local count = 0
local ore = blob { ore = 1 }
local clay = blob { clay = 1 }
local obsidian = blob { obsidian = 1 }
local geode = blob { geode = 1 }
local function find(prod, resources, bp, time, skip, best)
    count = count + 1

    if time <= 0 then
        return resources.geode
    end
    if resources.geode < (best[time] or 0) then
        -- This would neve amount to anything
        return 0
    end
    best[time] = resources.geode

    -- It doesn't cost geodes to build, so if we can build, then build something
    if bp.geode <= resources then
        return find(prod + geode, resources - bp.geode + prod, bp, time - 1, {}, best)
    else
        local score = 0
        local new_skip = {}
        if not skip.ore and bp.ore <= resources then
            new_skip.ore = true
            score = math.max(score, find(prod + ore, resources - bp.ore + prod, bp, time - 1, {}, best))
        end
        if not skip.clay and bp.clay <= resources then
            new_skip.clay = true
            score = math.max(score, find(prod + clay, resources - bp.clay + prod, bp, time - 1, {}, best))
        end
        if not skip.obsidian and bp.obsidian <= resources then
            new_skip.obsidian = true
            score = math.max(score, find(prod + obsidian, resources - bp.obsidian + prod, bp, time - 1, {}, best))
        end
        score = math.max(score, find(prod, resources + prod, bp, time - 1, new_skip, best))
        return score
    end
end

function M.solve_p1(lines)
    local blueprints = parse(lines)
    local robots = blob { ore = 1 }
    local resources = blob {}
    local sum = 0
    for i, blueprint in ipairs(blueprints) do
        local score = find(robots, resources, blueprint, 24, {}, {})
        sum = sum + score * i
    end
    return sum
end

function M.solve_p2(lines)
    local blueprints = parse(lines)
    local robots = blob { ore = 1 }
    local resources = blob {}
    local prod = 1
    for i = 1, 3 do
        local score = find(robots, resources, blueprints[i], 32, {}, {})
        prod = prod * score
    end
    return prod
end

function M.test()

end

M[1] = M.solve_p1
M[2] = M.solve_p2

return M

-- vim: set et ts=4 sw=4 ss=4 tw=100 :
