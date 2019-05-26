# ArborOS

Just playin' with building an OS in Rust.

Based on https://os.phil-opp.com/

## Prereqs:

1. Linux. On Windows, WSL is suported. On macOS, try Docker?
1. QEMU
1. [rustup](https://rustup.rs/)
1. Run `scripts/install-prereqs.sh`

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
