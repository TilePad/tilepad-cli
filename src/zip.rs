use std::{
    fs::File,
    io::{Read, Seek, Write},
    path::Path,
};

use walkdir::WalkDir;
use zip::write::SimpleFileOptions;

pub fn zip_directory<T>(input: &Path, writer: T) -> eyre::Result<()>
where
    T: Write + Seek,
{
    let walkdir = WalkDir::new(input);
    let it = walkdir.into_iter();
    let mut zip = zip::ZipWriter::new(writer);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    let mut buffer = Vec::new();
    for entry in it {
        let entry = match entry {
            Ok(value) => value,
            Err(_) => continue,
        };

        let path = entry.path();
        let relative_path = path.strip_prefix(input)?;

        if path.is_file() {
            zip.start_file_from_path(relative_path, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            buffer.clear();
        } else if !relative_path.as_os_str().is_empty() {
            zip.add_directory_from_path(relative_path, options)?;
        }
    }
    zip.finish()?;
    Ok(())
}
