# rp

Experimental implementation aims to be substitute for `cp` command.

For short, it's implemented for my practice. :smile:

Please give me advice when it's not fit for Rust culture.

## Performance

Not evaluated.

## Execution

**Build**
```bash
cargo build --release
```

**Execution**
```bash
# Copy file0 into file1
target/release/rp file0 file1
```

### Options
- `-f`: Overwrite the copied file even if it exists. (Same as `cp`)
- `-r`: Copy files in the directory recursively. (Same as `cp`)
- `-m`: Switch copy modes.

There are three modes in `rp` when copying multiple files.
- `seq`: Sequential copy.
- `fut`: Concurrent copy with [futures-fs](https://github.com/seanmonstar/futures-fs).
- `fut2`: Concurrent copy with basic futures.

Default mode is `fut2`.

## Current Restriction
- Not all functions of `cp` are covered.
- `too many open files` error occurs when you copy directory which contains too many files.
- File permissions are not copied in `fut` and `fut2` mode. It's related to [this](https://github.com/seanmonstar/futures-fs/issues/10) issue.
