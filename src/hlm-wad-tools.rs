use std::env;
use std::process;

mod extractor;
mod repacker;

use extractor::extract_wad;
use repacker::repack_wad;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage(&args[0]);
        process::exit(1);
    }

    let command = &args[1];

    match command.as_str() {
        "extract" => {
            if args.len() < 4 || args.len() > 5 {
                eprintln!("Error: extract requires 2 arguments");
                eprintln!(
                    "Usage: {} extract <input.wad> <output_dir> [--verbose]",
                    args[0]
                );
                process::exit(1);
            }

            let input_wad = &args[2];
            let output_dir = &args[3];
            let verbose = args.len() == 5 && (args[4] == "--verbose" || args[4] == "-v");

            if let Err(e) = extract_wad(input_wad, output_dir, verbose) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
        "repack" => {
            if args.len() < 4 || args.len() > 5 {
                eprintln!("Error: repack requires 2 arguments");
                eprintln!(
                    "Usage: {} repack <input_dir> <output.wad> [--verbose]",
                    args[0]
                );
                process::exit(1);
            }

            let input_dir = &args[2];
            let output_wad = &args[3];
            let verbose = args.len() == 5 && (args[4] == "--verbose" || args[4] == "-v");

            if let Err(e) = repack_wad(input_dir, output_wad, verbose) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
        "help" | "--help" | "-h" => {
            print_usage(&args[0]);
        }
        _ => {
            eprintln!("Error: Unknown command '{}'", command);
            print_usage(&args[0]);
            process::exit(1);
        }
    }
}

fn print_usage(program: &str) {
    eprintln!("WAD Tools - Extract and repack WAD archive files");
    eprintln!();
    eprintln!("Usage:");
    eprintln!("  {} <command> [arguments]", program);
    eprintln!();
    eprintln!("Commands:");
    eprintln!("  extract <input.wad> <output_dir> [--verbose|-v]   Extract WAD file to directory");
    eprintln!("  repack <input_dir> <output.wad> [--verbose|-v]    Repack WAD file from directory");
    eprintln!("  help                                              Show this help message");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  --verbose, -v   Show detailed progress information");
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  {} extract game.wad extracted/", program);
    eprintln!("  {} extract game.wad extracted/ --verbose", program);
    eprintln!("  {} repack extracted/ game_modified.wad", program);
}
