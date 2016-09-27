use file_tools;
use image_hash;
use std::io;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  FileTool(file_tools::errs::Error),
  ImageHash(image_hash::errs::Error),
  Io(io::Error),
}

impl From<file_tools::errs::Error> for Error {
  fn from(err: file_tools::errs::Error) -> Error {
    Error::FileTool(err)
  }
}

impl From<image_hash::errs::Error> for Error {
  fn from(err: image_hash::errs::Error) -> Error {
    Error::ImageHash(err)
  }
}

impl From<io::Error> for Error {
  fn from(err: io::Error) -> Error {
    Error::Io(err)
  }
}
