use std::fs::OpenOptions;
use std::path::Path;
use std::io::Write;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    OpenOptions::new()
                .append(true) //Autorise append content bytes
                .create(true) //Autorise la creation d fichier si open n trouve pas , car default il uau un panic du unwrap du open 
                .open(path)
                .unwrap() 
                .write_all(content.as_bytes()).unwrap();
}
