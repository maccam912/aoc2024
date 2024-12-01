# Advent of Code 2024

My solutions for [Advent of Code 2024](https://adventofcode.com/2024) in Rust.

## Project Structure

```
.
├── src/           # Solution code
│   ├── dayXX.rs   # One file per day
│   ├── lib.rs     # Common functionality
│   └── main.rs    # CLI interface
├── inputs/
│   ├── sample/    # Example inputs from the problem descriptions
│   └── real/      # Your personal inputs (not in git)
└── README.md
```

## Running Solutions

Build and run the project using Cargo. You can:

- Run a specific day:
  ```bash
  cargo run -- --day 1          # Run day 1 with real input
  cargo run -- --day 1 --sample # Run day 1 with sample input
  ```

- Run all implemented solutions:
  ```bash
  cargo run                     # Run all days with real input
  cargo run -- --sample        # Run all days with sample input
  ```

## Input Files

- Sample inputs (from problem descriptions) are included in the repository under `inputs/sample/`
- Place your personal puzzle inputs in `inputs/real/` (this directory is git-ignored)
- Input files should be named `XX.txt` where XX is the two-digit day number (e.g., `01.txt`, `02.txt`, etc.)

## Adding New Days

1. Copy the template from an existing day (e.g., `day01.rs`)
2. Create a new file `src/dayXX.rs`
3. Add `pub mod dayXX;` to `lib.rs`
4. Add the day to the match expression in `get_solution()` in `lib.rs`
5. Create input files:
   - `inputs/sample/XX.txt` with the example from the problem
   - `inputs/real/XX.txt` with your personal input
