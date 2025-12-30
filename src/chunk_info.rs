/// Represents a chunk of data for parallel processing.
///
/// This struct contains the metadata needed to process a specific
/// portion of a file in parallel, including start/end positions and size.
pub struct ChunkInfo {
    /// The starting byte position in the file
    pub start: u64,
    /// The ending byte position in the file
    pub end: u64,
    /// The total size of the chunk in bytes
    pub size: u64,
}
