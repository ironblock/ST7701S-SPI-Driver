# Sitronix ST7701S SPI/RGB Display Driver

**⚠️ This is a work in progress**

This is a basic display driver for displays based on the Sitronix ST7701S, using SPI for command and control and parallel RGB for video data. It currently targets ARM single-board computers, but may be able to be compiled for other architectures.

## Development Host Setup

Install [rust](https://rustup.rs).
Install [`cross`](https://github.com/cross-rs/cross) if your development host and target have different operating systems or architectures.

## Development Target Setup

You will need a dedicated single-board computer to target:

- Beaglebone Black ([Setup Docs](./platforms/beaglebone))
- Raspberry Pi (Untested)

### Software

The following programs are required on all development targets:

- `cpp`: Used to build the Device Tree Overlay
- `dtc`: Used to build the Device Tree Overlay

These are optional, but helpful:

- `fbset`: Show and set the mode and timings of the current framebuffer
- `fim`: Manually render an image to the framebuffer

### Device Tree Overlay

The device tree overlay will need to be built and selected in uBoot (or similar) for your particular target platform. See specific platform docs for more.

Regardless of the platform-specific configuration, these settings will be correct for the timing in any video mode:

```dts
  hsync-active    = <0>;
  vsync-active    = <0>;
  de-active       = <1>;
  pixelclk-active = <1>;
```

## Building the Driver

Run `cargo build --target armv7-unknown-linux-gnueabihf`, providing your own target. Ensure that any required linker is available and specified in `.cargo/config.toml` under the appropriate section for the given target.

Depending on your development host OS and architecture, it may be challenging to install the correct linker (or a compatible alternative). In these cases, it may be better to use [`cross`](https://github.com/cross-rs/cross) instead. In that case, the command would instead become `cross build --target armv7-unknown-linux-gnueabihf`.
