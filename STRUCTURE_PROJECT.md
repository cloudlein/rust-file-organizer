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

## Test files structure 
```json
test_files/
├── image.png
├── video.mp4
├── readme.txt
├── doc.pdf
└── old_backup.zip
```

to run organizer: 
```cmd
cargo run -- --path ./test_files --dry-run
```

## workflow v1.0
```json
User runs command
        ↓
CLI reads arguments
        ↓
Resolve target directory (user input OR working directory)
        ↓
Load config (if exists)
        ↓
Scan folder (recursively or not, based on config)
        ↓
Group detected files by extension rule
        ↓
If dry-run: show planned actions only
Else: move files to destination folders
        ↓
Show summary + stats
```