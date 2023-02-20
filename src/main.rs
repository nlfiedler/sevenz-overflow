use sevenz_rust::{SevenZArchiveEntry, SevenZWriter};
use std::fs::File;
use std::path::PathBuf;

fn main() {
    let source = PathBuf::from("./C++98-tutorial.pdf");
    let handle = File::open(source).expect("open file");
    let dest = PathBuf::from("./compressed.7z");
    let file = File::create(dest).expect("create file");
    let mut builder = SevenZWriter::new(file).expect("create writer");
    let mut entry: SevenZArchiveEntry = Default::default();
    entry.name = "corrupted.pdf".into();
    entry.has_stream = true;
    entry.is_directory = false;
    let result = builder
        .push_archive_entry(entry, Some(handle))
        .expect("push archive");
    println!("wrote {:?}", result);
    builder.finish().expect("finish builder");
}
