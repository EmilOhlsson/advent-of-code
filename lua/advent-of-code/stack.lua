local M = {}

-- Stack

function M.push(stack, value)
    table.insert(stack, value)
end

function M.pop(stack)
    return table.remove(stack)
end

function M.peek(stack)
    return stack[#stack]
end

return M

-- vim: set et ts=4 sw=4 ss=4 tw=100 :
