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
function table.copye(tbl, except)
    assert(type(tbl) == 'table')
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
                local destinations_new = table.copye(destinations, destination)
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

function table.copy(e)
    if type(e) == 'table' then
        local e_new = {}
        for k, v in pairs(e) do
            e_new[k] = table.copy(v)
        end
        setmetatable(e_new, getmetatable(e))
        return e_new
    else
        return e
    end
end

local function set_to_string(set)
    local list = {}
    for x, _ in pairs(set) do
        table.insert(list, x)
    end
    table.sort(list)
    local result = ""
    if #list > 0 then
        result = result .. list[1]
        for i = 2, #list do
            result = result .. ',' .. list[i]
        end
    end
    return result
end

local function get_set_size(set)
    local count = 0
    for _, _ in pairs(set) do
        count = count + 1
    end
    return count
end

local function is_disjoint(set1, set2)
    assert(type(set1) == 'table' and type(set2) == 'table')
    for k, _ in pairs(set1) do
        if set2[k] then
            return false
        end
    end
    for k, _ in pairs(set2) do
        if set1[k] then
            return false
        end
    end
    return true
end

local function string_to_set(str)
    local result = {}
    for _, e in ipairs(util.split_string(str, ',')) do
        result[e] = true
    end
    return result
end

function M.solve_p2(lines)
    local valves, tunnels = parse(lines)
    local destinations = get_nonzero_valves(valves)
    local distances = calculate_distances(tunnels)

    -- Calculate best achievable venting for each given set of visited
    local results = {}
    setmetatable(results, {
        __index = function(tbl, k)
            return rawget(tbl, tostring(k))
        end,
        __newindex = function(tbl, k, v)
            rawset(tbl, tostring(k), v)
        end,
    })
    local function walk(node, remaining, visited, vented)
        local best = results[visited] or 0
        results[visited] = math.max(best, vented)

        for destination, _ in pairs(destinations) do
            if not visited[destination] then
                local travel_time = distances[node][destination] + 1 -- Travel + opening valve
                local remaining_new = remaining - travel_time
                if remaining_new >= 0 then
                    local visited_new = table.copy(visited)
                    visited_new[destination] = true
                    walk(destination, remaining_new, visited_new, vented + remaining_new * valves[destination])
                end
            end
        end
    end

    local visited = {}
    setmetatable(visited, {
        __tostring = function(self)
            return set_to_string(self)
        end,
    })
    walk('AA', 26, visited, 0)

    local sum = 0
    for s1, f1 in pairs(results) do
        for s2, f2 in pairs(results) do
            -- Slow and yucky to convert to string, but hey...
            if is_disjoint(string_to_set(s1), string_to_set(s2)) then -- Visit sets does not overlap
                sum = math.max(sum, f1 + f2)
            end
        end
    end

    return sum
end

function M.test()
    print('bit', vim.inspect(bit.lshift(1, 31)))

    print('set', vim.inspect(string_to_set('AA,BB,CC,DD')))
    print('disjoint', is_disjoint(string_to_set('AA,BB,CC,DD'), string_to_set('BB')))

    local a = { a = 1, b = 2, c = 3, d = 4, e = 5, f = 6, g = 7, h = 8, i = 9, j = 10, k = 11, l = 12, m = 13 }
    local count = 0
    for _, _ in pairs(a) do
        for _, _ in pairs(a) do
            count = count + 1
        end
    end
    assert(count == (13 * 13))
end

M[1] = M.solve_p1
M[2] = M.solve_p2

return M

-- vim: set et ts=4 sw=4 ss=4 tw=100 :
