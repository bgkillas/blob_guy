mod blob_guy;
mod noita;
use crate::blob_guy::Blob;
use crate::noita::ParticleWorldState;
use noita_api::lua::LUA;
use noita_api::lua::LuaState;
use noita_api::lua::lua_bindings::{LUA_REGISTRYINDEX, lua_State};
use noita_api_macro::add_lua_fn;
use smallvec::SmallVec;
use std::cell::{LazyCell, RefCell};
use std::ffi::{c_int, c_void};
use std::hint::black_box;
use std::sync::LazyLock;
#[derive(Default)]
struct State {
    particle_world_state: Option<ParticleWorldState>,
    blobs: SmallVec<[Blob; 4]>,
}
thread_local! {
    static STATE: LazyCell<RefCell<State>> = LazyCell::new(|| {
        State::default().into()
    });
}
static KEEP_SELF_LOADED: LazyLock<Result<libloading::Library, libloading::Error>> =
    LazyLock::new(|| unsafe { libloading::Library::new("blob_guy.dll") });
/// # Safety
///
/// Only gets called by lua when loading a module.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaopen_blob_guy(lua: *mut lua_State) -> c_int {
    let _ = black_box(KEEP_SELF_LOADED.as_ref());
    unsafe {
        LUA.lua_createtable(lua, 0, 0);
        LUA.lua_createtable(lua, 0, 0);
        LUA.lua_setmetatable(lua, -2);
        LUA.lua_newuserdata(lua, 0);
        LUA.lua_createtable(lua, 0, 0);
        LUA.lua_setmetatable(lua, -2);
        LUA.lua_setfield(lua, LUA_REGISTRYINDEX, c"luaclose_blob_guy".as_ptr());
        add_lua_fn!(update);
        add_lua_fn!(init_particle_world_state);
    }
    1
}
fn init_particle_world_state(lua: LuaState) {
    let world_pointer = lua.to_integer(1);
    let chunk_map_pointer = lua.to_integer(2);
    let material_list_pointer = lua.to_integer(3);
    STATE.with(|state| {
        state.borrow_mut().particle_world_state = Some(ParticleWorldState {
            _world_ptr: world_pointer as *mut c_void,
            chunk_map_ptr: chunk_map_pointer as *mut c_void,
            material_list_ptr: material_list_pointer as _,
            runner: Default::default(),
        });
    });
}
fn update(_: LuaState) -> eyre::Result<()> {
    STATE.with(|state| {
        let mut state = state.try_borrow_mut()?;
        if state.blobs.is_empty() {
            state.blobs.push(Blob::default())
        }
        for blob in state.blobs.iter_mut() {
            blob.update()
        }
        Ok(())
    })
}
