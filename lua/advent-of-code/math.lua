local M = {}

function M.gcd(x, y)
    if y == 0 then
        return x
    end
    return M.gcd(y, x % y)
end

return M

-- vim: set et ts=4 sw=4 ss=4 tw=100 :
