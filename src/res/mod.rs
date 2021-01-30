use gio::{prelude::*, NONE_CANCELLABLE};
use parking_lot::Mutex;
use std::{cell::RefCell, io::prelude::*, sync::Arc};
use std::{
    error::Error,
    fs::{self, File},
    path::PathBuf,
    time::SystemTime,
};

type ResourcesFetcher = dyn Fn() -> gio::Resource;

fn copy_stream(ins: &gio::InputStream, outs: &mut File) -> Result<(), Box<dyn Error>> {
    let mut buf = [0_u8; 4 * 1024];
    let mut total_size = 0;
    loop {
        let read_size = ins.read(&mut buf, NONE_CANCELLABLE)?;
        if read_size == 0 {
            log::debug!("Wrote {} bytes to {:?}", total_size, outs);
            return Ok(());
        }

        let mut pos = 0;
        while pos < read_size {
            pos += outs.write(&buf[pos..read_size])?;
        }
        total_size += read_size;
    }
}

fn extract_resources(file_path: &PathBuf, res_path: &str, resource: &gio::Resource) -> () {
    fs::create_dir_all(file_path.parent().unwrap()).expect("Unable to make directories");
    let ins = resource.open_stream(res_path, gio::ResourceLookupFlags::NONE).expect("Unable to load resources");
    let mut outs = File::create(file_path).expect("Unable to create file");

    match copy_stream(&ins, &mut outs) {
        Ok(_) => return,
        Err(e) => {
            #[allow(unused_must_use)]
            fs::remove_file(file_path);
            panic!("Unable to write resouce {} to {:?}: {}", res_path, file_path, e)
        }
    };
}

fn get_gio_resource_as_file(exe_unique_name: &str, res_path: &str, resource_fetcher: &ResourcesFetcher) -> PathBuf {
    let file_path = {
        let mut dir = std::env::temp_dir();
        dir.push(exe_unique_name);
        dir.push(&res_path[1..]); // Remove the leading '/'.
        dir
    };

    let oldest = SystemTime::UNIX_EPOCH;
    let exe_mtime = fs::metadata(std::env::current_exe().unwrap()).unwrap().modified().unwrap_or(oldest);
    let file_mtile = match fs::metadata(&file_path) {
        Ok(stat) => stat.modified().unwrap_or(oldest),
        Err(_) => oldest,
    };
    if exe_mtime > file_mtile {
        log::debug!("Writing a resouce {} into a file {:?}", res_path, file_path);
        extract_resources(&file_path, res_path, &resource_fetcher());
    } else {
        log::debug!("File {:?} is up-to-date", file_path);
    }
    return file_path;
}

#[derive(Debug, Clone)]
pub struct ResourceIcon {
    path: PathBuf,
}

impl ResourceIcon {
    pub fn from_bytes(exe_unique_name: &str, res_path: &str, resource_bytes: &'static [u8]) -> ResourceIcon {
        let bytes = resource_bytes.clone();
        return ResourceIcon {
            path: get_gio_resource_as_file(&exe_unique_name, &res_path, &move || {
                let data = glib::Bytes::from(bytes);
                return gio::Resource::from_data(&data).unwrap();
            }),
        };
    }

    pub fn get_path(&self) -> PathBuf {
        return self.path.clone();
    }
}

impl Into<PathBuf> for ResourceIcon {
    fn into(self) -> PathBuf {
        return self.path;
    }
}

pub fn get_default_icon() -> ResourceIcon {
    return ResourceIcon::from_bytes("keyremapper-rs.default.res", "/keyremapper/resources/keyboard.png", include_bytes!("icons.bin"));
}

#[derive(Debug)]
struct LazyResourceInner {
    exe_unique_name: String,
    bytes: &'static [u8],
    resource: Option<gio::Resource>,
}

impl LazyResourceInner {
    fn get_resource(&mut self) -> &gio::Resource {
        if self.resource.is_none() {
            let data = glib::Bytes::from(self.bytes);
            self.resource = Some(gio::Resource::from_data(&data).unwrap());
        }
        return self.resource.as_ref().unwrap();
    }
}

#[derive(Debug)]
pub struct LazyResource {
    inner: Arc<Mutex<RefCell<LazyResourceInner>>>,
}

impl LazyResource {
    pub fn from_bytes(exe_unique_name: &str, bytes: &'static [u8]) -> LazyResource {
        return LazyResource {
            inner: Arc::new(Mutex::new(RefCell::new(LazyResourceInner {
                exe_unique_name: exe_unique_name.to_string(),
                bytes,
                resource: None,
            }))),
        };
    }

    pub fn get_icon(&self, resource_path: &str) -> PathBuf {
        let lock = self.inner.lock();
        let mut inner = lock.borrow_mut();

        if !resource_path.starts_with("/") {
            panic!("resource_path must starts with a '/'.");
        }

        let file_path = {
            let mut dir = std::env::temp_dir();
            dir.push(&inner.exe_unique_name);
            dir.push(&resource_path[1..]); // Remove the leading '/'.
            dir
        };
    
        let epoch = SystemTime::UNIX_EPOCH;
        let exe_mtime = fs::metadata(std::env::current_exe().unwrap()).unwrap().modified().unwrap_or(epoch);
        let file_mtile = match fs::metadata(&file_path) {
            Ok(stat) => stat.modified().unwrap_or(epoch),
            Err(_) => epoch,
        };
        if exe_mtime > file_mtile {
            log::debug!("Writing a resource {} into a file {:?}", resource_path, file_path);
            extract_resources(&file_path, resource_path, inner.get_resource());
        } else {
            log::debug!("File {:?} is up-to-date", file_path);
        }
        return file_path;
    }
}