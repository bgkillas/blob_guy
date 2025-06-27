dofile_once("mods/quant.ew/NoitaPatcher/load.lua")
local np = require("noitapatcher")
package.cpath = package.cpath .. ";./mods/blob_guy/?.dll"
package.path = package.path .. ";./mods/blob_guy/?.lua"
local blob_guy = require("blob_guy")
local world_ffi = require("noitapatcher.nsew.world_ffi")
local ffi = require("ffi")
local nxml = dofile_once("mods/blob_guy/nxml.lua")
ModMaterialsFileAdd("mods/blob_guy/materials.xml")
function OnWorldPreUpdate()
    blob_guy.update()
end
local function send_mats()
    local mats = {}
    local content_materials = ModTextFileGetContent("data/materials.xml")
    local xml_orig = nxml.parse(content_materials)
    local inp = ""
    local i = 0
    local name = CellFactory_GetName(i)
    while name ~= "unknown" do
        mats[name] = i
        i = i + 1
        name = CellFactory_GetName(i)
        print(name)
    end
    local info = {}
    for element in xml_orig:each_child() do
        if element.name == "CellDataChild" or element.name == "CellData" then
            local hp = element.attr.hp or 100
            local dur = element.attr.durability or 0
            local cell_type = element.attr.cell_type or "liquid"
            local liquid_sand = element.attr.liquid_sand or 0
            local liquid_static = element.attr.liquid_static or 0
            if element.attr._parent ~= nil then
                local p = info[element.attr._parent]
                if p ~= nil then
                    dur = element.attr.durability or p[1]
                    hp = element.attr.hp or p[2]
                    cell_type = element.attr.cell_type or p[3]
                    liquid_sand = element.attr.liquid_sand or p[4]
                    liquid_static = element.attr.liquid_static or p[5]
                end
            end
            info[element.attr.name] = { dur, hp, cell_type, liquid_sand, liquid_static }
            inp = inp
                .. mats[element.attr.name]
                .. " "
                .. element.attr.name
                .. " "
                .. dur
                .. " "
                .. hp
                .. " "
                .. cell_type
                .. " "
                .. tostring(liquid_sand)
                .. " "
                .. tostring(liquid_static)
                .. " "
                .. tostring(element.attr.wang_color)
                .. " "
        end
    end
    print(inp)
    print(content_materials)
    blob_guy.register_mats(inp, i)
end
function OnWorldInitialized()
    local grid_world = world_ffi.get_grid_world()
    local chunk_map = grid_world.vtable.get_chunk_map(grid_world)
    grid_world = tonumber(ffi.cast("intptr_t", grid_world))
    chunk_map = tonumber(ffi.cast("intptr_t", chunk_map))
    local material_list = tonumber(ffi.cast("intptr_t", world_ffi.get_material_ptr(0)))
    blob_guy.init_particle_world_state(grid_world, chunk_map, material_list)
    send_mats()
end