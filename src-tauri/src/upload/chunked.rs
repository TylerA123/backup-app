use std::path::Path;

pub struct ChunkedUploader {
    chunk_size: u64,
}

impl ChunkedUploader {
    pub fn new(chunk_size_mb: u64) -> Self {
        ChunkedUploader {
            chunk_size: chunk_size_mb * 1024 * 1024,
        }
    }

    pub fn should_chunk(&self, file_size: u64) -> bool {
        file_size > self.chunk_size * 10 // > 50MB
    }

    pub fn chunk_count(&self, file_size: u64) -> u64 {
        (file_size + self.chunk_size - 1) / self.chunk_size
    }

    pub fn read_chunk(&self, path: &Path, chunk_index: u64) -> Result<Vec<u8>, String> {
        use std::io::Read;
        let mut file = std::fs::File::open(path)
            .map_err(|e| format!("Cannot open chunk file: {}", e))?;

        let offset = chunk_index * self.chunk_size;
        let size = self.chunk_size.min(
            file.metadata()
                .map_err(|e| format!("Cannot get metadata: {}", e))?
                .len()
                - offset,
        );

        let mut buffer = vec![0u8; size as usize];
        use std::io::Seek;
        file.seek(std::io::SeekFrom::Start(offset))
            .map_err(|e| format!("Seek error: {}", e))?;
        file.read_exact(&mut buffer)
            .map_err(|e| format!("Chunk read error: {}", e))?;

        Ok(buffer)
    }
}
