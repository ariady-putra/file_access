use internal::{traits::to_vec_string::*, types::*};
use std::{
    fs::{self, File, Metadata},
    io::{Error, ErrorKind, Read, Result},
    path::PathBuf,
};

pub mod as_file;
pub mod file_path;
mod internal;

fn get_file(file_path: &impl AsRef<str>) -> Result<File> {
    File::open(file_path.as_ref())
}

fn path_of(file_path: &impl AsRef<str>) -> PathBuf {
    PathBuf::from(file_path.as_ref())
}

fn mk_file(file_path: &impl AsRef<str>) -> Result<File> {
    if let Some(path) = path_of(file_path).parent() {
        fs::create_dir_all(path)?;
    }
    return File::create(file_path.as_ref());
}

pub fn read_string<Path: AsRef<str>>(file_path: &Path) -> Result<String> {
    let mut buf = String::new();
    get_file(file_path)?.read_to_string(&mut buf)?;

    return Ok(buf);
}

pub fn read_lines<Path: AsRef<str>>(file_path: &Path) -> Result<Lines> {
    Ok(read_string(file_path)?
        .lines()
        .map(ToString::to_string)
        .collect())
}

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

pub fn write_lines<Path: AsRef<str>, Line: AsRef<str>>(
    file_path: &Path,
    lines: &Vec<Line>,
) -> Result<()> {
    write_string(file_path, &lines.to_vec_string().join("\n"))
}

pub fn append_string<Path: AsRef<str>, Text: AsRef<str>>(
    file_path: &Path,
    text: &Text,
) -> Result<()> {
    write_string(
        file_path,
        &format!("{}{}", read_string(file_path)?, text.as_ref()),
    )
}

pub fn append_lines<Path: AsRef<str>, Line: AsRef<str>>(
    file_path: &Path,
    lines: &Vec<Line>,
) -> Result<()> {
    let mut file = read_lines(file_path)?;
    file.extend_from_slice(&lines.to_vec_string());

    return write_lines(file_path, &file);
}

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

pub fn copy<From: AsRef<str>, To: AsRef<str>>(from: &From, to: &To) -> Result<()> {
    write_string(to, &read_string(from)?)
}

pub fn rename<From: AsRef<str>, To: AsRef<str>>(from: &From, to: &To) -> Result<()> {
    copy(from, to)?;

    return delete(from);
}

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
