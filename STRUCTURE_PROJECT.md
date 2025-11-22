## Structure
```json
rust-file-organizer/
│
├── src/
│   ├── main.rs
│   ├── scanner.rs        # v0.1: directory scanning logic
│   ├── grouping.rs       # v0.2: grouping logic
│   ├── cli.rs            # v0.3: argument parsing
│   ├── mover.rs          # v0.4: file moving logic
│   ├── ui.rs             # v0.5: progress bar + logging
│   └── config.rs         # v1.0: .toml config loading
│
├── Cargo.toml
└── README.md
```