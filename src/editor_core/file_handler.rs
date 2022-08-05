use std::fs::{File, Metadata, OpenOptions};
use std::fs;
use std::io::Read;
use std::io::Write;

pub struct FileHandler {
    file_handle: File,
    file_metadata: Metadata,
    file_name: String,
}

impl FileHandler {
    pub fn from(filename: String) -> Self {
        let f = File::options()
                .read(true)
                .write(true)
                .open(&filename)
                .expect("no file found");
        let metadata = fs::metadata(&filename).expect("unable to read metadata");
        FileHandler {
            file_handle: f, 
            file_metadata: metadata,
            file_name: filename,
        }
    }

    pub fn read_lined_buffer(&mut self) -> Vec<Vec<u8>> {
        let mut buffer = vec![0; self.file_metadata.len() as usize];
        self.file_handle.read(&mut buffer).expect("buffer overflow");
        buffer
            .split(
                |character| 
                character == "\n".as_bytes()
                                .first()
                                .unwrap()
            )
            .map(|line| line.to_vec())
            .collect::<Vec<Vec<u8>>>()
    }

    pub fn write_lined_buffer(&self, buffer: Vec<Vec<u8>>) -> std::io::Result<usize> {
        let buffer = buffer
            .iter()
            .map(
                |line| 
                std::str::from_utf8(line)
                .expect("invalid utf8 pattern")
                .as_bytes()
            )
            .collect::<Vec<&[u8]>>()
            .concat();
        let mut file = OpenOptions::new()
            .write(true)
            .open(self.file_name.as_str())?;
        
        file.write_all(buffer.as_ref())?;

        Ok(0)
    }
}