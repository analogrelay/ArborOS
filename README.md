# ArborOS

Just playin' with building an OS in Rust.

## Prereqs:

1. QEMU ([Windows](https://qemu.weilnetz.de/w64/), Linux/macOS use package manager or Homebrew)
2. [rustup](https://rustup.rs/)
3. Run `scripts/install-prereqs.ps1` (`.sh` coming) to initialize other prereqs.

## Building

Once you have `bootimage` installed (see `scripts/install-prereqs.ps1`):

```
bootimage build
```

## Running

Once you have `bootimage` installed (see `scripts/install-prereqs.ps1`):

```
bootimage run
```

Or

```
qemu-system-x86_64 -drive format=raw,file=target/x86_64-arbor_os/debug/bootimage-arbor_os.bin
```

## The fanciest OS ever

![Fancy Screenshot](https://user-images.githubusercontent.com/7574/46517634-e881d200-c824-11e8-91fa-7934195503f4.png)
