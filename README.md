# KeyRemapper in Rust

## Prerequisite

### Gain access to `/devn/input/*` and `/dev/uinput`

```sh
# Add self to the input and uinput groups
sudo usermod -aG input $USER 
sudo groupadd uinput
sudo usermod -aG uinput $USER

# See https://github.com/chrippa/ds4drv/issues/93
echo 'KERNEL=="uinput", SUBSYSTEM=="misc", MODE="0660", GROUP="uinput"' | sudo tee /etc/udev/rules.d/90-uinput.rules

# This seems to be needed because uinput isn't compiled as a loadable module these days.
echo uinput | sudo tee /etc/modules-load.d/uinput.conf
```

Then reboot.

See also:
- https://stackoverflow.com/questions/11939255/writing-to-dev-uinput-on-ubuntu-12-04.
- https://github.com/chrippa/ds4drv/issues/93#issuecomment-265300511

### Install dependeencies

- `sudo apt install -y libappindicator3-dev libgtk-3-dev libevdev-dev libudev-dev libwnck-3-dev`


## TODOs

- Better error handling (chain, stacktrace, etc?)
- Handle libevdev_read_status_LIBEVDEV_READ_STATUS_SYNC properly.

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


## Appendix A: runni code on the main thread

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