use std::{ fs::read_to_string, path::{ Path, PathBuf } };

use backyard_parser::{ error::ParserError, parse };
use walkdir::WalkDir;

fn list_php_files(directory: &Path) -> Vec<PathBuf> {
  WalkDir::new(directory)
    .into_iter()
    .filter_map(|entry| entry.ok())
    .filter(|entry| entry.path().is_file())
    .filter(|entry|
      entry
        .path()
        .extension()
        .map_or(false, |ext| ext == "php")
    )
    .map(|entry| entry.into_path())
    .collect()
}

fn main() {
  let directory = Path::new("./target/samples");
  let php_files = list_php_files(directory);
  println!("Found {} PHP files", php_files.len());
  for file in php_files {
    match read_to_string(&file) {
      Ok(content) => {
        let parsed = parse(&content);
        if let Err(err) = parsed {
          if let ParserError::Eof = err {
          } else {
            eprintln!("\nError parsing file {}: {}", file.display(), err);
          }
        }
      }
      Err(err) => {
        eprintln!("\nError reading file {}: {}", file.display(), err);
      }
    }
  }
}

// // globalize argument and parameter

// // match_pattern should skip unexpected comments

// unwrap, clone, to_string, Arc?
