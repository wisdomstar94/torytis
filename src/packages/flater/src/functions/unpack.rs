use std::{fs::File, path::PathBuf};
use flate2::read::GzDecoder;
use tar::Archive;

pub fn unpack_tar_gz(packed_file_path: &PathBuf, unpack_target_folder: &PathBuf) -> Result<(), std::io::Error> {
  let tar_gz = File::open(packed_file_path)?;
  let tar = GzDecoder::new(tar_gz);
  let mut archive = Archive::new(tar);
  archive.unpack(unpack_target_folder)?;
  Ok(())
}