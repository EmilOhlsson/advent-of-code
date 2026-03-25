local M = {}

---@class Logger
---@field private file_handle file*|nil File handle for file-based logging
---@field log fun(self: Logger, msg: string, ...: any) Log a formatted message

--- Create a logger instance
---@param enabled boolean Whether logging is enabled
---@param arg? number|string Log level or file path for file-based logging
---@param mode string|nil File open mode
---@return Logger
function M.create_logger(enabled, arg, mode)
    arg = arg or vim.log.levels.DEBUG
    if not enabled then
        return {
            log = function() end,
        }
    elseif type(arg) == 'number' then
        return {
            log = function(_, msg, ...)
                local formatted_msg = string.format(msg, ...)
                vim.notify(formatted_msg, vim.log.levels.DEBUG)
            end,
        }
    else
        assert(type(arg) == 'string')
        local file = io.open(arg, mode or 'a')
        assert(file, "unable to open log file for writing")
        return {
            file_handle = file,

            log = function (self, msg, ...)
                local formatted_msg = string.format(msg, ...)
                self.file_handle:write(formatted_msg .. '\n')
                self.file_handle:flush()
            end,
        }
    end
end

return M

-- vim: set et ts=4 sw=4 ss=4 tw=80 :
