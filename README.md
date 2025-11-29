# Installation

[Install Rust](https://rust-lang.org/tools/install/) to manually compile. Alternatively, if you are on windows, you can grab the EXE from the releases section.

### Manual compilation:
Once you have Rust installed, open terminal/powershell and navigate to the repo:
```
cargo build --release
```
You should get a binary/executable in `[repo_folder_location]/target/release` named `connect4`.

# Controls

`Enter/S/Down`: Drop the checker down

`A/Left`: Move a column left

`D/Right`: Move a column right

`Esc`: Exit interface