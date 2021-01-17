use nix::fcntl;
use std::os::unix::io::AsRawFd;
use std::{fs::File, path::PathBuf};

pub fn ensure_singleton(unique_name: &str) {
    if unique_name.is_empty() {
        panic!("unique_name must not be empty");
    }
    let filename = {
        let mut f = unique_name.to_string();
        f.push_str(".lockfile");
        f
    };
    let mut path = PathBuf::new();
    path.push("/tmp");
    path.push(filename);

    log::debug!("Lockfile={}", path.to_str().unwrap());

    unsafe {
        let old_mask = libc::umask(0o000);

        let success = match File::create(&path) {
            Err(_) => false,
            Ok(file) => {
                let fd = file.as_raw_fd();
                Box::leak(Box::new(file)); // Leak the file to avoid closing it.

                match fcntl::flock(fd, fcntl::FlockArg::LockExclusiveNonblock) {
                    Ok(_) => true,
                    Err(_) => false,
                }
            }
        };
        if !success {
            eprintln!("Unable to lock lockfile {}. Another process running?", path.as_os_str().to_str().unwrap());
            std::process::exit(1);
        }
        libc::umask(old_mask);
    }
}
