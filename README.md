A learning project of mine using the [Microbit V2](https://microbit.org/buy/bbc-microbit-go/) with Rust. Most of this setup is from this [excellent video](https://www.youtube.com/watch?v=TOAynddiu5M), alongside a Nix flake.

## Usage

To develop for this board, run `nix develop` to enter a pre-configured dev shell. From there, plug your microbit into a USB port and run `cargo embed` to compile and flash the program.

To get the rust-analyzer LSP working correctly, make sure that you set the ["rust-analyzer.cargo.allTargets"](https://rust-analyzer.github.io/book/configuration.html#cargo.allTargets) option to false. Otherwise, you will get errors for your host system.

### NixOS Impermanence

If you are using NixOS with impermanence, you may experience issues when clean building this project. This issue occurs with FUSE filesystems, and can be fixed by ensuring that this project is persisted with a symlink. An example is shown below

```nix
home.persistence."/persist/<user>".directories = [
  {
    directory = "<projects dir>";
    method = "symlink";
  }
];
```

Rebuild your NixOS after changing this, run `cargo clean` and then `cargo check`.

## Resources

Some useful resources when working with the Microbit V2

- [Schematics](https://tech.microbit.org/hardware/schematic/)
- [Datasheet](https://raspberrypi.dk/wp-content/uploads/2020/10/BBC-microbit-v2-datasheet-v1.2.pdf)
