use std::fs::File;

pub fn read_file(path: &str) -> futures::io::Result<File> {
    File::open(path.to_owned())
}

pub fn create_file(path: &str) -> futures::io::Result<File> {
    File::create(path)
}

