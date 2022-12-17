local M = {}

local util = require('advent-of-code.utils')
local iter = require('advent-of-code.iterators')
local func = require('advent-of-code.functional')
local ps = require('advent-of-code.points')

local function parse(lines)
    local valves = {}
    local tunnels = {}
    for _, line in ipairs(lines) do
        local valve, rate = line:match('^Valve (%a%a) has flow rate=(-?%d+);')
        assert(valve ~= nil and rate ~= nil)
        valves[valve] = tonumber(rate)

        local _, tunn = unpack(util.split_string(line, ';'))
        tunn = tunn:sub(#" tunnels lead to valve" + 1)
        if tunn:sub(1, 1) == 's' then
            tunn = tunn:sub(3)
        end
        tunnels[valve] = util.split_string(tunn, ', ')
    end
    --print(vim.inspect(valves), vim.inspect(tunnels))
    return valves, tunnels
end

local function set_to_to_string(set)
    local list = {}
    for x, s in pairs(set) do
        if s then
            table.insert(list, x)
        end
    end
    local result = "["
    if #list > 0 then
        result = result .. list[1]
        for i = 2, #list do
            result = result .. ',' .. list[i]
        end
    end
    return result .. ']'
end

local function calculate_distances(tunnels)
    local distances = {}
    for start, _ in pairs(tunnels) do
        distances[start] = {}
    end
    for start, _ in pairs(tunnels) do
        local queue = { { start, 0 } }
        local visited = { [start] = true }
        --print('starting in', start)
        while #queue > 0 do
            local position, distance = unpack(table.remove(queue, 1))
            --print('In', position)
            for _, destination in ipairs(tunnels[position]) do
                if not visited[destination] then
                    --print(' visiting', destination, 'at distance', distance + 1)
                    visited[destination] = true
                    distances[start][destination] = distance + 1
                    table.insert(queue, { destination, distance + 1 })
                end
            end
        end
    end
    return distances
end

local function get_nonzero_valves(valves)
    local valves_nonzero = {}
    for valve, flow in pairs(valves) do
        if flow > 0 then
            valves_nonzero[valve] = true
        end
    end
    return valves_nonzero
end

-- Copy and return table, except `except`, if provided
local function copy_table(tbl, except)
    local tbl_copy = {}
    for x, y in pairs(tbl) do
        tbl_copy[x] = y
    end
    if except ~= nil then
        tbl_copy[except] = nil
    end
    return tbl_copy
end

function M.solve_p1(lines)
    local valves, tunnels = parse(lines)
    local valves_nonzero = get_nonzero_valves(valves)
    local distances = calculate_distances(tunnels)

    local function walk(node, time, rate, released, destinations)
        assert(time <= 30)
        rate = rate + valves[node] -- Always open the valve we arrive at
        local score = 0
        for destination, _ in pairs(destinations) do
            local travel_time = distances[node][destination] + 1 -- Also open
            if travel_time + time > 30 then
                local destination_score = released + (30 - time) * rate
                score = math.max(score, destination_score)
            else
                local destinations_new = copy_table(destinations, destination)
                local destination_score = walk(destination,
                    time + travel_time, rate, released + rate * travel_time,
                    destinations_new)
                score = math.max(score, destination_score)
            end
        end
        score = math.max(score, released + (30 - time) * rate) -- Might be no destinations
        return score
    end

    -- This works, because start position has rate = 0
    local answer = walk('AA', 0, 0, 0, valves_nonzero)
    return answer
end

function M.solve_p2(lines)
end

function M.test() end

M = {
    M.solve_p1,
    M.solve_p2,
}

return M

-- vim: set et ts=4 sw=4 ss=4 tw=100 :
