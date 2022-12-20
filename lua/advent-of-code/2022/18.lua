local M = {}

local util = require('advent-of-code.utils')
local iter = require('advent-of-code.iterators')
local func = require('advent-of-code.functional')
local ps = require('advent-of-code.points')
local hash = require('advent-of-code.string_hash')
local pset = require('advent-of-code.point_set')

function M.solve_p1(lines)
    local points = pset.new()
    for _, line in ipairs(lines) do
        local toks = util.split_string(line, ',')
        pset.set(points, { tonumber(toks[1]), tonumber(toks[2]), tonumber(toks[3]) })
    end

    print('points', vim.inspect(points))

    local surfaces = 0
    for x, ys in pairs(points) do
        for y, zs in pairs(ys) do
            for z, _ in pairs(zs) do
                if not pset.get(points, { x - 1, y, z }) then surfaces = surfaces + 1 end
                if not pset.get(points, { x + 1, y, z }) then surfaces = surfaces + 1 end
                if not pset.get(points, { x, y - 1, z }) then surfaces = surfaces + 1 end
                if not pset.get(points, { x, y + 1, z }) then surfaces = surfaces + 1 end
                if not pset.get(points, { x, y, z - 1 }) then surfaces = surfaces + 1 end
                if not pset.get(points, { x, y, z + 1 }) then surfaces = surfaces + 1 end
            end
        end
    end

    return surfaces
end

function M.solve_p2(lines)
    local obsidian = pset.new()
    local air = pset.new()
    local x_lim = {}
    local y_lim = {}
    local z_lim = {}

    -- Parse
    for _, line in ipairs(lines) do
        local toks = util.split_string(line, ',')
        local x, y, z = tonumber(toks[1]), tonumber(toks[2]), tonumber(toks[3])
        if x_lim.lo == nil then -- first
            x_lim.lo, x_lim.hi = x, x
            y_lim.lo, y_lim.hi = y, y
            z_lim.lo, z_lim.hi = z, z
        end
        x_lim.lo, x_lim.hi = math.min(x_lim.lo, x), math.max(x_lim.hi, x)
        y_lim.lo, y_lim.hi = math.min(y_lim.lo, y), math.max(y_lim.hi, y)
        z_lim.lo, z_lim.hi = math.min(z_lim.lo, z), math.max(z_lim.hi, z)
        pset.set(obsidian, { x, y, z })
    end

    -- Fill outside
    for x = x_lim.lo - 1, x_lim.hi + 1 do
        for y = y_lim.lo - 1, y_lim.hi + 1 do
            pset.set(air, { x, y, z_lim.lo - 1 })
            pset.set(air, { x, y, z_lim.hi + 1 })
        end
        for z = z_lim.lo - 1, z_lim.hi + 1 do
            pset.set(air, { x, y_lim.lo - 1, z })
            pset.set(air, { x, y_lim.hi + 1, z })
        end
    end
    for z = z_lim.lo - 1, z_lim.hi + 1 do
        for y = y_lim.lo - 1, y_lim.hi + 1 do
            pset.set(air, { x_lim.lo - 1, y, z })
            pset.set(air, { x_lim.hi + 1, y, z })
        end
    end

    -- Local functions, because I'm lazy
    local function has_air_neighbor(x, y, z)
        return pset.get(air, { x - 1, y, z }) or
            pset.get(air, { x + 1, y, z }) or
            pset.get(air, { x, y - 1, z }) or
            pset.get(air, { x, y + 1, z }) or
            pset.get(air, { x, y, z - 1 }) or
            pset.get(air, { x, y, z + 1 })
    end

    local queue = {}
    local function enqueue(x, y, z)
        if not pset.get(air, { x, y, z }) and not pset.get(obsidian, { x, y, z }) then
            table.insert(queue, { x, y, z })
        end
    end

    local function set_air(x, y, z)
        if not pset.get(obsidian, { x, y, z }) then
            if not pset.get(air, { x, y, z }) then
                if has_air_neighbor(x, y, z) then
                    -- Then mark this as air, and enqueu neighbors
                    pset.set(air, { x, y, z })
                    enqueue(x + 1, y, z)
                    enqueue(x - 1, y, z)
                    enqueue(x, y + 1, z)
                    enqueue(x, y - 1, z)
                    enqueue(x, y, z + 1)
                    enqueue(x, y, z - 1)
                end
            end
        end
    end

    -- enqueue first layer
    for x = x_lim.lo, x_lim.hi do
        for y = y_lim.lo, y_lim.hi do
            set_air(x, y, z_lim.lo)
            set_air(x, y, z_lim.hi)
        end
        for z = z_lim.lo, z_lim.hi do
            set_air(x, y_lim.lo, z)
            set_air(x, y_lim.hi, z)
        end
    end
    for z = z_lim.lo, z_lim.hi do
        for y = y_lim.lo, y_lim.hi do
            set_air(x_lim.lo, y, z)
            set_air(x_lim.hi, y, z)
        end
    end

    -- Grow air
    while #queue > 0 do
        local p = table.remove(queue)
        set_air(p[1], p[2], p[3])
    end

    local surfaces = 0
    for x, ys in pairs(obsidian) do
        for y, zs in pairs(ys) do
            for z, _ in pairs(zs) do
                if pset.get(air, { x - 1, y, z }) then surfaces = surfaces + 1 end
                if pset.get(air, { x + 1, y, z }) then surfaces = surfaces + 1 end
                if pset.get(air, { x, y - 1, z }) then surfaces = surfaces + 1 end
                if pset.get(air, { x, y + 1, z }) then surfaces = surfaces + 1 end
                if pset.get(air, { x, y, z - 1 }) then surfaces = surfaces + 1 end
                if pset.get(air, { x, y, z + 1 }) then surfaces = surfaces + 1 end
            end
        end
    end

    return surfaces
end

function M.test()

end

M[1] = M.solve_p1
M[2] = M.solve_p2

return M

-- vim: set et ts=4 sw=4 ss=4 tw=100 :
