use crate::channel_message_count::ChannelMessageCount;
use crate::chat_message::ChatMessage;
use crate::custom_error::CustomError;
use crate::language_message_count::LanguageMessageCount;
use crate::parser::{analize_file_for_chunks, parse_chunk_streaming};
use crate::streaming_aggregators::StreamingAggregators;
use crate::top_channels_result::TopChannelsByLanguageResult;
use crate::top_languages_result::TopLanguagesByViewerRangeResult;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
};

impl Default for StreamingAggregators {
    fn default() -> Self {
        Self::new()
    }
}

impl StreamingAggregators {
    /// Creates a new instance of `StreamingAggregators` with empty collections.
    ///
    /// Initializes all the thread-safe data structures (`Arc<Mutex<HashMap>>`)
    /// needed for parallel data aggregation.
    #[must_use]
    pub fn new() -> Self {
        Self {
            language_channel_counts: Arc::new(Mutex::new(HashMap::new())),
            range_language_counts: Arc::new(Mutex::new(HashMap::new())),
            total_messages: Arc::new(Mutex::new(0)),
        }
    }

    /// Processes a single chat message and updates the aggregated counts.
    ///
    /// This method is thread-safe and updates both language-channel counts
    /// and viewer range-language counts atomically. It categorizes viewer counts
    /// into ranges: 0-100, 101-500, 501-1000, and 1000+.
    ///
    /// # Arguments
    ///
    /// * `message` - The `ChatMessage` to process and aggregate
    ///
    /// # Panics
    ///
    /// Panics if any of the internal mutexes are poisoned due to panic in another thread.
    pub fn process_message(&self, message: ChatMessage) {
        {
            let mut lang_channel_counts = self.language_channel_counts.lock().unwrap();
            let channel_counts = lang_channel_counts
                .entry(message.language.clone())
                .or_default();
            *channel_counts
                .entry(message.channel_name.clone())
                .or_insert(0) += 1;
        }

        let range = match message.viewer_count {
            0..=100 => "0-100",
            101..=500 => "101-500",
            501..=1000 => "501-1000",
            _ => "1000+",
        };

        {
            let mut range_lang_counts = self.range_language_counts.lock().unwrap();
            let language_counts = range_lang_counts.entry(range.to_string()).or_default();
            *language_counts.entry(message.language).or_insert(0) += 1;
        }

        {
            let mut total = self.total_messages.lock().unwrap();
            *total += 1;
        }
    }
}

/// Processes a file using streaming approach with parallel chunk processing.
///
/// This function divides the file into chunks and processes them in parallel using Rayon,
/// maintaining real-time progress updates and thread-safe aggregation.
///
/// # Arguments
///
/// * `path` - Path to the file to process
/// * `num_threads` - Number of threads to use for parallel processing
///
/// # Returns
///
/// Returns a `StreamingAggregators` containing all aggregated data from the file processing.
///
/// # Errors
///
/// Returns `CustomError` if file analysis or chunk processing fails.
fn process_file_streaming(
    path: &str,
    num_threads: usize,
) -> Result<StreamingAggregators, CustomError> {
    println!("Starting streaming analysis process...");
    println!("File: {path}");
    println!("Using {num_threads} threads");

    let chunks = analize_file_for_chunks(path, num_threads)?;
    let chunks_len = chunks.len();
    println!("File divided into {chunks_len} chunks for parallel processing");

    let aggregators = StreamingAggregators::new();
    let processed_chunks = Arc::new(AtomicUsize::new(0));
    let total_chunks = chunks.len();

    chunks
        .par_iter()
        .try_for_each(|chunk| -> Result<(), CustomError> {
            let aggregators_ref = &aggregators;
            let processed_count = parse_chunk_streaming(path, chunk, |message| {
                aggregators_ref.process_message(message);
            })?;

            let completed = processed_chunks.fetch_add(1, Ordering::SeqCst) + 1;
            let percentage = (completed * 100) / total_chunks;
            println!("Chunk {completed}/{total_chunks} completed ({processed_count} messages processed) - {percentage}% done");
            Ok(())
        })?;

    let total = *aggregators.total_messages.lock().unwrap();
    println!("Processing completed! Total messages processed: {total}");
    Ok(aggregators)
}

