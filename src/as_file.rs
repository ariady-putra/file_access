use crate::file_path::*;

pub trait AsFile {
    fn as_file(&self) -> FilePath;
}

impl<Path: AsRef<str>> AsFile for Path {
    /// Converts an `AsRef<str>`, such as `String` or `&str`, into a `FilePath`.
    fn as_file(&self) -> FilePath {
        FilePath::access(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Result;

    #[test]
    fn str_as_file() -> Result<()> {
        Ok({
            let text = "Cargo.toml".as_file().read_string()?;
            assert_ne!(text.len(), 0, "Cargo.toml shouldn't be empty");
        })
    }

    #[test]
    fn string_as_file() -> Result<()> {
        Ok({
            // Arrange
            let lines = vec!["hello", "world"]; // .to_vec_string();

            // Action
            let file = "from_string.txt".to_string().as_file();
            file.write_lines(&lines)?;

            // Assert
            assert_eq!(file.read_lines()?, lines);

            // Clean-up
            file.delete()?;
        })
    }
}
