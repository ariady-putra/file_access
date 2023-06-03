use crate::*;

pub trait AsFile {
    fn as_file(&self) -> FilePath;
}

impl<Path: AsRef<str>> AsFile for Path {
    /// Converts an `AsRef<str>`, such as `String` or `&str`, into a `FilePath`.
    ///
    /// # Examples
    /// ```
    /// use file_access::AsFile;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     Ok({
    ///         "as_file.1".as_file().write_string(&"Hello, World!")?;
    ///
    ///         let file = "as_file.1".as_file();
    ///         file.append_lines(&vec!["hello", "world"])?;
    ///         file.copy_to(&"as_file.2")?; // copies ./as_file.1 to ./as_file.2
    ///
    ///         "as_file.2".as_file().rename_to(&"as_file.1")?; // replace
    ///         "as_file.1".as_file().delete()?; // clean-up
    ///     })
    /// }
    /// ```
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
