local M = {}

local util = require("advent-of-code.utils")

-- This is just here to quickly try out stuff
local function test_fun(opts)
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

    -- Create a floating window with some text
    local scratch_buffer = vim.api.nvim_create_buf(false, true)
    -- Get cursor position for current window
    local pos = vim.api.nvim_win_get_cursor(0)
    vim.api.nvim_buf_set_lines(scratch_buffer, 0, -1, true, { "Test", "text" })
    vim.api.nvim_open_win(scratch_buffer, 0, {
        relative = 'cursor',
        row = pos[1] - 1, -- Because for some reason, row is 1-indexed
        col = pos[2],
        width = 10,
        height = 2,
        style = 'minimal',
        border = 'shadow'
    })

    vim.pretty_print("Hi!")
end

local function aoc(opts)
    local year, day, part = string.match(opts.args, "(%d+) (%d+) (%d)")
    assert(year and day and part, "Missing <year> <day> <part>")
    local solver = require(string.format('advent-of-code.%04d.%02d', year, day))
    if part == "0" then
        solver.test()
    else
        local lines = vim.api.nvim_buf_get_lines(0, opts.line1 - 1, opts.line2, false)
        local solution = solver[tonumber(part)]
        local answer
        if solution ~= nil then
            answer = solution(lines, opts)
        else
            answer = solver.solve(lines, tonumber(part), opts)
        end
        if answer ~= nil then
            util.printf("Answer: %s", answer)
        end
    end
end

-- Load solutions, and bind them to commands
vim.api.nvim_create_user_command('AoC', aoc, {
    nargs = 1, -- Take one argument
    range = '%', -- Take a selection. Default is entire file
})

-- Just create a test command, to try things out
vim.api.nvim_create_user_command('TestFun', test_fun, {
    nargs = '?', range = '%'
})

return M

-- vim: set et ts=4 sw=4 ss=4 tw=100 :
