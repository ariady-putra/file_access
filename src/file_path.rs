use crate::{internal::types::*, *};
use std::{
    env::current_dir,
    fs::{canonicalize, Metadata},
    io::{Error, ErrorKind, Result},
};

/// A wrapper that acts as a file handle.
pub struct FilePath {
    get_path: String,
}

impl FilePath {
    /// Wraps a **borrowed** `AsRef<str>`, such as `String` or `&str`, into a `FilePath`.
    ///
    /// # Returns
    /// file_access::file_path::`FilePath`
    ///
    /// # Examples
    /// ```
    /// use file_access::file_path::*;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     Ok({
    ///         let file_path: &str = "absolute/or/relative.path";
    ///         let file_path: String = String::from(file_path);
    ///
    ///         let file: FilePath = FilePath::access(&file_path);
    ///         assert_eq!(file.as_ref(), file_path);
    ///     })
    /// }
    /// ```
    pub fn access<Path: AsRef<str>>(file_path: &Path) -> Self {
        Self {
            get_path: file_path.as_ref().to_string(),
        }
    }

    /// Attempts to get the absolute path of an **existing** file or directory.
    ///
    /// # Returns
    /// Result<`String`>
    ///
    /// # Examples
    /// ```
    /// use file_access::file_path::*;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     Ok({
    ///         let file_path: &str = "./Cargo.toml";
    ///         let file_path: String = String::from(file_path);
    ///
    ///         let file: FilePath = FilePath::access(&file_path);
    ///         let path: String = file.get_full_path()?;
    ///         println!("{}", path);
    ///     })
    /// }
    /// ```
    pub fn get_full_path(&self) -> Result<String> {
        Ok(canonicalize(&self.get_path)?.display().to_string())
    }

    /// Attempts to get the relative path of an **existing** file or directory.
    ///
    /// # Returns
    /// Result<`String`>
    ///
    /// # Examples
    /// ```
    /// use file_access::file_path::*;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     Ok({
    ///         let file_path: &str = "/home/ariady/rust/file_access/Cargo.toml";
    ///         let file_path: String = String::from(file_path);
    ///
    ///         let file: FilePath = FilePath::access(&file_path);
    ///         let path: String = file.get_relative_path()?;
    ///         println!("{}", path);
    ///     })
    /// }
    /// ```
    pub fn get_relative_path(&self) -> Result<String> {
        match path_of(&self.get_full_path()?).strip_prefix(&current_dir()?) {
            Ok(p) => match p.to_str() {
                Some(s) => Ok(s.to_string()),
                None => Err(Error::new(ErrorKind::InvalidData, "&Path.to_str() error")),
            },
            Err(x) => Err(Error::new(ErrorKind::InvalidInput, x)),
        }
    }

    /// Reads the contents of a file.
    ///
    /// # Returns
    /// Result<`String`>
    ///
    /// # Examples
    /// ```
    /// use file_access::file_path::*;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     Ok({
    ///         let file_path: &str = "Cargo.toml";
    ///         let file_path: String = String::from(file_path);
    ///
    ///         let file: FilePath = FilePath::access(&file_path);
    ///         let text: String = file.read_string()?;
    ///         println!("{}", text);
    ///     })
    /// }
    /// ```
    pub fn read_string(&self) -> Result<String> {
        read_string(self)
    }

    /// Reads the contents of a file and returns it as lines.
    ///
    /// # Returns
    /// Result<`Vec<String>`>
    ///
    /// # Examples
    /// ```
    /// use file_access::file_path::*;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     Ok({
    ///         let file_path: &str = "Cargo.toml";
    ///         let file_path: String = String::from(file_path);
    ///
    ///         let file: FilePath = FilePath::access(&file_path);
    ///         let lines: Vec<String> = file.read_lines()?;
    ///         lines.iter().for_each(|line| println!("{}", line));
    ///     })
    /// }
    /// ```
    pub fn read_lines(&self) -> Result<Lines> {
        read_lines(self)
    }

