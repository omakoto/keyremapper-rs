use gio::{prelude::*, NONE_CANCELLABLE};
use std::io::prelude::*;
use std::{
    error::Error,
    fs::{self, File},
    path::PathBuf,
    time::SystemTime,
};

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
    let ins = resource
        .open_stream(res_path, gio::ResourceLookupFlags::NONE)
        .expect("Unable to load resources");
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

pub fn get_default_icon() -> PathBuf {
    return Resources::from_bytes("keyremapper-rs.default.res", include_bytes!("icons.bin")).get_icon("/keyremapper/resources/keyboard.png");
}

#[derive(Debug)]
pub struct Resources {
    exe_unique_name: String,
    bytes: &'static [u8],
    resource: Option<gio::Resource>,
}

impl Resources {
    pub fn from_bytes(exe_unique_name: &str, bytes: &'static [u8]) -> Resources {
        return Resources {
            exe_unique_name: exe_unique_name.to_string(),
            bytes,
            resource: None,
        };
    }

    fn get_resource(&mut self) -> &gio::Resource {
        if self.resource.is_none() {
            let data = glib::Bytes::from(self.bytes);
            self.resource = Some(gio::Resource::from_data(&data).unwrap());
        }
        return self.resource.as_ref().unwrap();
    }

    pub fn get_icon(&mut self, resource_path: &str) -> PathBuf {
        if !resource_path.starts_with("/") {
            panic!("resource_path must starts with a '/'.");
        }

        let file_path = {
            let mut dir = std::env::temp_dir();
            dir.push(&self.exe_unique_name);
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
            extract_resources(&file_path, resource_path, self.get_resource());
        } else {
            log::debug!("File {:?} is up-to-date", file_path);
        }
        return file_path;
    }
}
