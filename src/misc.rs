use std::path::Path;

pub async fn read_file(path: impl AsRef<Path>) -> Result<Vec<u8>, macroquad::Error> {
  let result = macroquad::file::load_file(path.as_ref().as_os_str().to_str().unwrap_or("")).await;

  result
}
