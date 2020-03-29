# BeagleBone Black Setup

The BeagleBone Black was selected as a development board because it uses a variant of the Texas Instruments AM335X, a well-documented and inexpensive platform.

## First Time Setup

The best available setup documentation is from the [Beaglebone website](http://beagleboard.org/getting-started).

If you've used the "IoT" image, or something else that has no Xorg, this will install a minimal X setup:

**Note: You might not need (or want) Xorg.**

```
sudo apt-get install --no-install-recommends -y xserver-xorg-core xserver-xorg-input-all xserver-xorg-video-fbdev
```

### Physically Conneting the Display

Refer to the [pinmux docs](./docs/Pinmux.md)

### Connecting to Device

Once the OS is installed, there are three methods which can be used to communicate with the BeagleBone Black:

- **Ethernet**:
  Preferred method. Connect to a host ethernet port and share an internet connection, or to a switch or hub that can easily let you determine the DHCP assigned IP address of the BBB.
- **Serial cable**:
  Will let you watch boot. Use the PL2303 cable as [described here](https://elinux.org/Beagleboard:BeagleBone_Black_Serial#Adafruit_4_Pin_Cable_.28PL2303.29). Connect black to pin 1 (has a dot next to it), green to pin 4, white to pin 5. NOTE: It's a six pin header, the last pin is pin 6. Install the [Prolific drivers](http://www.prolific.com.tw/us/showproduct.aspx?p_id=229&pcid=41). Connect to the board before boot with `sudo screen /dev/tty.usbserial 115200 8n1`.
- **USB**:
  Not recommended. Strange, fragile setup. Sketchy USB drivers. Follow the [Adafruit guide](https://learn.adafruit.com/ssh-to-beaglebone-black-over-usb/overview) at your own risk.

Generally, the BeagleBone will pick an IP address like `192.168.x.2`. The non-privileged user is `debian` and the password is `temppwd`.

## Device Tree Overlay

The display requires a device tree overlay to be created and loaded in order for the Linux kernel to correctly target the video device.

### On host machine

The `.dts` file uses some C headers that must be copied to the BeagleBone in order to compile the overlay:

```sh
scp -r ./beaglebone/dt-bindings debian@beaglebone.local:~/dts/dt-bindings
```

Make changes to `./beaglebone/VE-2IN-BBB.dts`, then copy it over:

```sh
scp ./beaglebone/VE-2IN-BBB.dts debian@beaglebone.local:~/dts
```

### On BeagleBone Black

Build the `.dts` file and reboot to load it:

```sh
cd ~/dts && \
cpp -nostdinc -I . -undef -x assembler-with-cpp VE-2IN-BBB.dts VE-2IN-BBB.dts.preprocessed && \
dtc -O dtb -o VE-2IN-BBB.dtbo -b 0 -@ VE-2IN-BBB.dts.preprocessed && \
sudo cp VE-2IN-BBB.dtbo /lib/firmware/VE-2IN-BBB.dtbo && \
sudo reboot
```

In `/boot/uEnv.txt`, comment out the virtual HDMI cape and the universal cape. Then, add this line:

```
dtb_overlay=/lib/firmware/VE-2IN-BBB.dtbo
```

You may also want to disable the HDMI framer in the kernel command line arguments:

```
cmdline=coherent_pool=1M net.ifnames=0 quiet
```

## Other things

These are \*_not necessary_, but documented here for reference.

### Manually Flash the Reset Pin

The GPIO `P9_12` on the **board** is `GPIO1_28`/`PIN 60` in software.

```sh
sudo echo 60 > /sys/class/gpio/export && sleep 1 && \
sudo echo out > /sys/class/gpio/gpio60/direction && \
echo 1 > /sys/class/gpio/gpio60/value && \
echo 0 > /sys/class/gpio/gpio60/value && sleep 0.5 && \
echo 1 > /sys/class/gpio/gpio60/value
```
