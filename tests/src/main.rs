use std::{ fs::read_to_string, path::{ Path, PathBuf } };

use backyard_parser::parse;
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
  for file in php_files {
    match read_to_string(&file) {
      Ok(content) => {
        let parsed = parse(&content);
        if parsed.is_err() {
          eprintln!("\nError parsing file {}: {}", file.display(), parsed.unwrap_err());
        }
      }
      Err(err) => {
        eprintln!("\nError reading file {}: {}", file.display(), err);
      }
    }
  }
}

// // property hook only, single item
// public bool $virtualHook { &get => true; set($i) => $value; }
// public string $fullName { get { return $this->firstName.' '.$this->lastName; } }

// // support type on const, make const item
// const string FOO = 'foo';

// // globalize argument and parameter
