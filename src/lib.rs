//! This crate is a collection of utilities to make performing certain
//! file manipulations more convenient.
//!
//! # Examples
//! ```
//! use file_access::as_file::*;
//!
//! fn main() -> std::io::Result<()> {
//!     Ok({
//!         let text = "Cargo.toml".as_file().read_string()?;
//!         println!("{}", text);
//!
//!         "Cargo.toml".as_file().read_lines()?
//!             .iter()
//!             .for_each(|line| {
//!                 println!("{}", line);
//!             });
//!
//!         "file.1".as_file().write_string(&"Hello, World!")?;
//!
//!         let file = "file.1".as_file();
//!         file.append_lines(&vec!["hello", "world"])?;
//!         file.copy_to(&"file.2")?; // copies ./file.1 to ./file.2
//!
//!         "file.2".as_file().rename_to(&"file.1")?; // replace
//!         "file.1".as_file().delete()?; // clean-up
//!     })
//! }
//! ```

pub use as_file::*; // re-export AsFile
pub use file_path::*; // re-export FilePath
use internal::{traits::to_vec_string::*, types::*};
use std::{
    fs::{self, File, Metadata},
    io::{Error, ErrorKind, Read, Result},
    path::PathBuf,
};

pub mod as_file;
pub mod file_path;
mod internal;

// Gets a File::open handle from AsRef<str> such as String or &str
fn get_file<Path: AsRef<str>>(file_path: &Path) -> Result<File> {
    File::open(file_path.as_ref())
}

// Converts AsRef<str> such as String or &str to PathBuf
fn path_of<Path: AsRef<str>>(file_path: &Path) -> PathBuf {
    PathBuf::from(file_path.as_ref())
}

// Creates a file and its full directory path if they don't exist
fn mk_file<Path: AsRef<str>>(file_path: &Path) -> Result<File> {
    if let Some(path) = path_of(file_path).parent() {
        fs::create_dir_all(path)?;
    }
    return File::create(file_path.as_ref());
}

/// Reads the contents of a file.
///
/// # Returns
/// Result<`String`>
///
/// # Examples
/// ```
/// fn main() -> std::io::Result<()> {
///     Ok({
///         let file_path: &str = "Cargo.toml";
///         let file_path: String = String::from(file_path);
///
///         let text: String = file_access::read_string(&file_path)?;
///         println!("{}", text);
///     })
/// }
/// ```
pub fn read_string<Path: AsRef<str>>(file_path: &Path) -> Result<String> {
    let mut buf = String::new();
    get_file(file_path)?.read_to_string(&mut buf)?;

    return Ok(buf);
}

/// Reads the contents of a file and returns it as lines.
///
/// # Returns
/// Result<`Vec<String>`>
///
/// # Examples
/// ```
/// fn main() -> std::io::Result<()> {
///     Ok({
///         let file_path: &str = "Cargo.toml";
///         let file_path: String = String::from(file_path);
///
///         let lines: Vec<String> = file_access::read_lines(&file_path)?;
///         lines.iter().for_each(|line| println!("{}", line));
///     })
/// }
/// ```
pub fn read_lines<Path: AsRef<str>>(file_path: &Path) -> Result<Lines> {
    Ok(read_string(file_path)?
        .lines()
        .map(ToString::to_string)
        .collect())
}

/// Writes text to a file. This function will create the file **and its full directory path** if they don't exist,
/// and will entirely replace the contents.
///
/// # Parameters
/// - `file_path`: **borrowed** `AsRef<str>` such as `String` or `&str`
/// - `text`: **borrowed** `AsRef<str>` such as `String` or `&str`
///
/// # Returns
/// Result<`()`>
///
/// # Examples
/// ```
/// fn main() -> std::io::Result<()> {
///     Ok({
///         let file_path: &str = "write_to/absolute_or_relative.path";
///         let file_path: String = String::from(file_path);
///
///         let text: &str = "Hello, World!";
///         let text: String = String::from(text);
///
///         file_access::write_string(&file_path, &text)?;
///
///         // Clean-up:
///         file_access::delete(&"write_to")?; // ./write_to/
///     })
/// }
/// ```
pub fn write_string<Path: AsRef<str>, Text: AsRef<str>>(
    file_path: &Path,
    text: &Text,
) -> Result<()> {
    let path = path_of(file_path);
    if !path.exists() {
        mk_file(file_path)?;
    }
    return fs::write(path, text.as_ref());
}

