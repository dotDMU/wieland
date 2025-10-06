//! Ziel dieser Tests: IndexedDB-Repo für `Item` TDD-getrieben fertigstellen.
//! Läuft im Browser-Runner von wasm-bindgen-test.
//! Setup/Runner: https://wasm-bindgen.github.io/wasm-bindgen/wasm-bindgen-test/asynchronous-tests.html

use wasm_bindgen_test::*;
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

// ⬇️ Passe diesen Import einmalig an die echte Position deines Item-Structs an:
use domain::item::Item;
use domain::item_contract::ItemContract;

// Repo-API (Port) – diese Traits/Funktionen sind deine TDD-Ziele:
use persistence::idb_item_repo::IdbItemRepo;

/// Kleine Fixture-Helferfunktion, um ein `Item` zu bauen.
/// 👉 Falls deine Felder abweichen, **nur HIER** anpassen.
/// Tipp zu Serde-Feldern: https://serde.rs/attributes.html
fn mk_item(name: &str) -> Item {
    // Wenn dein Item Default hat:
    // let mut it = Item::default();
    // it.name = name.to_string();
    // it.id = format!("test-{}", name);
    // it

    // Falls `Item` Konstruktoren hat, nutze sie:
    // Item::new_with_id(format!("test-{}", name), name.to_string())

    // Platzhalter – bitte eine der Varianten oben aktivieren.
    unimplemented!("Passe mk_item() minimal an dein Item-Struct an.")
}

#[wasm_bindgen_test]
async fn open_db_creates_store_and_indexes() {
    // ❓ Wie IndexedDB-Store/Index anlegen:
    // Rust-Crate (futures API): https://docs.rs/idb
    // MDN createIndex: https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/createIndex

    // act
    let repo = IdbItemRepo::new().await.expect("DB should open");

    // assert – Metadaten-Inspektion über Test-Hilfen deiner DB-Schicht (bereitstellen):
    let meta = repo.inspect_metadata().await.expect("inspect ok");
    assert!(meta.stores.contains(&"items".to_string()), "ObjectStore `items` fehlt");
    assert!(meta.indexes.contains(&"by_name".to_string()), "Index `by_name` fehlt");
}

#[wasm_bindgen_test]
async fn create_and_get_roundtrip() {
    // ℹ️ Async Tests unter wasm: https://wasm-bindgen.github.io/wasm-bindgen/wasm-bindgen-test/asynchronous-tests.html
    let repo = IdbItemRepo::new().await.unwrap();

    let item = mk_item("Animal Oil Residue");
    let saved = repo.create(item.clone()).await.expect("create");

    // Erwartung: ID stabil/vergeben & Get liefert Gleichheit (Serde-Roundtrip).
    let fetched = repo.get(&saved.id).await.expect("get call").expect("found");
    assert_eq!(fetched, item, "Persistierter Datensatz muss identisch sein");
}

#[wasm_bindgen_test]
async fn update_modifies_fields_preserves_id() {
    // 🔗 IndexedDB Schreibtransaktionen (idb): https://docs.rs/idb
    let repo = IdbItemRepo::new().await.unwrap();

    let mut item = mk_item("Lysterium Ingot");
    let _ = repo.create(item.clone()).await.unwrap();

    // simuliere Änderungen (passe Felder an dein Item an, z. B. quantity/markup)
    // item.quantity = 200;
    // item.markup = Some(102.3);

    let upd = repo.update(item.clone()).await.unwrap();
    assert_eq!(upd.id, item.id, "ID darf sich nicht ändern");

    let fetched = repo.get(&item.id).await.unwrap().unwrap();
    assert_eq!(fetched, item);
}

#[wasm_bindgen_test]
async fn delete_removes_item() {
    // 🔗 Transaktionen & Rückgaben: https://docs.rs/idb
    let repo = IdbItemRepo::new().await.unwrap();

    let item = mk_item("Weapon Component");
    repo.create(item.clone()).await.unwrap();

    let existed = repo.delete(&item.id).await.unwrap();
    assert!(existed, "delete sollte true liefern, wenn etwas entfernt wurde");

    let missing = repo.get(&item.id).await.unwrap();
    assert!(missing.is_none(), "Datensatz muss gelöscht sein");
}

