# Rust File Organizer

A simple CLI tool for organizing files based on their extensions.  
This project is designed as a real-world Rust practice exercise using an **incremental development** approach — building feature by feature instead of all at once.
structre project can see on [this](STRUCTURE_PROJECT.md)

---

## Project Goals

- Strengthen Rust fundamentals: ownership, borrowing, and error handling.
- Learn filesystem interaction (reading directories, moving files).
- Practice using common Rust crates: `clap`, `walkdir`, `serde`, `indicatif`.
- Apply a professional development workflow with milestones and versioning.

---

## Target Feature Overview

| Feature | Status |
|--------|--------|
| Scan folder and list files | pending |
| Group files by extension | pending |
| CLI arguments (`--path`, `--dry-run`) | pending |
| Organize (move files) | pending |
| Progress bar + logging | pending |
| `.toml` config support for custom rules | pending |

---

## Milestone Breakdown

The project will be developed in small iterative releases.  
Each milestone corresponds to a version tag.

| Version | Goal | Description |
|--------|------|-------------|
| `v0.1` | Directory Scanner | Display all files in a target directory |
| `v0.2` | File Grouping | Group scanned files by extension |
| `v0.3` | CLI Support | Add CLI arguments (`clap`) including `--path` |
| `v0.4` | File Moving | Move files based on extension categories |
| `v0.5` | UX Improvements | Add progress bar (`indicatif`) and logging |
| `v1.0` | Config Support | Allow custom rules via `.toml` config |

---

## Kanban Task List

> Tasks are aligned with each milestone.  
> Check them off as progress is made.

### 🔹 Milestone: `v0.1 — Directory Scanner`

- [ ] Initialize project using `cargo new`
- [ ] Implement directory reading (`std::fs`, `walkdir`)
- [ ] Output list of detected files
- [ ] Add basic error handling (missing folder, permissions)
- [ ] Add minimal unit tests
- [ ] Commit & tag release `v0.1`

---

### 🔹 Milestone: `v0.2 — File Grouping`

- [ ] Extract file extensions
- [ ] Store results in `HashMap<String, Vec<Path>>`
- [ ] Print grouped results
- [ ] Add error handling for files without extensions
- [ ] Refactor into modular functions
- [ ] Commit & tag release `v0.2`

---

### 🔹 Milestone: `v0.3 — CLI Support`

- [ ] Add `clap` dependency
- [ ] Add CLI options:
    - `--path <directory>`
    - `--dry-run`
- [ ] Validate input arguments
- [ ] Commit & tag release `v0.3`

---

### 🔹 Milestone: `v0.4 — File Moving`

- [ ] Implement `move_to_folder(ext)` logic
- [ ] Auto-create directories if needed
- [ ] Add safe-failure mode (skip problematic files)
- [ ] Commit & tag release `v0.4`

---

### 🔹 Milestone: `v0.5 — UX Upgrade`

- [ ] Add progress bar using `indicatif`
- [ ] Add formatted CLI logs (info/warning/success)
- [ ] Commit & tag release `v0.5`

---

### 🔹 Milestone: `v1.0 — Config Support (.toml)`

- [ ] Implement config parsing using `serde` + `toml`
- [ ] Allow custom extension → folder mapping
- [ ] Update documentation
- [ ] Commit & tag release `v1.0`

---

## Tech Stack

| Category | Tool |
|---------|------|
| Language | Rust |
| CLI parsing | `clap` |
| Filesystem traversal | `walkdir` |
| Config parsing | `serde` + `toml` |
| UX enhancement | `indicatif` |

---

