mod blob_guy;
use crate::blob_guy::Blob;
use noita_api::lua::LUA;
use noita_api::lua::LuaState;
use noita_api::lua::lua_bindings::{LUA_REGISTRYINDEX, lua_State};
use noita_api_macro::add_lua_fn;
use smallvec::SmallVec;
use std::cell::{LazyCell, RefCell};
use std::ffi::c_int;
use std::hint::black_box;
use std::sync::LazyLock;
struct State(SmallVec<[Blob; 4]>);
impl Default for State {
    fn default() -> Self {
        Self(SmallVec::new())
    }
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
        fn update(_: LuaState) -> eyre::Result<()> {
            STATE.with(|state| {
                let mut state = state.try_borrow_mut()?;
                if state.0.is_empty() {
                    state.0.push(Blob::default())
                }
                for blob in state.0.iter_mut() {
                    blob.update()
                }
                Ok(())
            })
        }
        add_lua_fn!(update);
    }
    1
}
