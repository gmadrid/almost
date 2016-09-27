extern crate file_tools;
extern crate image_hash;

mod almost_errs;

use almost_errs::Result;
use file_tools::crawler::{Crawler, HandlesCrawl};
use image_hash::cgimage::CGImage;
use std::collections::HashMap;
use std::fs::{DirEntry, File};
use std::path::Path;

struct FileInfo {
  path: String,
}

struct AlmostCrawler {
  map: HashMap<String, Vec<FileInfo>>,
}

impl HandlesCrawl for AlmostCrawler {
  fn process_entry(&self, entry: &DirEntry) -> file_tools::errs::Result<()> {
    let path = entry.path();

    if let Some(extension) = path.extension().map(|p| p.to_string_lossy()) {
      match &extension as &str {
        "jpg" | "jpeg" => {
          println!("ABOUT TO OPEN: {:?}", path);
          let f = try!(File::open(path.to_owned()));
          println!("OPENED");
          let image = try!(CGImage::read_jpg(f).map_err(|e| {
            println!("IN HERE, but probably not");
            file_tools::errs::Error::GenericError("Cannot open jpeg file".to_string())
          }));
          println!("ahash: {:?}", image.ahash());
          ()
        }
        _ => {
          println!("IN HREE");
        }
      }
    }

    println!("AFTER{:?}", entry.file_name());
    Ok(())
  }

  fn filter_entry(&self, entry: &DirEntry) -> file_tools::errs::Result<bool> {
    let path = entry.path();

    if let Ok(metadata) = entry.metadata() {
      if metadata.len() < 1 {
        return Ok(false);
      }
    } else {
      return Ok(false);
    }

    if path.file_name().map_or(true, |fnm| fnm == ".DS_Store") {
      return Ok(false);
    }

    if path.extension().map_or(true, |ext| ext != "jpg" && ext != "jpeg" && ext != "png") {
      return Ok(false);
    }

    Ok(true)
  }
}

impl AlmostCrawler {
  fn new() -> AlmostCrawler {
    AlmostCrawler { map: HashMap::new() }
  }
}

fn real_main() -> Result<()> {
  // let file = try!(File::open("/Users/gmadrid/Dropbox/Media/Porn/Sorted/Other/Zebra.jpg"
  //   .to_owned()));
  // let image = try!(CGImage::read_jpg(file));
  // println!("{:?}", image.ahash());

  let crawl_handler = AlmostCrawler::new();
  let mut crawler = Crawler::new(crawl_handler);
  try!(crawler.add_path(Path::new("/Users/gmadrid/Dropbox/Media/Testing")));
  Ok(())
}

fn main() {
  match real_main() {
    Ok(_) => (),
    // TODO: implement Display for err
    Err(err) => println!("{:?}", err),
  }
}
