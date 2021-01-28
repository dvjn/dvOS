# dvOS

Learning what goes into making an operating system 🤯
<br />
Also, I just wanted to practice rust 🤷🏻

## How To

<details>
<summary><b>📦 Build</b></summary>

- **Install rust nightly**

  ```shell
  $ rustup update nightly --force
  ```

- **Set nightly as default**

  ```shell
  $ rustup override set nightly
  ```

- **Install bootimage**

  ```shell
  $ cargo install bootimage
  ```

- **Build the project**

  ```shell
  $ cargo build
  ```

</details>

<details>
<summary><b>💻 Run</b></summary>

- **Run on [QEMU](https://www.qemu.org/)**

  ```shell
  $ cargo run
  ```

- **Run on Real Machine**

  Build the project

  ```shell
  $ cargo build
  ```

  Connect an USB drive, and run

  ```shell
  $ dd if=target/x86_64-dv_os/release/bootimage-dv_os.bin of=/dev/sdX && sync
  ```

  Replace `sdX` with the location of the usb drive.
  Use this usb as bootable drive in the machine.

  **Note:** All the data in the usb drive will be overwritten.

</details>

<details>
<summary><b>✅ Test</b></summary>

- **Run tests**

  ```shell
  $ cargo test
  ```

</details>

## References

Source of Learning: [Writing an OS in Rust](https://os.phil-opp.com/)
