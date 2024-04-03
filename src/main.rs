use clap::{Parser, Subcommand};
use kikande::bao::game_builder::GameBuilder;
use kikande::config::SearchConfig;
use kikande::error::SearchResult;
use kikande::play;
use kikande::search::negamax;

#[derive(Parser)]
#[command(version, about, long_about = None)]
/// Search for the best move.
struct Cli {
    // Subcommands
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Play a game against the computer.
    Play {
        /// Set the difficulty level [default: 5]
        #[arg(short, long, default_value_t = 5)]
        difficulty: u8,
    },

    /// Search for the best move.
    Search {
        #[arg(short, long)]
        depth: Option<u8>,

        #[arg(short, long)]
        threads: Option<u8>,

        #[arg(short, long)]
        max_time_ms: Option<u32>,
    },
}

fn main() -> SearchResult<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Play { difficulty } => play::play(difficulty),
        Commands::Search {
            depth,
            threads,
            max_time_ms,
        } => {
            // Default search config.
            let mut config = SearchConfig::default();
            config.max_depth = depth.unwrap_or(config.max_depth);
            config.num_threads = threads.unwrap_or(config.num_threads);
            config.max_time_ms = max_time_ms;

            // New game.
            let game = kikande::bao_game!();

            // Start search.
            negamax::search(config, game, true)?;
        }
    };

    Ok(())
}
