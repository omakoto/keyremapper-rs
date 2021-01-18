# keyremapper-rs

A Rust library to help enable AHK like flexible key remapping.

## What it is and what it provides

At a high level, it allows to "steal" input events from specific input devices
(keyboards, pointing devices, etc) using evdev, modify with your rust code and inject using
`/dev/uinput`, which allows to create a versatile mapping.

- Unlike AHK, KeyRemapper is a library. Your "script" is a full rust program.

- Unlike AHK, KeyRemapper can distinguish different input devices, and allows you to handle different
  devices differently.

- Unlike AHK, KeyRemapper doesn't provide any features to allow to use different mappings for
  different apps, or to control windows, etc. You need to write code to do such things.

  (See also later section for creating different mappings for different apps.)
  
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

- `sudo apt install -y libappindicator3-dev libgtk-3-dev libevdev-dev libudev-dev libwnck-3-dev`


## Samples
 
Note: all the following samples will _remap only certain kinds of keyboards_ specified
by the regex `DEVICE_RE` in them. In order to use them with your input devices, use the `--match-device-name` option and provide a regex that
matches your input device. Find the device name using `evtest`.

If you need to disdistinguish different devices with the same name,
provide a regex mathing the vendor/product ID with the `--match-id` option.

- [keyboard-remapper.py](blob/main/examples/keyboard-remapper/main.rs)
  - For the following 3 keyboards:
    - The Thinkpad Internal keyboard (at least for X1 carbon gen7 and P1 gen2)
    - Topre Realforce
    - https://www.amazon.com/gp/product/B00EZ4A2OQ
  - Adds various shortcuts using `ESC`.
  - Creates an extra uinput device to inject mouse wheel events.
    e.g. `ESC` + `H`, `J`, `K` and `L` for virtucal and horizontal scroll.

- [shortcut-remote-remapper](blob/main/examples/shortcut-remote-remapper/main.rs) for https://www.amazon.com/gp/product/B01NC2LEYP

- [trackpoint-speedup](blob/main/examples/trackpoint-speedup/main.rs) Speed up Thinkpad trackpoint.
   I can never figure out how to easily do it.


## Does it support creating different mappings for different apps?

The previous version written in Python (https://github.com/omakoto/key-remapper) supported it using libwnck.

I tried the same thing but faced problems:

- First, using libwnck in a worker thread (the I/O thread) caused the follwoing problem.

```
(keyboard-remapper:444669): Wnck-CRITICAL **: 18:20:10.378: update_client_list: assertion 'reentrancy_guard == 0' failed
[xcb] Unknown sequence number while processing reply
[xcb] Most likely this is a multi-threaded client and XInitThreads has not been called
[xcb] Aborting, sorry about that.
keyboard-remapper: ../../src/xcb_io.c:641: _XReply: Assertion `!xcb_xlib_threads_sequence_lost' failed.
./start-keyboard-remapper.sh: line 7: 444669 Aborted                 (core dumped) RUST_BACKTRACE=1 RUST_LOG=debug cargo run --example $prog_name -- "$@"
```
- Calling `XInitThreads()` in the I/O thread still caused anther problem. (Deadlock or something, forgot.)
- Then I tired https://github.com/psychon/x11rb, following https://www.reddit.com/r/rust/comments/f7yrle/get_information_about_current_window_xorg/ (the branch: https://github.com/omakoto/keyremapper-rs/blob/wnck-bg-thread/src/ui/mod.rs), which did work, if I called `XInitThreads()` in the worker thread.
- However, calling `XInitThreads()` in a non-mainthread appears to break
libappindicator -- if I click the task tray icon, the process seg faults.

- The python version did the I/O on the mainthread, using https://developer.gnome.org/pygobject/stable/glib-functions.html#function-glib--io-add-watch. In order to do this, I need to integrate the device polling into the gtk main loop, which however doesn't seem to be supported with the current versin of gtk-rs. The relevant APIs (e.g. `g_source_add_unix_fd()` -- see https://developer.gnome.org/glib/stable/glib-The-Main-Event-Loop.html) all seems to be left unimplemented with TODOs.

Getting the current window information doesn't seem to be supported by wayland by design for "security" anyway, so I've decided not to do it.


## TODOs

- Don't ignore udev device creation events
- If not grabing, allow to read from other key remapper uinputs.

- Better error handling (chain, stacktrace, etc?)
- Handle libevdev_read_status_LIBEVDEV_READ_STATUS_SYNC properly.
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

## Appendix A: runnig code on the main thread

From https://coaxion.net/blog/2019/02/mpsc-channel-api-for-painless-usage-of-threads-with-gtk-in-rust/

```rust
let label = gtk::Label::new("not finished");
[...]
// We wrap the label clone in the Fragile type here
// and move that into the new thread instead.
let label_clone = fragile::Fragile::new(label.clone());
thread::spawn(move || {
    // Let's sleep for 10s
    thread::sleep(time::Duration::from_secs(10));

    // Defer the label update to the main thread.
    // For this we get the default main context,
    // the one used by GTK on the main thread,
    // and use invoke() on it. The closure also
    // takes ownership of the label_clone and drops
    // it at the end. From the correct thread!
    glib::MainContext::default().invoke(move || {
        label_clone.get().set_text("finished");
    });
});
```