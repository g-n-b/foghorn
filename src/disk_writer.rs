use std::fs::File;
use std::io::{self, BufWriter, Write};
use serde::Serialize;
use std::sync::Mutex;
use rocket::serde::json::serde_json;

/// A writer that saves serializable data to disk in a streaming fashion.
pub struct DiskWriter {
    writer: Mutex<BufWriter<File>>,
}

impl DiskWriter {
    /// Creates a new `DiskWriter` that writes to the specified file.
    ///
    /// # Arguments
    /// * `file_path` - The path to the file where data will be written.
    ///
    /// # Returns
    /// A `Result` containing the `DiskWriter` or an `io::Error`.
    pub fn new(file_path: &str) -> io::Result<Self> {
        let file = File::create(file_path)?;
        let writer = BufWriter::new(file);
        Ok(DiskWriter {
            writer: Mutex::new(writer),
        })
    }

    /// Writes a serializable item to the disk.
    ///
    /// # Arguments
    /// * `item` - The item to serialize and write.
    ///
    /// # Returns
    /// A `Result` indicating success or failure.
    pub fn write_item<T: Serialize>(&self, item: &T) -> io::Result<()> {
        let mut writer = self.writer.lock().unwrap(); // Acquire a mutable guard
        let serialized = serde_json::to_string(item)?;
        writer.write_all(serialized.as_bytes())?;
        writer.write_all(b"\n")?; // Add a newline for readability
        Ok(())
    }

    /// Flushes the internal buffer to ensure all data is written to disk.
    ///
    /// # Returns
    /// A `Result` indicating success or failure.
    pub fn flush(&self) -> io::Result<()> {
        let mut writer = self.writer.lock().unwrap(); // Acquire a mutable guard
        writer.flush()
    }
}