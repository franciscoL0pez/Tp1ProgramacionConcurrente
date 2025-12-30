use ::std::env;
use TP0ProgramacionConcurrente::transformations::{
    analyze_both, top_channels_by_language, top_languages_by_viewer_range,
};
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 || args.len() > 4 {
        eprintln!(
            "Usage: {} <file_path> <num_threads> [analysis_type]",
            args[0]
        );
        eprintln!("analysis_type: top_channels | top_languages | both (default: both)");
        std::process::exit(1);
    }

    let file_path = &args[1];
    let num_threads: usize = args[2]
        .parse()
        .expect("Please provide a valid number for threads");

    let analysis_type = if args.len() == 4 { &args[3] } else { "both" };

    let start = Instant::now();

    match analysis_type {
        "top_channels" => match top_channels_by_language(file_path, num_threads) {
            Ok(results) => {
                println!("\n=== TOP 3 CHANNELS BY LANGUAGE ===");
                for result in results {
                    println!("\nLanguage: {}", result.language);
                    for (i, channel) in result.top_channels.iter().enumerate() {
                        println!(
                            "  {}. {}: {} messages",
                            i + 1,
                            channel.channel_name,
                            channel.message_count
                        );
                    }
                }
            }
            Err(e) => eprintln!("Error: {e}"),
        },
        "top_languages" => match top_languages_by_viewer_range(file_path, num_threads) {
            Ok(results) => {
                println!("\n=== TOP 5 LANGUAGES BY VIEWER RANGE ===");
                for result in results {
                    println!("\nViewer Range: {}", result.viewer_range);
                    for (i, language) in result.top_languages.iter().enumerate() {
                        println!(
                            "  {}. {}: {} messages",
                            i + 1,
                            language.language,
                            language.message_count
                        );
                    }
                }
            }
            Err(e) => eprintln!("Error: {e}"),
        },
        "both" => match analyze_both(file_path, num_threads) {
            Ok((top_channels, top_languages)) => {
                println!("\n=== TOP 3 CHANNELS BY LANGUAGE ===");
                for result in top_channels {
                    println!("\nLanguage: {}", result.language);
                    for (i, channel) in result.top_channels.iter().enumerate() {
                        println!(
                            "  {}. {}: {} messages",
                            i + 1,
                            channel.channel_name,
                            channel.message_count
                        );
                    }
                }

                println!("\n=== TOP 5 LANGUAGES BY VIEWER RANGE ===");
                for result in top_languages {
                    println!("\nViewer Range: {}", result.viewer_range);
                    for (i, language) in result.top_languages.iter().enumerate() {
                        println!(
                            "  {}. {}: {} messages",
                            i + 1,
                            language.language,
                            language.message_count
                        );
                    }
                }
            }
            Err(e) => eprintln!("Error: {e}"),
        },
        _ => {
            eprintln!("Invalid analysis type. Use: top_channels, top_languages, or both");
            std::process::exit(1);
        }
    }

    let duration = start.elapsed();
    println!("\nTime elapsed: {duration:?}");
}
