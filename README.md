# dvOS

My WIP `Operating System` from scratch in `Rust`.

I'm following this awesome series of blog post tutorials called [Writing an OS in Rust](https://os.phil-opp.com/), while implementing some extra features myself along the way.

## Build Instructions

Firstly, you'll need to install [rust](https://www.rust-lang.org/).

```shell
❯ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then install rust nightly through rustup.

```shell
❯ rustup update nightly --force
```

To create a bootable image, install bootimage.

```shell
❯ cargo install bootimage
```

And then build the project.

```shell
❯ cargo build
```

## Run Instructions

To run the disk image through [QEMU](https://www.qemu.org/), install QEMU, and then run,

```shell
❯ cargo run
```

To boot it into a real machine, connect an usb drive, and use,

```shell
❯ dd if=target/x86_64-dv_os/debug/bootimage-dv_os.bin of=/dev/sdX && sync
```

Where `sdX` is location at which the usb drive is located. All the content on this usb drive is overriten.

## Running Tests

To run unit and integrations tests,

```shell
❯ cargo test
```
