use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
use persistence::idb_item_repo::IdbItemRepo;
use std::rc::Rc;

mod shell;
mod tabs;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Repo erzeugen (async) und per Context verfügbar machen
    mount_to_body(|| {

        view! {
            <Router>
                <shell::Shell>
                    <Routes fallback=|| view! { <p>"Not found"</p> }>
                        <Route path=path!("/")         view=tabs::hunting::HuntingView />
                        <Route path=path!("/hunting")  view=tabs::hunting::HuntingView />
                        <Route path=path!("/crafting") view=tabs::crafting::CraftingView />
                        <Route path=path!("/items")    view=tabs::items::ItemsView />
                    </Routes>
                </shell::Shell>
            </Router>
        }
    });
}

pub fn main() {} // für `cargo check` nativ
