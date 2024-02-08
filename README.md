This is a test.

It will draw 2 sprites to begin with. You can select a sprite using LEFT and RIGHT d-pad and change its image with UP and DOWN d-pad. To add additional sprites, press RIGHT many times. To check the currently selected sprite, press L. To halt the selected sprite's automatic movement, press X. To move the selected sprite manually, hold L and use the d-pad. To reset the selected sprite to upper left, press A. To delete the selected sprite, press B.

This example uses some mildly fancy features to get performance:

- PNGs are decoded in build.rs and embedded into the executable.
- Drawing is double-buffered.
- Only rectangles which have changed are updated frame to frame.

In my testing, I can get about 13 sprites before I start missing frame deadlines (as measured by speed-debug, see [run.txt](run.txt)). I think I can bump that up around 2x with some optimizations.

# Usage

See [run.txt](run.txt)

# Getting Started

To get started with openfpga-litex, make sure to notice the [README](external/openfpga-litex), [control.md](external/openfpga-litex/docs/control.md), and the [existing Rust examples](external/openfpga-litex/lang/rust/examples) in openfpga-litex; and the "build docs" command in [run.txt](run.txt) (most useful for the litex-pac and litex-openfpga crate docs, since litex-pac contains the Rust version of control.md).

Once you have built a `rust.bin`, you have two options for deployment: You can live upload to a running copy of the Pocket RISC-V core as described in [run.txt](run.txt), or you can create a new copy of the Pocket RISC-V core as described in the [Analogue docs](https://www.analogue.co/developer/docs/packaging-a-core) and include rust.bin as `boot.bin` in the `/Assets/.../common` directory.

# License

The Rust code in this directory is written by Andi McClure <<andi.m.mcclure@gmail.com>>. The images are by Miguel Sternberg. You are granted no rights to reuse any of this currently.