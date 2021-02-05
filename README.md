# keyremapper-rs

A Rust library to help enable AHK like flexible key remapping.

## What it is and what it provides

At a high level, it allows to "steal" input events from specific input devices
(keyboards, pointing devices, etc) using evdev, modify with your rust code and inject using
`/dev/uinput`, which allows to create a versatile mapping.

- Unlike AHK, KeyRemapper is a library. Your "script" is a full rust program.

- Unlike AHK, KeyRemapper can distinguish different input devices, and allows you to handle different
  devices differently.

- This library provides a function to get the active window information. Use this to use different mappings
  for different apps.

## Prerequisite

### 1. Gain access to `/devn/input/*` and `/dev/uinput`

```sh
# Add self to the input and uinput groups
sudo usermod -aG input $USER 
sudo groupadd uinput
sudo usermod -aG uinput $USER

# See https://github.com/chrippa/ds4drv/issues/93
echo 'KERNEL=="uinput", SUBSYSTEM=="misc", MODE="0660", GROUP="uinput"' | sudo tee /etc/udev/rules.d/90-uinput.rules

# This seems to be needed because uinput isn't compiled as a loadable module these days.
# See https://github.com/chrippa/ds4drv/issues/93#issuecomment-265300511
echo uinput | sudo tee /etc/modules-load.d/uinput.conf
```

Then reboot.

See also:
- https://stackoverflow.com/questions/11939255/writing-to-dev-uinput-on-ubuntu-12-04.
- https://github.com/chrippa/ds4drv/issues/93#issuecomment-265300511

### 2. Install dependeencies

- `sudo apt install -y libappindicator3-dev libgtk-3-dev libevdev-dev libudev-dev`

## Samples
 
Note: all the following samples will _remap only certain kinds of keyboards_ specified
by the regex `DEVICE_RE` in them. In order to use them with your input devices, use the `--match-device-name` option and provide a regex that
matches your input device. You can find the device name and the vendor/product ID of your device using the `evsniff` sample.

If you need to disdistinguish different devices with the same name,
provide a regex mathing the vendor/product ID with the `--match-id` option.

- Install all the samples: run `./install-examples.sh`

- [keyboard-remapper](examples/keyboard-remapper/main.rs)
  - For the following 3 keyboards:
    - The Thinkpad Internal keyboard (at least for X1 carbon gen7 and P1 gen2)
    - Topre Realforce
    - https://www.amazon.com/gp/product/B00EZ4A2OQ
  - Adds various shortcuts using `ESC`.
  
  - Creates an extra uinput device to inject mouse wheel events.
    e.g. `ESC` + `H`, `J`, `K` and `L` for virtucal and horizontal scroll.
  
  - App specific remap -- on Chrome, use `F5`/`F6` as `BACK`/`FORWARD`. This can be
    done using `keyremapper::ui::WindowInfo::from_active_window()`.

- [shortcut-remote-remapper](examples/shortcut-remote-remapper/main.rs) for https://www.amazon.com/gp/product/B01NC2LEYP

- [trackpoint-speedup](examples/trackpoint-speedup/main.rs) Speed up Thinkpad trackpoint.
   I can never figure out how to easily do it.

- [evsniff](examples/evsniff/main.rs) Kind of like `evtest(1)` but reads from all the devices at once.
  Use this to figure out the device name and its vendor/product IDs.

## TODOs

- Better error handling (chain, stacktrace, etc?)
- Better APIs.

## Links

- evdev
  - https://www.kernel.org/doc/html/latest/input/input.html
- libevdev:
  - Top page: https://www.freedesktop.org/wiki/Software/libevdev/
  - API: https://www.freedesktop.org/software/libevdev/doc/latest/modules.html
  - Doc: https://www.freedesktop.org/software/libevdev/doc/latest/
  - Source code: https://gitlab.freedesktop.org/libevdev/libevdev
  - About SYN_DROPPED: https://www.freedesktop.org/software/libevdev/doc/1.1/syn_dropped.html
- libudev
  - Reference manual: http://presbrey.scripts.mit.edu/doc/libudev/

- libwnck
  - Reference manual: https://developer.gnome.org/libwnck/stable/

- gresource: https://developer.gnome.org/gio/stable/GResource.html
