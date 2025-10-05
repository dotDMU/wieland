use std::rc::Rc;
use leptos::prelude::*;
use leptos::prelude::{Get, GetUntracked, Set};

use domain::item::Item;
use domain::item_contract::ItemContract;
use persistence::idb_item_repo::IdbItemRepo;

#[component]
pub fn ItemsView() -> impl IntoView {


    view! {
        <section>
            <h2>"Items"</h2>
            <div style="margin:.5rem 0; color:#555;">{move || msg.get()}</div>

            <div style="display:grid; gap:.5rem; grid-template-columns: 1fr 1fr 1fr auto; max-width:48rem;">
                <input placeholder="Name"
                    value=move || name.get()
                    on:input=move |ev| name.set(event_target_value(&ev)) />
                <input type="number" step="0.0001" placeholder="TT (PED)"
                    value=move || format!("{:.4}", tt.get())
                    on:input=move |ev| {
                        tt.set(event_target_value(&ev).parse::<f64>().unwrap_or(0.0));
                    } />
                <input type="number" step="1" placeholder="MU %"
                    value=move || format!("{:.0}", mu.get())
                    on:input=move |ev| {
                        mu.set(event_target_value(&ev).parse::<f64>().unwrap_or(100.0));
                    } />
                <button on:click=on_add>"Hinzufügen"</button>
            </div>

            <h3 style="margin-top:1rem;">"Gespeicherte Items"</h3>
            <table style="min-width:32rem; margin-top:.5rem; border-collapse:collapse;">
                <thead>
                    <tr style="border-bottom:1px solid #ddd;">
                        <th style="text-align:left;  padding:.25rem .5rem;">"Name"</th>
                        <th style="text-align:right; padding:.25rem .5rem;">"TT (PED)"</th>
                        <th style="text-align:right; padding:.25rem .5rem;">"MU %"</th>
                        <th style="text-align:right; padding:.25rem .5rem;">"Wert @ MU"</th>
                    </tr>
                </thead>
                <tbody>
                    {move || {
                        items.get().into_iter().map(|it| {
                            let sale = it.tt_value_ped * (it.mu_percent / 100.0);
                            view! {
                                <tr style="border-bottom:1px solid #eee;">
                                    <td style="padding:.25rem .5rem;">{it.name}</td>
                                    <td style="padding:.25rem .5rem; text-align:right;">{format!("{:.4}", it.tt_value_ped)}</td>
                                    <td style="padding:.25rem .5rem; text-align:right;">{format!("{:.0}", it.mu_percent)}</td>
                                    <td style="padding:.25rem .5rem; text-align:right;">{format!("{:.4}", sale)}</td>
                                </tr>
                            }
                        }).collect::<Vec<_>>()
                    }}
                </tbody>
            </table>
        </section>
    }
}