/// Writes a list of text as lines to a file. This function will create the file **and its full directory path** if they don't exist,
/// and will entirely replace the contents with the provided strings each on its own line.
///
/// # Parameters
/// - `file_path`: **borrowed** `AsRef<str>` such as `String` or `&str`
/// - `lines`: **borrowed** `Vec<AsRef<str>>` such as `Vec<String>` or `Vec<&str>`
///
/// # Returns
/// Result<`()`>
///
/// # Examples
/// ```
/// fn main() -> std::io::Result<()> {
///     Ok({
///         let file_path: &str = "lines_to/absolute_or_relative.path";
///         let file_path: String = String::from(file_path);
///
///         let lines: Vec<&str> = "Hello, World!".split_whitespace().collect();
///         let lines: Vec<String> = lines.iter().map(ToString::to_string).collect();
///
///         file_access::write_lines(&file_path, &lines)?;
///
///         // Clean-up:
///         file_access::delete(&"lines_to"); // ./lines_to/
///     })
/// }
/// ```
pub fn write_lines<Path: AsRef<str>, Line: AsRef<str>>(
    file_path: &Path,
    lines: &Vec<Line>,
) -> Result<()> {
    write_string(file_path, &lines.to_vec_string().join("\n"))
}

/// Appends text to a file. This function will append the contents of the file,
/// or write a new one **and its full directory path** if they don't exist yet.
///
/// # Parameters
/// - `file_path`: **borrowed** `AsRef<str>` such as `String` or `&str`
/// - `text`: **borrowed** `AsRef<str>` such as `String` or `&str`
///
/// # Returns
/// Result<`()`>
///
/// # Examples
/// ```
/// fn main() -> std::io::Result<()> {
///     Ok({
///         let file_path: &str = "append_to/absolute_or_relative.path";
///         let file_path: String = String::from(file_path);
///
///         let text: &str = "Hello, World!";
///         let text: String = String::from(text);
///
///         file_access::append_string(&file_path, &text)?;
///
///         // Clean-up:
///         file_access::delete(&"append_to"); // ./append_to/
///     })
/// }
/// ```
pub fn append_string<Path: AsRef<str>, Text: AsRef<str>>(
    file_path: &Path,
    text: &Text,
) -> Result<()> {
    write_string(
        file_path,
        &match read_string(file_path) {
            Ok(file) => format!("{}{}", file, text.as_ref()),
            Err(_) => text.as_ref().to_string(),
        },
    )
}

/// Appends a list of text as lines to a file. This function will append the contents of the file,
/// or write a new one **and its full directory path** if they don't exist yet.
///
/// # Parameters
/// - `file_path`: **borrowed** `AsRef<str>` such as `String` or `&str`
/// - `lines`: **borrowed** `Vec<AsRef<str>>` such as `Vec<String>` or `Vec<&str>`
///
/// # Returns
/// Result<`()`>
///
/// # Examples
/// ```
/// fn main() -> std::io::Result<()> {
///     Ok({
///         let file_path: &str = "append_lines_to/absolute_or_relative.path";
///         let file_path: String = String::from(file_path);
///
///         let lines: Vec<&str> = "Hello, World!".split_whitespace().collect();
///         let lines: Vec<String> = lines.iter().map(ToString::to_string).collect();
///
///         file_access::append_lines(&file_path, &lines)?;
///
///         // Clean-up:
///         file_access::delete(&"append_lines_to"); // ./append_lines_to/
///     })
/// }
/// ```
pub fn append_lines<Path: AsRef<str>, Line: AsRef<str>>(
    file_path: &Path,
    lines: &Vec<Line>,
) -> Result<()> {
    let mut file = match read_lines(file_path) {
        Ok(lines) => lines,
        Err(_) => vec![],
    };
    file.extend_from_slice(&lines.to_vec_string());

    return write_lines(file_path, &file);
}

