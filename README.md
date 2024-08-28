<br/><br/>
<div>
    <h3 align="center">🕵️ Deja vu</h3>
    <p align="center">
        A simple command tool to identify and handle duplicated files over directories.
    </p>
</div>


## Getting Started

### Prerequisites

Install [Rust](https://www.rust-lang.org/tools/install) if it's not already installed.

### Installation

Build it from source. This is just a toy tool for learning purposes, so I won't upload to crates.io nor provide binaries.

## Usage

```
dejavu --action <ACTION> <DIRECTORIES>...
```

Possible actions:
- report: Reports duplicate files to the console along with their hash.
- delete: Prompts the user to select zero or more files to delete from each duplicate set.
- symlink: Prompts the user to pick the original file. All other files are converted into symlinks.

## License

[MIT](https://github.com/williamfedele/dejavu/blob/main/LICENSE)
