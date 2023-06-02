use crate::{internal::types::*, *};
use std::{
    env::current_dir,
    fs::{canonicalize, Metadata},
    io::{Error, ErrorKind, Result},
};

pub struct FilePath {
    get_path: String,
}

impl FilePath {
    pub fn access<Path: AsRef<str>>(file_path: &Path) -> Self {
        Self {
            get_path: file_path.as_ref().to_string(),
        }
    }

    pub fn get_full_path(&self) -> Result<String> {
        Ok(canonicalize(&self.get_path)?.display().to_string())
    }

    pub fn get_relative_path(&self) -> Result<String> {
        match path_of(&self.get_full_path()?).strip_prefix(&current_dir()?) {
            Ok(p) => match p.to_str() {
                Some(s) => Ok(s.to_string()),
                None => Err(Error::new(ErrorKind::InvalidData, "&Path.to_str() error")),
            },
            Err(x) => Err(Error::new(ErrorKind::InvalidInput, x)),
        }
    }

    pub fn read_string(&self) -> Result<String> {
        read_string(self)
    }

    pub fn read_lines(&self) -> Result<Lines> {
        read_lines(self)
    }

    pub fn write_string<Text: AsRef<str>>(&self, text: &Text) -> Result<()> {
        write_string(self, text)
    }

    pub fn write_lines<Line: AsRef<str>>(&self, lines: &Vec<Line>) -> Result<()> {
        write_lines(self, lines)
    }

    pub fn append_string<Text: AsRef<str>>(&self, text: &Text) -> Result<()> {
        append_string(self, text)
    }

    pub fn append_lines<Line: AsRef<str>>(&self, lines: &Vec<Line>) -> Result<()> {
        append_lines(self, lines)
    }

    pub fn delete(&self) -> Result<()> {
        delete(self)
    }

    pub fn copy_to<Path: AsRef<str>>(&self, to: &Path) -> Result<()> {
        copy(self, to)
    }

    pub fn rename_to<Path: AsRef<str>>(&self, to: &Path) -> Result<()> {
        rename(self, to)
    }

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
