dofile_once("mods/quant.ew/NoitaPatcher/load.lua")
np = require("noitapatcher")
package.cpath = package.cpath .. ";./mods/blob_guy/?.dll"
package.path = package.path .. ";./mods/blob_guy/?.lua"
blob_guy = require("blob_guy")
ModMaterialsFileAdd("mods/blob_guy/materials.xml")
function OnWorldPreUpdate()
    blob_guy.update()
end
function OnWorldInitialized()
    local grid_world = world_ffi.get_grid_world()
    local chunk_map = grid_world.vtable.get_chunk_map(grid_world)
    grid_world = tonumber(ffi.cast("intptr_t", grid_world))
    chunk_map = tonumber(ffi.cast("intptr_t", chunk_map))
    local material_list = tonumber(ffi.cast("intptr_t", world_ffi.get_material_ptr(0)))
    blob_guy.init_particle_world_state(grid_world, chunk_map, material_list)
end
