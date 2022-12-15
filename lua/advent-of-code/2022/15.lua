local M = {}

local util = require('advent-of-code.utils')
local iter = require('advent-of-code.iterators')
local func = require('advent-of-code.functional')
local ps = require('advent-of-code.points')

local function parse(lines)
    local sensors = {}
    local beacons = {}
    local distances = {}
    for i, line in ipairs(lines) do
        local a, b, c, d = line:match('^Sensor at x=(-?%d+), y=(-?%d+): closest beacon is at x=(-?%d+), y=(-?%d+)$')
        assert(a ~= nil and b ~= nil and c ~= nil and d ~= nil)
        sensors[i] = ps.create { tonumber(a), tonumber(b) }
        beacons[i] = ps.create { tonumber(c), tonumber(d) }
        distances[i] = sensors[i]:mdistance(beacons[i])
    end
    return sensors, beacons, distances
end

function M.solve_p1(lines)
    local sensors, beacons, distances = parse(lines)

    local x_lo = sensors[1][1]
    local x_hi = sensors[1][1]
    local n_sensors = #sensors
    for sensor, distance in iter.zip(iter.values(sensors), iter.values(distances)) do
        x_lo = math.min(x_lo, sensor[1] - distance)
        x_hi = math.max(x_hi, sensor[1] + distance)
    end

    local count = 0
    local y = 2000000
    for x = x_lo, x_hi do
        local beacon_can_be_here = true
        local point = ps.create { x, y }
        for i = 1, n_sensors do
            local sensor, beacon, distance = sensors[i], beacons[i], distances[i]
            if point == beacon then
                beacon_can_be_here = true
                break
            elseif sensor:mdistance(point) <= distance then
                beacon_can_be_here = false
            end
        end
        if not beacon_can_be_here then
            count = count + 1
        end
    end
    return count
end

local function can_beacon_be_here(sensors, distances, position)
    local x,y = unpack(position)
    local limit = 4000000
    if x < 0 or x > limit or y < 0 or y > limit then
        return false
    end
    for i = 1, #sensors do
        local sensor, distance = sensors[i], distances[i]
        if sensor:mdistance(position) <= distance then
            return false
        end
    end
    return true
end

function M.solve_p2(lines)
    local sensors, _, distances = parse(lines)

    for i = 1, #sensors do
        local dist = distances[i] + 1
        local x0, y0 = unpack(sensors[i])
        for dy = -dist, dist do
            local dx = dist - math.abs(dy)
            local x1, x2, y = x0 - dx, x0 + dx, y0 + dy
            if can_beacon_be_here(sensors, distances, { x1, y }) then
                return 4000000 * x1 + y
            elseif can_beacon_be_here(sensors, distances, { x2, y }) then
                return 4000000 * x2 + y
            end
        end
    end
    assert(false)
end

function M.test() end

M = {
    M.solve_p1,
    M.solve_p2,
}

return M

-- vim: set et ts=4 sw=4 ss=4 tw=100 :
