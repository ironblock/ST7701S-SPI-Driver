# Sitronix ST7701S SPI/RGB Display Driver

**⚠️ This is a work in progress**

This is a basic display driver for displays based on the Sitronix ST7701S, using SPI for command and control and parallel RGB for video data. It currently targets ARM single-board computers, but may be able to be compiled for other architectures.

## Development Host Setup

Install the following software:

- [Vagrant](https://www.vagrantup.com/downloads.html)
- [Virtualbox 6.0](https://www.virtualbox.org/wiki/Download_Old_Builds_6_0) (6.1 is incompatible with Vagrant as of late 2019)

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

Because the driver is made to target an ARM device, it must be cross-compiled. The most reliable way to do this is in an Ubuntu guest machine which will share the project directory.

To start:

```sh
$ vagrant up && vagrant rsync && vagrant ssh
$ cd /vagrant/st7701s/ && cargo build --target=armv7-unknown-linux-gnueabihf && scp target/armv7-unknown-linux-gnueabihf/debug/st7701s debian@beaglebone.local:~
```

Remember, you can run `vagrant rsync-auto` in another shell to continually sync changes for "live" development.
