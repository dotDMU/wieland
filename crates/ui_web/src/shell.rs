use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Shell(children: Children) -> impl IntoView {
    view! {
        <header style="padding:0.5rem 1rem; border-bottom:1px solid #ddd;">
            <nav style="display:flex; gap:1rem;">
                <A href="/hunting">"Hunting"</A>
                <A href="/crafting">"Crafting"</A>
                <A href="/items">"Items"</A>
            </nav>
        </header>

        <main style="padding:1rem;">
            {children()}
        </main>
    }
}
