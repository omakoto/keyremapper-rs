use gio::{prelude::*, NONE_CANCELLABLE};
use std::io::prelude::*;
use std::{
    error::Error,
    fs::{self, File},
    path::PathBuf,
    time::SystemTime,
};

pub(crate) const DEFAULT_ICON_NAME: &str = "/keyremapper/resources/keyboard.png";

pub(crate) fn load_gio_resources() -> gio::Resource {
    let data = glib::Bytes::from(include_bytes!("icons.bin"));
    return gio::Resource::from_data(&data).unwrap();
}

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

fn extract_resources(file_path: &PathBuf, res_path: &str, resource_fetcher: &ResourcesFetcher) -> () {
    fs::create_dir_all(file_path.parent().unwrap()).expect("Unable to make directories");
    let res = resource_fetcher();
    let ins = res.open_stream(res_path, gio::ResourceLookupFlags::NONE).expect("Unable to load resources");
    let mut outs = File::create(file_path).expect("Unable to create file");

    match copy_stream(&ins, &mut outs) {
        Ok(_) => return,
        Err(e) => {
            #[allow(unused_must_use)]
            fs::remove_file(file_path);
            panic!("Unable to write resouce {} to {:?}", res_path, file_path)
        }
    };
}

pub fn get_gio_resource_as_file(exe_unique_name: &str, res_path: &str, resource_fetcher: &ResourcesFetcher) -> PathBuf {
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
        extract_resources(&file_path, res_path, resource_fetcher);
    } else {
        log::debug!("File {:?} is up-to-date", file_path);
    }
    return file_path;
}
