// Attribution: large part of this file was written by ChatGPT.
// The author has then verified the correctness of the code and
// added some tests to cover missing functionality.

use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;

use crate::{assert_eq_res, assert_res};

// Test creating a file
pub fn test_create_file() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "test_file.txt";
    fs::File::create(file_path)?;
    assert_res!(Path::new(file_path).exists());
    Ok(())
}

// Test writing to and reading from a file
pub fn test_write_and_read_file() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "test_write_read.txt";
    let content = "Hello, Rust!";
    fs::write(file_path, content)?;

    let read_content = fs::read_to_string(file_path)?;
    assert_eq_res!(read_content, content);
    Ok(())
}

// Test if file exists
pub fn test_file_exists() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "test_file_existence.txt";
    fs::File::create(file_path)?;
    assert_res!(Path::new(file_path).exists());
    Ok(())
}

// Test removing a file
pub fn test_remove_file() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "test_remove_file.txt";
    fs::File::create(file_path)?;
    fs::remove_file(file_path)?;
    assert_res!(!Path::new(file_path).exists());
    Ok(())
}

// Test creating a directory
pub fn test_create_directory() -> Result<(), Box<dyn std::error::Error>> {
    let dir_path = "test_directory";
    fs::create_dir(dir_path)?;
    assert_res!(Path::new(dir_path).is_dir());
    Ok(())
}

// Test if a directory exists
pub fn test_directory_exists() -> Result<(), Box<dyn std::error::Error>> {
    let dir_path = "test_directory_exists";
    fs::create_dir(dir_path)?;
    assert_res!(Path::new(dir_path).is_dir());
    Ok(())
}

// Test removing a directory
pub fn test_remove_directory() -> Result<(), Box<dyn std::error::Error>> {
    let dir_path = "test_remove_directory";
    fs::create_dir(dir_path)?;
    fs::remove_dir(dir_path)?;
    assert_res!(!Path::new(dir_path).is_dir());
    Ok(())
}

// Test creating a file with a specific path
pub fn test_create_file_with_path() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "nested/test_file_path.txt";
    fs::create_dir_all("nested")?;
    fs::File::create(file_path)?;
    assert_res!(Path::new(file_path).exists());
    Ok(())
}

// Test writing to a file multiple times
pub fn test_file_write_multiple_times() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "test_write_multiple_times.txt";
    {
        let mut file = fs::File::create(file_path)?;
        writeln!(file, "First line")?;
    }

    {
        let mut file = fs::OpenOptions::new().append(true).open(file_path)?;
        writeln!(file, "Second line")?;
    }

    let content = fs::read_to_string(file_path)?;
    assert_res!(content.contains("First line"));
    assert_res!(content.contains("Second line"));
    Ok(())
}

// Test reading a file as a string
pub fn test_file_read_as_string() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "test_file_read_string.txt";
    let content = "This is a test";
    fs::write(file_path, content)?;

    let read_content = fs::read_to_string(file_path)?;
    assert_eq_res!(read_content, content);
    Ok(())
}

// Test reading a file with BufReader
pub fn test_read_file_with_bufreader() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "test_bufreader.txt";
    let content = "Buffered reader test\nThis is the second line";
    fs::write(file_path, content)?;

    let file = fs::File::open(file_path)?;
    let mut buf_reader = BufReader::new(file);
    let mut read_content = String::new();
    buf_reader.read_line(&mut read_content)?;
    assert_eq_res!(read_content, "Buffered reader test\n");
    read_content.clear();
    buf_reader.read_to_string(&mut read_content)?;

    assert_eq_res!(read_content, "This is the second line");
    Ok(())
}

// Test reading a file with File::open
pub fn test_read_file_with_file_open() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "test_open_file.txt";
    let content = "Open file test";
    fs::write(file_path, content)?;

    let mut file = fs::File::open(file_path)?;
    let mut read_content = String::new();
    file.read_to_string(&mut read_content)?;

    assert_eq_res!(read_content, content);
    Ok(())
}

// Test writing to and reading a large file
pub fn test_write_and_read_large_file() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "test_large_file.txt";
    let content: String = (0..10000).map(|_| "Hello\n").collect();
    fs::write(file_path, &content)?;

    let read_content = fs::read_to_string(file_path)?;
    assert_eq_res!(read_content, content);
    Ok(())
}

// Test creating nested directories
pub fn test_create_nested_directories() -> Result<(), Box<dyn std::error::Error>> {
    let nested_dir_path = "parent/child";
    fs::create_dir_all(nested_dir_path)?;
    assert_res!(Path::new(nested_dir_path).is_dir());
    Ok(())
}

// Test listing a directory
pub fn test_list_directory() -> Result<(), Box<dyn std::error::Error>> {
    let dir_path = "list_dir_test";
    fs::create_dir(dir_path)?;
    fs::write(format!("{}/file1.txt", dir_path), "test1")?;
    fs::write(format!("{}/file2.txt", dir_path), "test2")?;

    let entries = fs::read_dir(dir_path)?
        .filter_map(Result::ok)
        .map(|entry| entry.file_name())
        .collect::<Vec<_>>();

    assert_eq_res!(entries.len(), 2);
    Ok(())
}

// Test copying a file
pub fn test_copy_file() -> Result<(), Box<dyn std::error::Error>> {
    let src_path = "src_copy_file.txt";
    let dest_path = "dest_copy_file.txt";
    fs::write(src_path, "Copy test")?;

    fs::copy(src_path, dest_path)?;
    let read_content = fs::read_to_string(dest_path)?;
    assert_eq_res!(read_content, "Copy test");
    Ok(())
}

// Test moving a file
pub fn test_move_file() -> Result<(), Box<dyn std::error::Error>> {
    let src_path = "src_move_file.txt";
    let dest_path = "dest_move_file.txt";
    fs::write(src_path, "Move test")?;

    fs::rename(src_path, dest_path)?;
    assert_res!(!Path::new(src_path).exists());
    assert_res!(Path::new(dest_path).exists());
    let read_content = fs::read_to_string(dest_path)?;
    assert_eq_res!(read_content, "Move test");
    Ok(())
}

// Test empty directory
pub fn test_empty_directory() -> Result<(), Box<dyn std::error::Error>> {
    let dir_path = "empty_dir_test";
    fs::create_dir(dir_path)?;

    let entries: Vec<_> = fs::read_dir(dir_path)?.filter_map(Result::ok).collect();

    assert_res!(entries.is_empty());
    Ok(())
}

// Test reading an empty file
pub fn test_read_empty_file() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "test_empty_file.txt";
    fs::File::create(file_path)?;

    let content = fs::read_to_string(file_path)?;
    assert_eq_res!(content, "");
    Ok(())
}

// Test directory listing after removal
pub fn test_directory_listing_after_removal() -> Result<(), Box<dyn std::error::Error>> {
    let dir_path = "dir_after_removal";
    fs::create_dir(dir_path)?;
    fs::write(format!("{}/file.txt", dir_path), "test")?;

    fs::remove_dir_all(dir_path)?;

    let entries: Vec<_> = fs::read_dir(".")?
        .filter_map(Result::ok)
        .map(|entry| entry.file_name())
        .collect();

    assert_res!(!entries.contains(&"dir_after_removal".into()));
    Ok(())
}

// Test replacing a file
pub fn test_file_replacement() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "replace_test.txt";
    fs::write(file_path, "Initial content")?;
    fs::write(file_path, "Replaced content")?;

    let content = fs::read_to_string(file_path)?;
    assert_eq_res!(content, "Replaced content");
    Ok(())
}
