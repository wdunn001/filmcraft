# Pilot: filmcraft-web craft-runtime bridge

FilmCraft-side checklist for wiring `apps/filmcraft-web` to [craft-runtime](https://github.com/storytold/craft-libs/tree/main/crates/runtime). The cross-repo map (OPFS, GPU providers, tier-store keys) lives in craft-libs:

**[craft-libs `docs/pilot-filmcraft.md`](https://github.com/storytold/craft-libs/blob/main/docs/pilot-filmcraft.md)**

Cloud sync is out of scope here (CollabCraft / `cloud-delegate` later). No `craft-nextcloud` in the app.

---

## This branch (minimal slice)

- [x] Git dependency on `craft-runtime` (`wdunn001/craft-libs` until `craft-runtime` lands on the `craft-ui` rev in `storytold/craft-libs`).
- [x] `apps/filmcraft-web/src/craft_runtime_bridge.rs` — JSON host/worker round-trip (pilot; does not replace OPFS/recovery).
- [x] Native unit test: `init`, `storage_put` / `storage_flush`, `craft.noop` GPU batch (`cargo test -p filmcraft-web craft_runtime_bridge`).

---

## Next (track craft-libs pilot sections A–E)

See **[pilot-filmcraft.md](https://github.com/storytold/craft-libs/blob/main/docs/pilot-filmcraft.md)** — sections *Dependencies and features*, *Storage / autosave*, *Worker / GPU*, *UI shell*, *Verification gates*.

### Dependencies and wiring

- [ ] Enable `craft-runtime` features (`web`, `shared-worker`) when SharedWorker host ships on CollabCraft static assets.
- [ ] Connect bridge to SharedWorker RPC (parallel path; keep eframe shell on main thread).
- [ ] Register FilmCraft GPU providers (`filmcraft.*`) after init.

### Storage

- [ ] Route recovery/autosave through tier-store / runtime storage messages instead of direct `opfs` calls.
- [ ] Preserve `?fresh`, `?norecover`, media restore semantics.

### Verification

- [ ] `cargo fmt`, clippy, native bridge test, `wasm32` web build (`cargo xtask web`).
- [ ] Manual recovery + OPFS media restore before cutting over persistence.