/// Deletes a file, or a directory **recursively**.
///
/// # Parameters
/// - `file_path`: **borrowed** `AsRef<str>` such as `String` or `&str`
///
/// # Returns
/// Result<`()`>
///
/// # Examples
/// ```
/// fn main() -> std::io::Result<()> {
///     Ok({
///         let file_path: &str = "absolute_or_relative_path/to_a_file/or_a_directory";
///         let file_path: String = String::from(file_path);
///
///         file_access::write_string(&file_path, &"Hello, World!");
///         file_access::delete(&file_path)?; // delete file
///
///         // Delete directory:
///         file_access::delete(&"absolute_or_relative_path")?;
///     })
/// }
/// ```
pub fn delete<Path: AsRef<str>>(file_path: &Path) -> Result<()> {
    let path = path_of(file_path);

    if path.is_file() {
        return fs::remove_file(path);
    }

    if path.is_dir() {
        return fs::remove_dir_all(path);
    }

    return Err(Error::new(ErrorKind::InvalidInput, file_path.as_ref()));
}

/// Copies the contents of a file and write it to a destination.
/// This function will entirely replace the contents of the destination if it already exists.
///
/// # Parameters
/// - `from`: **borrowed** `AsRef<str>` such as `String` or `&str`
/// - `to`: **borrowed** `AsRef<str>` such as `String` or `&str`
///
/// # Returns
/// Result<`()`>
///
/// # Examples
/// ```
/// fn main() -> std::io::Result<()> {
///     Ok({
///         let source: &str = "Cargo.toml";
///         let source: String = String::from(source);
///
///         let destination: &str = "Cargo.toml.2";
///         let destination: String = String::from(destination);
///
///         file_access::copy(&source, &destination)?;
///
///         // Delete file:
///         file_access::delete(&destination);
///     })
/// }
/// ```
pub fn copy<From: AsRef<str>, To: AsRef<str>>(from: &From, to: &To) -> Result<()> {
    write_string(to, &read_string(from)?)
}

/// Copies the contents of a file, writes it to a destination and then deletes the source.
/// This function will entirely replace the contents of the destination if it already exists.
///
/// # Parameters
/// - `from`: **borrowed** `AsRef<str>` such as `String` or `&str`
/// - `to`: **borrowed** `AsRef<str>` such as `String` or `&str`
///
/// # Returns
/// Result<`()`>
///
/// # Examples
/// ```
/// fn main() -> std::io::Result<()> {
///     Ok({
///         let source: &str = "file.1";
///         let source: String = String::from(source);
///
///         let destination: &str = "file.2";
///         let destination: String = String::from(destination);
///
///         file_access::write_string(&source, &"Hello, World!")?;
///         file_access::rename(&source, &destination)?;
///
///         // Clean-up:
///         file_access::delete(&destination)?;
///     })
/// }
/// ```
pub fn rename<From: AsRef<str>, To: AsRef<str>>(from: &From, to: &To) -> Result<()> {
    copy(from, to)?;

    return delete(from);
}

