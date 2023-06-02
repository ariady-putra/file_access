# Rust File Access Wrapper Lib
This lib-proj is mainly for me to experiment with various Rust concepts.

## Exposed Actions
- `read_string`: Returns `String`.
- `read_lines`: Returns `Vec<String>`.
- `write_string`: Takes a **borrowed** `AsRef<str>` such as `String` or `&str`. This function will create a file **and its full directory path** if they don't exist, and will entirely replace the contents.
- `write_lines`: Takes a **borrowed** `Vec<AsRef<str>>` such as `Vec<String>` or `Vec<&str>`. This function will create a file **and its full directory path** if they don't exist, and will entirely replace the contents with the provided strings each on its own line.
- `append_string`: Takes a **borrowed** `AsRef<str>` such as `String` or `&str`. This function will append the contents of a file, or write a new one **and its full directory path** if they don't exist yet.
- `append_lines`: Takes a **borrowed** `Vec<AsRef<str>>` such as `Vec<String>` or `Vec<&str>`. This function will append the contents of a file, or write a new one **and its full directory path** if they don't exist yet.
- `delete`: This function will delete a file, or a directory **recursively**.
- `copy`/`copy_to`: This function will copy the contents of a file and write it to a destination. It will entirely replace the contents of the destination if it already exists.
- `rename`/`rename_to`: This function will copy the contents of a file, write it to a destination and then delete the source. It will entirely replace the contents of the destination if it already exists.

## Usages
There are 3 ways to use this library:
- By calling methods directly: `let result = file_access::METHOD_NAME(&file_path, &..)?`
- By using a FilePath handle: `let file = FilePath::access(&file_path); let result = file.METHOD_NAME(&..)?`
- By using the AsFile trait: `let file = "string_path".as_file(); let result = file.METHOD_NAME(&..)?`

where `file_path` can be a **borrowed** `String`, `&str`, or `file_access::file_path::FilePath`.

### Examples
- Call `read_string` directly:
```rust
let text: String = file_access::read_string(&file_path)?;
println!("{text}");
```

- Use a `FilePath` handle:
```rust
let file: FilePath = FilePath::access(&file_path);
let lines: Vec<&str> = vec!["hello", "world"];

file.write_lines(&lines)?;
file.append_lines(&lines)?;
file.copy_to(&another_path)?;
```

- Use the `AsFile` trait:
```rust
// delete a file:
file_path.as_file().delete()?;

// rename a file:
"another_path".as_file().rename_to(&"a_new_file_path")?;
```
