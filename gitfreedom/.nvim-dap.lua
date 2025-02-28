-- Run the program
vim.keymap.set("n", "<leader>cr", function()
    vim.cmd("ToggleTerm")
    -- Send the command to the default terminal
    vim.cmd("TermExec cmd='cargo run -- config'")
end)

local dap = require("dap")

dap.adapters.lldb = {
    type = "executable",
    command = "/usr/bin/lldb-dap",
    name = "lldb",
}

dap.configurations.rust = {
    {
        name = "gf",
        type = "lldb",
        request = "launch",
        program = function()
            return vim.fn.getcwd() .. "/target/debug/gf"
        end,
        cwd = "${workspaceFolder}",
        runInTerminal = false,
        stopOnEntry = false,
        args = {"config"},
    },
}