/// Queries metadata about the underlying file.
///
/// # Returns
/// Result<`Metadata`>
///
/// # Examples
/// ```
/// fn main() -> std::io::Result<()> {
///     Ok({
///         let file_path: &str = "Cargo.toml";
///         let file_path: String = String::from(file_path);
///
///         let metadata: std::fs::Metadata = file_access::get_metadata(&file_path)?;
///         println!("{:#?}", metadata);
///     })
/// }
/// ```
pub fn get_metadata<Path: AsRef<str>>(file_path: &Path) -> Result<Metadata> {
    get_file(file_path)?.metadata()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Result;

    // cargo test -- --show-output --test-threads=1
    // cargo test <TESTNAME> --show-output

    #[test]
    fn read_string() -> Result<()> {
        Ok({
            // Arrange
            let file = "Cargo.toml";

            // Action
            let text = super::read_string(&file)?;
            println!("{text}");

            // Assert
            assert_ne!(text.len(), 0);
        })
    }

    #[test]
    fn read_lines() -> Result<()> {
        Ok({
            // Arrange
            let file = "Cargo.toml";

            // Action
            let lines = super::read_lines(&file)?;
            for line in &lines {
                println!("{line}");
            }

            // Assert
            assert_ne!(lines.len(), 0);
        })
    }

    #[test]
    fn write_string() -> Result<()> {
        Ok({
            // Arrange
            let file = "write_string/file_access.txt";
            let text = "Hello, World!";

            // Action
            super::write_string(&file, &text)?;

            // Assert
            assert_eq!(super::read_string(&file)?, text);

            // Clean-up
            super::delete(&"write_string")?;
        })
    }

    #[test]
    fn write_lines() -> Result<()> {
        Ok({
            // Arrange
            let file = "write_lines/file_access.txt";
            let lines = "Hello, World!"
                .split_whitespace()
                .map(ToString::to_string)
                .collect();

            // Action
            super::write_lines(&file, &lines)?;

            // Assert
            assert_eq!(super::read_lines(&file)?, lines);

            // Clean-up
            super::delete(&"write_lines")?;
        })
    }

    #[test]
    fn append_string() -> Result<()> {
        Ok({
            // Arrange
            let file = "append_string/file_access.txt";
            let text = "Hello, World!";
            super::write_string(&file, &text)?;

            // Action
            super::append_string(&file, &text)?;

            // Assert
            assert_eq!(super::read_string(&file)?, format!("{text}{text}"));

            // Clean-up
            super::delete(&"append_string")?;
        })
    }

    #[test]
    fn append_lines() -> Result<()> {
        Ok({
            // Arrange
            let file = "append_lines/file_access.txt";
            let lines1 = vec!["1", "2"]; // .to_vec_string();
            super::write_lines(&file, &lines1)?;

            // Action
            let lines2 = vec!["3", "4"]; //.to_vec_string();
            super::append_lines(&file, &lines2)?;

            // Assert
            assert_eq!(super::read_lines(&file)?, vec!["1", "2", "3", "4"]); // .to_vec_string());

            // Clean-up
            super::delete(&"append_lines")?;
        })
    }

    #[test]
    fn delete() -> Result<()> {
        Ok({
            // Arrange
            let file = "delete/file_access.txt";
            mk_file(&file)?;

            // Action
            super::delete(&file)?;

            // Assert
            assert!(!path_of(&file).exists(), "{file} should no longer exist");

            // Clean-up
            super::delete(&"delete")?;
        })
    }

    #[test]
    fn copy() -> Result<()> {
        Ok({
            // Arrange
            let from = "copy_from/file_access.txt";
            let to = "copy_to/file_access.txt";
            super::write_string(&from, &"Hello, World!")?;

            // Action
            super::copy(&from, &to)?;

            // Assert
            assert_eq!(
                super::read_string(&from)?,
                super::read_string(&to)?,
                "{from} and {to} should contain the same text"
            );

            // Clean-up
            super::delete(&"copy_from")?;
            super::delete(&"copy_to")?;
        })
    }

    #[test]
    fn rename() -> Result<()> {
        Ok({
            // Arrange
            let from = "rename_from/file_access.txt";
            let to = "rename_to/file_access.txt";
            let text = "Hello, World!";
            super::write_string(&from, &text)?;

            // Action
            super::rename(&from, &to)?;

            // Assert
            assert!(!path_of(&from).exists(), "{from} should no longer exist");
            assert_eq!(
                super::read_string(&to)?,
                text,
                "{to} should contain: {text}"
            );

            // Clean-up
            super::delete(&"rename_from")?;
            super::delete(&"rename_to")?;
        })
    }
}
