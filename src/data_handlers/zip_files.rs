use std::{fs::File, path::PathBuf};

use log::{debug, info};
use zip::{read::ZipArchive, result::ZipResult};

pub fn extract_archive(data_path: PathBuf, archive_path: PathBuf) -> ZipResult<PathBuf> {
    info!("Extracting archive: {:?}", archive_path);
    let file = File::open(archive_path.clone()).unwrap();
    let mut archive = ZipArchive::new(file)?;
    archive.extract(data_path)?;
    let file = archive.by_index(0)?;
    let name = match file.enclosed_name() {
        Some(path) => path.to_owned(),
        None => {
            return Err(zip::result::ZipError::InvalidArchive(
                "Archive contains no files",
            ))
        }
    };
    debug!("Extracting archive: {:?} completed!", archive_path);
    Ok(name.to_path_buf())
}

#[cfg(test)]
mod tests {
    use zip::ZipWriter;

    use super::*;
    use std::{
        fs::{remove_file, File},
        io::Write,
        path::PathBuf,
    };

    #[test]
    fn test_extract_archive() {
        let file_path = PathBuf::from("test.zip");
        let file = File::create(&file_path).unwrap();
        let mut archive = ZipWriter::new(file);
        archive.start_file("test.csv", Default::default()).unwrap();
        archive.write_all("test".as_bytes()).unwrap();
        archive.finish().unwrap();
        let data_path = PathBuf::from("./");
        let file_path = extract_archive(data_path, file_path).unwrap();
        assert_eq!(&file_path, &file_path.with_extension("csv"));
        remove_file(file_path).unwrap();
    }
}
