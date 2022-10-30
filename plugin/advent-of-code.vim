if exists("g:loaded_aocplugin")
	finish
endif
let g:loaded_aocplugin = 1

lua require("advent-of-code")
