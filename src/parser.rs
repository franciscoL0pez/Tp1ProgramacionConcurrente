use crate::chat_message::ChatMessage;
use crate::chunk_info::ChunkInfo;
use crate::custom_error::CustomError;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};

/// Analyzes a file and divides it into chunks for parallel processing.
///
/// # Errors
///
/// Returns `CustomError::IOError` if the file cannot be opened or read.
pub fn analize_file_for_chunks(
    path: &str,
    num_of_threads: usize,
) -> Result<Vec<ChunkInfo>, CustomError> {
    let file = File::open(path).map_err(|e| CustomError::IOError(e.to_string()))?;
    let metadata = file
        .metadata()
        .map_err(|e| CustomError::IOError(e.to_string()))?;
    let file_size = metadata.len();

    let mut buf_reader = BufReader::new(&file);

    let chunk_size = file_size / num_of_threads as u64;
    let mut chunks = Vec::new();

    let mut start = 0;
    for i in 0..num_of_threads {
        let mut end = start + chunk_size;
        if i == num_of_threads - 1 {
            end = file_size;
        } else {
            end = find_next_object_boundary(&mut buf_reader, end)?;
        }

        chunks.push(ChunkInfo {
            start,
            end,
            size: end.saturating_sub(start),
        });

        start = end;
    }

    Ok(chunks)
}

/// Finds the next JSON object boundary starting from a given position.
///
/// This function searches for the end of a JSON object ('}' followed by ',' or ']')
/// to ensure chunk boundaries don't split JSON objects, enabling safe parallel processing.
///
/// # Arguments
///
/// * `buf_reader` - Mutable reference to the buffered file reader
/// * `start_pos` - Starting position in the file to search from
///
/// # Returns
///
/// Returns the position of the next safe boundary for chunk splitting.
///
/// # Errors
///
/// Returns `CustomError::IOError` if file seeking or reading operations fail.
fn find_next_object_boundary(
    buf_reader: &mut BufReader<&File>,
    start_pos: u64,
) -> Result<u64, CustomError> {
    buf_reader
        .seek(SeekFrom::Start(start_pos))
        .map_err(|e| CustomError::IOError(e.to_string()))?;

    let mut position = start_pos;
    let mut buffer = vec![0u8; 1_048_576].into_boxed_slice();

    loop {
        let bytes_read = buf_reader
            .read(&mut buffer)
            .map_err(|e| CustomError::IOError(e.to_string()))?;
        if bytes_read == 0 {
            break;
        }

        for i in 0..bytes_read {
            if buffer[i] == b'}'
                && i + 1 < bytes_read
                && (buffer[i + 1] == b',' || buffer[i + 1] == b']')
            {
                return Ok(position + i as u64 + 1);
            }
        }

        position += bytes_read as u64;

        if position > start_pos + 1_048_576 {
            return Ok(start_pos);
        }
    }

    Ok(position)
}

/// Parses a chunk of a JSON file in streaming mode, calling a callback for each valid message.
///
/// # Errors
///
/// Returns `CustomError::IOError` if the file cannot be opened or read.
pub fn parse_chunk_streaming<F>(
    path: &str,
    chunk: &ChunkInfo,
    mut callback: F,
) -> Result<usize, CustomError>
where
    F: FnMut(ChatMessage),
{
    let file = File::open(path).map_err(|e| CustomError::IOError(e.to_string()))?;
    let mut buf_reader = BufReader::new(&file);
    buf_reader
        .seek(SeekFrom::Start(chunk.start))
        .map_err(|e| CustomError::IOError(e.to_string()))?;

    let mut processed_count = 0;
    let mut current_object = String::new();
    let mut brace_count = 0;
    let mut bytes_read = 0;
    let mut in_string = false;
    let mut escape_next = false;

    let mut buffer = vec![0u8; 65536];

    loop {
        if bytes_read >= chunk.size {
            break;
        }

        let remaining_bytes = usize::try_from((chunk.size - bytes_read).min(buffer.len() as u64))
            .unwrap_or(buffer.len());
        let bytes_in_buffer = buf_reader
            .read(&mut buffer[..remaining_bytes])
            .map_err(|e| CustomError::IOError(e.to_string()))?;

        if bytes_in_buffer == 0 {
            break;
        }

        for &byte in buffer.iter().take(bytes_in_buffer) {
            let ch = byte as char;
            bytes_read += 1;

            if escape_next {
                current_object.push(ch);
                escape_next = false;
                continue;
            }

            if ch == '\\' {
                escape_next = true;
                current_object.push(ch);
                continue;
            }

            if ch == '"' {
                in_string = !in_string;
                current_object.push(ch);
                continue;
            }

            if !in_string {
                if (ch == '[' || ch == ']' || ch == ',') && brace_count == 0 {
                    continue;
                }

                if ch == '{' {
                    brace_count += 1;
                } else if ch == '}' {
                    brace_count -= 1;
                }
            }

            if brace_count > 0 || (ch == '}' && !current_object.is_empty()) {
                current_object.push(ch);
            }

            if brace_count == 0
                && !current_object.trim().is_empty()
                && current_object.trim().starts_with('{')
            {
                if let Ok(message) = serde_json::from_str::<ChatMessage>(current_object.trim()) {
                    callback(message);
                    processed_count += 1;
                }

                current_object.clear();
            }

            if bytes_read >= chunk.size {
                break;
            }
        }
    }

    Ok(processed_count)
}
