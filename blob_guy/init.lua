dofile_once("mods/quant.ew/NoitaPatcher/load.lua")
np = require("noitapatcher")
package.cpath = package.cpath .. ";./mods/blob_guy/?.dll"
package.path = package.path .. ";./mods/blob_guy/?.lua"
blob_guy = require("blob_guy")
function OnWorldPreUpdate()
    blob_guy.update()
end
