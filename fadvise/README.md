# fadvise
`fadvise` is a CLI command to call `posix_fadvise(2)` for a specific file.

## How to use
The following is an example of cache eviction.

```shell
❯ cat Cargo.toml > /dev/null
❯ fincore -b Cargo.toml 
  RES PAGES SIZE FILE
 4096     1  500 Cargo.toml
❯ fadvise dontneed Cargo.toml 
filename: Cargo.toml
advice: POSIX_FADV_DONTNEED
offset: 0
len: 500
❯ fincore -b Cargo.toml
RES PAGES SIZE FILE
  0     0  500 Cargo.toml
```

## Installation

### From crates.io

```shell
❯ cargo install fadvise
```

### From source code (GitHub)

```shell
❯ git clone https://github.com/StoneDot/linux-tools.git
❯ cd linux-tools/fadvise/
❯ cargo install --path .
```

## Completion
### bash
```shell
# Create a directory to store a completion code
❯ mkdir -p $HOME/.local/share/bash-completion/completions
# Place a completion code
❯ fadvise completion --shell bash > $HOME/.local/share/bash-completion/completions/fadvise
```

### zsh

```shell
# Create a directory to store a completion code
❯ mkdir -p $HOME/.zsh.d/functions
# Place a completion code
❯ fadvise completion --shell zsh > $HOME/.zsh.d/functions/_fadvise
# Execute the following line if your fpath does not include `$HOME/.zsh.d/functions`
❯ echo '[ "${fpath[(I)$HOME/.zsh.d/functions]}" -eq 0 ] && fpath=($fpath $HOME/.zsh.d/functions)' | tee -a $HOME/.zshrc
```