    /// Writes text to a file. This function will create the file **and its full directory path** if they don't exist,
    /// and will entirely replace the contents.
    ///
    /// # Parameters
    /// - `text`: **borrowed** `AsRef<str>` such as `String` or `&str`
    ///
    /// # Returns
    /// Result<`()`>
    ///
    /// # Examples
    /// ```
    /// use file_access::file_path::*;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     Ok({
    ///         let file_path: &str = "fp_write/absolute_or_relative.path";
    ///         let file_path: String = String::from(file_path);
    ///
    ///         let text: &str = "Hello, World!";
    ///         let text: String = String::from(text);
    ///
    ///         let file: FilePath = FilePath::access(&file_path);
    ///         file.write_string(&text)?;
    ///
    ///         // Clean-up:
    ///         let file = FilePath::access(&"fp_write"); // ./fp_write/
    ///         file.delete()?;
    ///     })
    /// }
    /// ```
    pub fn write_string<Text: AsRef<str>>(&self, text: &Text) -> Result<()> {
        write_string(self, text)
    }

    /// Writes a list of text as lines to a file. This function will create the file **and its full directory path** if they don't exist,
    /// and will entirely replace the contents with the provided strings each on its own line.
    ///
    /// # Parameters
    /// - `lines`: **borrowed** `Vec<AsRef<str>>` such as `Vec<String>` or `Vec<&str>`
    ///
    /// # Returns
    /// Result<`()`>
    ///
    /// # Examples
    /// ```
    /// use file_access::file_path::*;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     Ok({
    ///         let file_path: &str = "fp_lines/absolute_or_relative.path";
    ///         let file_path: String = String::from(file_path);
    ///
    ///         let lines: Vec<&str> = "Hello, World!".split_whitespace().collect();
    ///         let lines: Vec<String> = lines.iter().map(ToString::to_string).collect();
    ///
    ///         let file: FilePath = FilePath::access(&file_path);
    ///         file.write_lines(&lines)?;
    ///
    ///         // Clean-up:
    ///         let file = FilePath::access(&"fp_lines"); // ./fp_lines/
    ///         file.delete()?;
    ///     })
    /// }
    /// ```
    pub fn write_lines<Line: AsRef<str>>(&self, lines: &Vec<Line>) -> Result<()> {
        write_lines(self, lines)
    }

    /// Appends text to a file. This function will append the contents of the file,
    /// or write a new one **and its full directory path** if they don't exist yet.
    ///
    /// # Parameters
    /// - `text`: **borrowed** `AsRef<str>` such as `String` or `&str`
    ///
    /// # Returns
    /// Result<`()`>
    ///
    /// # Examples
    /// ```
    /// use file_access::file_path::*;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     Ok({
    ///         let file_path: &str = "fp_append/absolute_or_relative.path";
    ///         let file_path: String = String::from(file_path);
    ///
    ///         let text: &str = "Hello, World!";
    ///         let text: String = String::from(text);
    ///
    ///         let file: FilePath = FilePath::access(&file_path);
    ///         file.append_string(&text)?;
    /// 
    ///         // Clean-up:
    ///         let file = FilePath::access(&"fp_append"); // ./fp_append/
    ///         file.delete()?;
    ///     })
    /// }
    /// ```
    pub fn append_string<Text: AsRef<str>>(&self, text: &Text) -> Result<()> {
        append_string(self, text)
    }

