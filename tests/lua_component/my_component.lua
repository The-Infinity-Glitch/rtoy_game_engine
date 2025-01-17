-- local rust_module = require("rust_module")

local my_var = 1

function on_ready( ... )
	print("Ready")
end

function on_update(delta)
	print("Component update: delta -> " .. delta)

	my_var = my_var + 1

	print("my var = " .. my_var)
end