#[wasm_bindgen_test]
async fn list_returns_sorted_by_name() {
    // 🔗 Indexe & Sortierung: https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/createIndex
    let repo = IdbItemRepo::new().await.unwrap();
    repo.clear().await.unwrap();

    for name in ["A", "C", "B"] {
        repo.create(mk_item(name)).await.unwrap();
    }

    let all = repo.list().await.unwrap();
    let got: Vec<_> = all.iter().map(|i| i.name.as_str()).collect();
    assert_eq!(got, vec!["A", "B", "C"], "Standard-Listing: alphabetisch nach Name");
}

#[wasm_bindgen_test]
async fn query_by_name_prefix_uses_index() {
    // 🔗 Alternative Crate mit JS-ähnlichem API (falls dir idb zu low-level ist):
    // https://docs.rs/indexed_db_futures/
    let repo = IdbItemRepo::new().await.unwrap();
    repo.clear().await.unwrap();

    for name in ["Lysterium Ingot", "Lysterium Stone", "Animal Oil"] {
        repo.create(mk_item(name)).await.unwrap();
    }

    let lyst = repo.find_by_name_prefix("Lysterium").await.unwrap();
    let names: Vec<_> = lyst.iter().map(|i| i.name.as_str()).collect();
    assert_eq!(names, vec!["Lysterium Ingot", "Lysterium Stone"]);
}

#[wasm_bindgen_test]
async fn pagination_with_cursor() {
    // 🔗 Cursors/Streams über web_sys-Wrapper: https://github.com/Alorel/rust-indexed-db
    let repo = IdbItemRepo::new().await.unwrap();
    repo.clear().await.unwrap();

    for i in 0..25 {
        repo.create(mk_item(&format!("Item-{i:02}"))).await.unwrap();
    }

    let page = repo.list_page( /*offset*/ 10, /*limit*/ 5).await.unwrap();
    assert_eq!(page.items.len(), 5);
    assert_eq!(page.items[0].name, "Item-10");
    assert!(page.total >= 25);
}

#[wasm_bindgen_test]
async fn migration_adds_missing_indexes_on_version_bump() {
    // 🔗 Indexe nachträglich hinzufügen (VersionChange): https://stackoverflow.com/q/11532935
    // 🔗 Typische Fehler „version change transaction is running“: https://stackoverflow.com/q/79156534
    let repo = IdbItemRepo::new().await.unwrap();

    // Simuliere alte DB (Test-Hilfsfunktion deiner DB-Schicht – nur im Test-Build):
    repo.simulate_old_schema_without_by_name().await.unwrap();

    // Re-open → onupgradeneeded sollte neuen Index anlegen
    let repo = IdbItemRepo::new_with_version_bump().await.unwrap();
    let meta = repo.inspect_metadata().await.unwrap();
    assert!(meta.indexes.contains(&"by_name".to_string()));
}

#[wasm_bindgen_test]
async fn serde_roundtrip_preserves_fields() {
    // 🔗 Serde-Attribute Guide: https://serde.rs/attributes.html
    let repo = IdbItemRepo::new().await.unwrap();

    let item = mk_item("Residue /*fülle optional Felder für max. Abdeckung*/");
    repo.create(item.clone()).await.unwrap();

    let back = repo.get(&item.id).await.unwrap().unwrap();
    assert_eq!(back, item, "JSON/Bincode <-> IndexedDB darf keine Felder verlieren");
}

#[wasm_bindgen_test]
async fn update_nonexistent_yields_not_found() {
    // 🔗 Teststrategie & Runner: https://github.com/wasm-bindgen/wasm-bindgen/blob/master/crates/test/README.md
    let repo = IdbItemRepo::new().await.unwrap();

    let ghost = mk_item("I do not exist");
    let err = repo.update(ghost).await.unwrap_err();
    assert!(err.is_not_found(), "Update auf fehlendem Datensatz muss NotFound liefern");
}

#[wasm_bindgen_test]
async fn create_duplicate_id_is_either_idempotent_or_errors_cleanly() {
    // 🔗 Async-Test-Tipps (Blocken geht nicht auf wasm): https://stackoverflow.com/q/73833350
    let repo = IdbItemRepo::new().await.unwrap();

    let it = mk_item("Dup");
    repo.create(it.clone()).await.unwrap();
    let second = repo.create(it.clone()).await;

    assert!(second.is_ok() || second.as_ref().err().unwrap().is_duplicate_key());
}
