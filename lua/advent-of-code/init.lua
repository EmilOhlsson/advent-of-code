local M = {}

local util = require("advent-of-code.utils")

-- This is just here to quickly try out stuff
function test_fun(opts)
    print("Hello, world " .. vim.inspect(opts))
    print(string.format("Got argument: %s", opts.args))
    vim.notify("Hello, world, as a notification")
    print(vim.version().api_level)

    local s_start = vim.fn.getpos("'<")
    print("Got s_start = " .. vim.inspect(s_start))

    local lines = vim.api.nvim_buf_get_lines(0, opts.line1 - 1, opts.line2, false)
    -- string.match

    print(string.format("got lines %s", vim.inspect(lines)))
    util.printf("This is a formatted string %s", "Indeed")

    if opts.args == "foo" then
        solve_2021_01_2(lines)
    else
        solve_2021_01_1(lines)
    end
end

function aoc(opts)
    local year, day, part = string.match(opts.args, "(%d+) (%d+) (%d)")
    assert(year and day and part, "Missing <year> <day> <part>")
    local solution = require(string.format('advent-of-code.%04d.%02d', year, day))
    local lines = vim.api.nvim_buf_get_lines(0, opts.line1 - 1, opts.line2, false)
    local answer = solution.solve(lines, tonumber(part))
    util.printf("Answer: %s", answer)
end

-- Load solutions, and bind them to commands
vim.api.nvim_create_user_command('AoC', aoc,{
    nargs = 1, -- Take one argument
    range = '%', -- Take a selection. Default is entire file
})

-- Just create a test command, to try things out
vim.api.nvim_create_user_command('TestFun', test_fun, {
    nargs = '?', range = '%'
})

return M

-- vim: set et ts=4 sw=4 ss=4 tw=100 :
