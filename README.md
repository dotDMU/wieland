# README – Erstinstallation & Erstlauf

Diese Anleitung beschreibt die **Erstinstallation** der Werkzeuge und den **ersten Start** des Projekts (Rust + WebAssembly + Leptos + Trunk). Sie ist bewusst pragmatisch gehalten und funktioniert auf **macOS**, **Linux** und **Windows** (PowerShell).

---

## 1) Voraussetzungen

- **Git** (zum Klonen des Repos)
- **Chrome** oder **Chromium** (für Browser-Tests mit `wasm-pack test`)

> Tipp: Für Windows empfiehlt sich **PowerShell** oder **Windows Terminal**.

---

## 2) Rust & Toolchain installieren

### macOS / Linux
```bash
# Rustup installieren (interaktiv, bestätigt die Installation)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Aktuelle Sitzung auf die neue Cargo-Umgebung vorbereiten
source "$HOME/.cargo/env"
```

### Windows (PowerShell)
1. Installer laden: https://win.rustup.rs
2. Ausführen und Standard-Einstellungen übernehmen.

### Toolchain prüfen
```bash
rustc --version
cargo --version
rustup --version
```

---

## 3) WebAssembly-Target hinzufügen
Damit der Rust-Teil als **WASM** gebaut wird:
```bash
rustup target add wasm32-unknown-unknown
```

---

## 4) Build- & Dev-Tools installieren

Wir nutzen **Trunk** (statischer WASM-Bundler/Dev-Server) und optional **Leptos**-CLI sowie **wasm-pack** für Browser-Tests.

```bash
# Trunk (erforderlich für Dev-Server & Builds)
cargo install trunk

# wasm-pack (empfohlen für Browser-Tests mit wasm-bindgen-test)
cargo install wasm-pack

# Leptos CLI (optional – falls „cargo leptos …“ genutzt wird)
cargo install cargo-leptos
```

### Versionen prüfen
```bash
trunk --version
wasm-pack --version
cargo leptos --help   # gibt Usage-Hilfe aus, falls installiert
```

---

## 5) Repository klonen
```bash
git clone https://github.com/dotDMU/wieland.git
cd wieland
```

> Falls es projektinterne Beispiel-Configs gibt (z. B. `.env.example`), kopiere sie jetzt nach `.env` und passe sie an.

---

## 6) (Optional) Projektabhängigkeiten lokal auflösen
Normalerweise erledigt **Cargo** dies beim ersten Build automatisch. Falls du Pakete vorab auflösen willst:
```bash
cargo fetch
```

---

## 7) Entwicklung starten (Dev-Server)

Die meisten Leptos/WASM-Setups im Frontend werden mit **Trunk** entwickelt. Wechsle ins Client-/PWA-Verzeichnis (falls euer Repo ein Unterprojekt für die PWA hat) und starte den Server.

```bash
# Variante A: Projektwurzel (falls dort Trunk.toml bzw. index.html liegt)
trunk serve --open

# Variante B: Unterordner (Beispiel)
cd crates/app
trunk serve --open
```

- `--open` öffnet den Browser automatisch.
- Trunk beobachtet Quelländerungen und lädt die Seite neu.

> Falls euer Projekt **SSR** oder **cargo-leptos** nutzt, kann alternativ gelten:
> ```bash
> cargo leptos watch
> ```
> (Bitte die projektspezifische Dokumentation prüfen.)

---

## 8) Build (Release)
```bash
# Release-Build (minifiziert) – Ausgabe geht nach /dist
trunk build --release
```

Statisches Artefakt liegt anschließend in `dist/` und kann von einem beliebigen Webserver ausgeliefert werden.

---

## 9) Tests ausführen (inkl. Browser/WASM)

Für reine Rust-Unit-Tests (nicht-WASM):
```bash
cargo test
```

Für **WASM-/Browser-Tests** mit `wasm-bindgen-test` (z. B. für IndexedDB, DOM, Web APIs):
```bash
# 1) Sicherstellen, dass das WASM-Target installiert ist (siehe Schritt 3)
# 2) Browser-Tests starten (Chrome/Chromium)
wasm-pack test --chrome --headless -- --features wasm-tests
```

> Hinweise:
> - `--headless` kann weggelassen werden, um den Browser sichtbar zu starten.
> - Falls ihr Feature-Flags nutzt (z. B. `wasm-tests`), in den Befehl übernehmen.
> - Alternativ: `wasm-pack test --firefox` (sofern installiert).

---

## 10) Häufige Stolpersteine & Lösungen

- **Fehlendes WASM-Target**: `error: target not found: wasm32-unknown-unknown` → `rustup target add wasm32-unknown-unknown`.
- **Trunk nicht gefunden**: `command not found: trunk` → `cargo install trunk` und PATH prüfen (neues Terminal öffnen).
- **Windows: fehlende OpenSSL/Build-Tools**: Installiere die **Visual Studio Build Tools** (C++ Build Tools) und/oder `vcpkg`. Meist genügt der Rustup-Installer + neuestes Windows SDK.
- **Browser blockiert lokale Datei-APIs**: Immer über `trunk serve` starten (kein `file://`), sonst funktionieren Fetch/IndexedDB/Service Worker ggf. nicht korrekt.
- **Chrome nicht gefunden bei `wasm-pack test`**: Chrom(e/ium) installieren oder `--firefox` nutzen.

---

## 11) Nächste Schritte

1. **Tests ausführen**: Schaue dir die vorhandenen **Unit-Tests** an – sie definieren die zu implementierenden Funktionen (TDD).
2. **Grün bekommen**: Implementiere die Ports/Adapter (z. B. IndexedDB-Repo) so, dass die Tests erfolgreich sind.
3. **Optional**: Lint/Format – `cargo fmt`, `cargo clippy`.

Viel Erfolg! Wenn etwas in deiner Umgebung abweicht (Ordnerstruktur, SSR/CSR, Feature-Flags), passe die Befehle minimal an. Die oben genannten Schritte decken >90 % der Setups ab.