    /// Appends a list of text as lines to a file. This function will append the contents of the file,
    /// or write a new one **and its full directory path** if they don't exist yet.
    ///
    /// # Parameters
    /// - `lines`: **borrowed** `Vec<AsRef<str>>` such as `Vec<String>` or `Vec<&str>`
    ///
    /// # Returns
    /// Result<`()`>
    ///
    /// # Examples
    /// ```
    /// use file_access::file_path::*;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     Ok({
    ///         let file_path: &str = "fp_append_lines/absolute_or_relative.path";
    ///         let file_path: String = String::from(file_path);
    ///
    ///         let lines: Vec<&str> = "Hello, World!".split_whitespace().collect();
    ///         let lines: Vec<String> = lines.iter().map(ToString::to_string).collect();
    ///
    ///         let file: FilePath = FilePath::access(&file_path);
    ///         file.append_lines(&lines)?;
    /// 
    ///         // Clean-up:
    ///         let file = FilePath::access(&"fp_append_lines"); // ./fp_append_lines/
    ///         file.delete()?;
    ///     })
    /// }
    /// ```
    pub fn append_lines<Line: AsRef<str>>(&self, lines: &Vec<Line>) -> Result<()> {
        append_lines(self, lines)
    }

    /// Deletes a file, or a directory **recursively**.
    ///
    /// # Returns
    /// Result<`()`>
    ///
    /// # Examples
    /// ```
    /// use file_access::file_path::*;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     Ok({
    ///         let file_path: &str = "absolute_or_relative_path/to_a_file/or_a_directory";
    ///         let file_path: String = String::from(file_path);
    ///
    ///         let file: FilePath = FilePath::access(&file_path);
    ///         file.write_string(&"Hello, World!");
    ///         file.delete()?; // delete file
    ///
    ///         // Delete directory:
    ///         let dir = FilePath::access(&"absolute_or_relative_path");
    ///         dir.delete()?;
    ///     })
    /// }
    /// ```
    pub fn delete(&self) -> Result<()> {
        delete(self)
    }

    /// Copies the contents of a file and write it to a destination.
    /// This function will entirely replace the contents of the destination if it already exists.
    ///
    /// # Parameters
    /// - `to`: **borrowed** `AsRef<str>` such as `String` or `&str`
    ///
    /// # Returns
    /// Result<`()`>
    ///
    /// # Examples
    /// ```
    /// use file_access::file_path::*;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     Ok({
    ///         let source: &str = "Cargo.toml";
    ///         let source: String = String::from(source);
    ///
    ///         let destination: &str = "Cargo.toml.2";
    ///         let destination: String = String::from(destination);
    ///
    ///         let file: FilePath = FilePath::access(&source);
    ///         file.copy_to(&destination)?;
    ///
    ///         // Delete file:
    ///         let destination = FilePath::access(&destination);
    ///         destination.delete()?;
    ///     })
    /// }
    /// ```
    pub fn copy_to<Path: AsRef<str>>(&self, to: &Path) -> Result<()> {
        copy(self, to)
    }

    /// Copies the contents of a file, writes it to a destination and then deletes the source.
    /// This function will entirely replace the contents of the destination if it already exists.
    ///
    /// # Parameters
    /// - `to`: **borrowed** `AsRef<str>` such as `String` or `&str`
    ///
    /// # Returns
    /// Result<`()`>
    ///
    /// # Examples
    /// ```
    /// use file_access::file_path::*;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     Ok({
    ///         let source: &str = "file.1";
    ///         let source: String = String::from(source);
    ///
    ///         let destination: &str = "file.2";
    ///         let destination: String = String::from(destination);
    ///
    ///         let file: FilePath = FilePath::access(&source);
    ///         file.write_string(&"Hello, World!")?;
    ///         file.rename_to(&destination)?;
    ///
    ///         // Clean-up:
    ///         let file = FilePath::access(&destination);
    ///         file.delete()?;
    ///     })
    /// }
    /// ```
    pub fn rename_to<Path: AsRef<str>>(&self, to: &Path) -> Result<()> {
        rename(self, to)
    }

    /// Queries metadata about the underlying file.
    ///
    /// # Returns
    /// Result<`Metadata`>
    ///
    /// # Examples
    /// ```
    /// use file_access::file_path::*;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     Ok({
    ///         let file_path: &str = "Cargo.toml";
    ///         let file_path: String = String::from(file_path);
    ///
    ///         let file: FilePath = FilePath::access(&file_path);
    ///         let metadata: std::fs::Metadata = file.get_metadata()?;
    ///         println!("{:#?}", metadata);
    ///     })
    /// }
    /// ```
    pub fn get_metadata(&self) -> Result<Metadata> {
        get_metadata(self)
    }
}