/// Generates results for top channels by language from aggregated data.
///
/// Processes the language-channel counts from the aggregators and returns
/// the top 3 channels for each language, sorted by message count in descending order.
///
/// # Arguments
///
/// * `aggregators` - Reference to the `StreamingAggregators` containing the data
///
/// # Returns
///
/// A vector of `TopChannelsByLanguageResult` containing the top channels for each language.
fn generate_top_channels_results(
    aggregators: &StreamingAggregators,
) -> Vec<TopChannelsByLanguageResult> {
    let language_channel_counts = aggregators.language_channel_counts.lock().unwrap();

    language_channel_counts
        .iter()
        .map(|(language, channel_counts)| {
            let mut channels: Vec<(String, i32)> = channel_counts
                .iter()
                .map(|(k, v)| (k.clone(), *v))
                .collect();
            channels.sort_by(|a, b| b.1.cmp(&a.1));
            channels.truncate(3);

            let top_channels = channels
                .into_iter()
                .map(|(channel_name, message_count)| ChannelMessageCount {
                    channel_name,
                    message_count,
                })
                .collect();

            TopChannelsByLanguageResult {
                language: language.clone(),
                top_channels,
            }
        })
        .collect()
}

/// Generates results for top languages by viewer range from aggregated data.
///
/// Processes the viewer range-language counts from the aggregators and returns
/// the top 5 languages for each viewer range, sorted by message count in descending order.
/// Results are sorted by viewer range in logical order (0-100, 101-500, 501-1000, 1000+).
///
/// # Arguments
///
/// * `aggregators` - Reference to the `StreamingAggregators` containing the data
///
/// # Returns
///
/// A vector of `TopLanguagesByViewerRangeResult` containing the top languages for each viewer range.
fn generate_top_languages_results(
    aggregators: &StreamingAggregators,
) -> Vec<TopLanguagesByViewerRangeResult> {
    let range_language_counts = aggregators.range_language_counts.lock().unwrap();

    let mut results: Vec<TopLanguagesByViewerRangeResult> = range_language_counts
        .iter()
        .map(|(viewer_range, language_counts)| {
            let mut languages: Vec<(String, i32)> = language_counts
                .iter()
                .map(|(k, v)| (k.clone(), *v))
                .collect();
            languages.sort_by(|a, b| b.1.cmp(&a.1));
            languages.truncate(5);

            let top_languages = languages
                .into_iter()
                .map(|(language, message_count)| LanguageMessageCount {
                    language,
                    message_count,
                })
                .collect();

            TopLanguagesByViewerRangeResult {
                viewer_range: viewer_range.clone(),
                top_languages,
            }
        })
        .collect();

    results.sort_by(|a, b| {
        let order_a = match a.viewer_range.as_str() {
            "0-100" => 0,
            "101-500" => 1,
            "501-1000" => 2,
            "1000+" => 3,
            _ => 999,
        };
        let order_b = match b.viewer_range.as_str() {
            "0-100" => 0,
            "101-500" => 1,
            "501-1000" => 2,
            "1000+" => 3,
            _ => 999,
        };
        order_a.cmp(&order_b)
    });

    results
}

/// Analyzes a dataset and returns both top channels by language and top languages by viewer range.
///
/// # Errors
///
/// Returns `CustomError` if file processing fails or if the file cannot be read.
pub fn analyze_both(
    path: &str,
    num_threads: usize,
) -> Result<
    (
        Vec<TopChannelsByLanguageResult>,
        Vec<TopLanguagesByViewerRangeResult>,
    ),
    CustomError,
> {
    let aggregators = process_file_streaming(path, num_threads)?;

    println!("Generating results...");
    println!("Processing top channels by language...");
    let top_channels = generate_top_channels_results(&aggregators);

    println!("Processing top languages by viewer range...");
    let top_languages = generate_top_languages_results(&aggregators);

    println!("Analysis complete!");
    Ok((top_channels, top_languages))
}

/// Analyzes a dataset and returns the top 3 channels by language.
///
/// # Errors
///
/// Returns `CustomError` if file processing fails or if the file cannot be read.
pub fn top_channels_by_language(
    path: &str,
    num_threads: usize,
) -> Result<Vec<TopChannelsByLanguageResult>, CustomError> {
    let aggregators = process_file_streaming(path, num_threads)?;
    println!("Generating top channels by language results...");
    let results = generate_top_channels_results(&aggregators);
    println!("Analysis complete!");
    Ok(results)
}

/// Analyzes a dataset and returns the top 5 languages by viewer range.
///
/// # Errors
///
/// Returns `CustomError` if file processing fails or if the file cannot be read.
pub fn top_languages_by_viewer_range(
    path: &str,
    num_threads: usize,
) -> Result<Vec<TopLanguagesByViewerRangeResult>, CustomError> {
    let aggregators = process_file_streaming(path, num_threads)?;
    println!("Generating top languages by viewer range results...");
    let results = generate_top_languages_results(&aggregators);
    println!("Analysis complete!");
    Ok(results)
}
