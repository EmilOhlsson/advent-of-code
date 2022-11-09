local M = {}

-- Binary heap, stored as an array
-- Children of node is stored in 2*i, and 2*i+1,
-- Parent of a node is calculated by i // 2


-- Create a new priority queue, if no `comparison`
-- function is given, then default to b - a will be
-- used
function M.new(comparison)
    if comparison == nil then
        comparison = function(a, b) return b - a end
    end
    return {
        size = 0,
        comparison = comparison
    }
end

-- Return the size of a priority queue
function M.getn(pq)
    return pq.size
end

-- Returns a consuming iterator
function M.iterator(pq)
    return function()
        return M.pop(pq)
    end
end

-- Push a value to the priority queue
function M.push(pq, value)
    -- Create a hole in the tree at the right place,
    -- also known as the percolate up operation
    local pos = pq.size + 1
    pq.size = pq.size + 1
    pq[pos] = value
    local parent_i = math.floor(pos / 2)
    while parent_i > 0 and pq.comparison(pq[pos], pq[parent_i]) > 0 do
        pq[pos], pq[parent_i] = pq[parent_i], pq[pos]
        pos = parent_i
        parent_i = math.floor(pos / 2)
    end
end

-- Pop first value from priority queue
function M.pop(pq)
    local value = pq[1]

    -- Copy last element first, and percolate down
    -- to not cause fragmentation in tree
    pq[1], pq[pq.size] = pq[pq.size], nil
    pq.size = pq.size - 1
    local pos = 1
    while true do
        local left_i = 2 * pos
        local right_i = left_i + 1
        local left = pq[left_i]
        local right = pq[right_i]
        if left == nil then --No children, done
            break
        elseif right == nil then -- One child
            if pq.comparison(pq[pos], left) < 0 then
                pq[pos], pq[left_i] = pq[left_i], pq[pos]
            end
            -- Done, at the bottom of the tree
            break
        else -- Both children
            local min_i
            -- Pick smallest child node
            if pq.comparison(pq[left_i], pq[right_i]) > 0 then
                min_i = left_i
            else
                min_i = right_i
            end
            -- Compare with current node
            if pq.comparison(pq[pos], pq[min_i]) < 0 then
                pq[pos], pq[min_i] = pq[min_i], pq[pos]
                pos = min_i
            else
                break
            end
        end
    end

    return value
end

function M.peek(pq)
    return pq[1]
end

return M

-- vim: set et ts=4 sw=4 ss=4 tw=100 :