impl AsRef<str> for FilePath {
    fn as_ref(&self) -> &str {
        self.get_path.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::as_file::*;
    use std::io::Result;

    // cargo test -- --show-output --test-threads=1
    // cargo test <TESTNAME> --show-output

    #[test]
    fn read_string() -> Result<()> {
        Ok({
            // Arrange
            let file = FilePath::access(&"Cargo.toml");

            // Action
            let text = file.read_string()?;
            println!("{text}");

            // Assert
            assert_ne!(text.len(), 0);
        })
    }

    #[test]
    fn read_lines() -> Result<()> {
        Ok({
            // Arrange
            let file = FilePath::access(&"Cargo.toml");

            // Action
            let lines = file.read_lines()?;
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
            let file = FilePath::access(&"write_string.txt");
            let text = "Hello, World!";

            // Action
            file.write_string(&text)?;

            // Assert
            assert_eq!(file.read_string()?, text);

            // Clean-up
            file.delete()?;
        })
    }

    #[test]
    fn write_lines() -> Result<()> {
        Ok({
            // Arrange
            let file = FilePath::access(&"write_lines.txt");
            let lines = "Hello, World!"
                .split_whitespace()
                .map(ToString::to_string)
                .collect();

            // Action
            file.write_lines(&lines)?;

            // Assert
            assert_eq!(file.read_lines()?, lines);

            // Clean-up
            file.delete()?;
        })
    }

    #[test]
    fn append_string() -> Result<()> {
        Ok({
            // Arrange
            let file = FilePath::access(&"append_string.txt");
            let text = "Hello, World!";
            file.write_string(&text)?;

            // Action
            file.append_string(&text)?;

            // Assert
            assert_eq!(file.read_string()?, format!("{text}{text}"));

            // Clean-up
            file.delete()?;
        })
    }

    #[test]
    fn append_lines() -> Result<()> {
        Ok({
            // Arrange
            let file = FilePath::access(&"append_lines.txt");
            let lines1 = vec!["1", "2"]; // .to_vec_string();
            file.write_lines(&lines1)?;

            // Action
            let lines2 = vec!["3", "4"]; //.to_vec_string();
            file.append_lines(&lines2)?;

            // Assert
            assert_eq!(file.read_lines()?, vec!["1", "2", "3", "4"]); // .to_vec_string());

            // Clean-up
            file.delete()?;
        })
    }

    #[test]
    fn delete() -> Result<()> {
        Ok({
            // Arrange
            let path = "delete.txt";
            let file = FilePath::access(&path);
            mk_file(&path)?;

            // Action
            file.delete()?;

            // Assert
            assert!(!path_of(&path).exists(), "{path} should no longer exist");
        })
    }

    #[test]
    fn copy() -> Result<()> {
        Ok({
            // Arrange
            let from = "copy_from.txt";
            let to = "copy_to.txt";
            let file = FilePath::access(&from);
            file.write_string(&"Hello, World!")?;

            // Action
            file.copy_to(&to)?;

            // Assert
            assert_eq!(
                from.as_file().read_string()?,
                to.as_file().read_string()?,
                "{from} and {to} should contain the same text"
            );

            // Clean-up
            from.as_file().delete()?;
            to.as_file().delete()?;
        })
    }

    #[test]
    fn rename() -> Result<()> {
        Ok({
            // Arrange
            let from = "rename_from.txt";
            let to = "rename_to.txt";
            let text = "Hello, World!";
            let file = FilePath::access(&from);
            file.write_string(&text)?;

            // Action
            file.rename_to(&to)?;

            // Assert
            assert!(!path_of(&from).exists(), "{from} should no longer exist");
            assert_eq!(
                to.as_file().read_string()?,
                text,
                "{to} should contain: {text}"
            );

            // Clean-up
            to.as_file().delete()?;
        })
    }
}
