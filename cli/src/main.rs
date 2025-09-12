use clap::{Parser, Subcommand};
use colored::*;
use std::process;

mod commands;
mod config;
mod error;
mod output;

// Import command modules
use commands::config as config_cmd;
use commands::dates;
use commands::list;
use error::RomcalCliError;

#[derive(Parser)]
#[command(
    name = "romcal",
    about = "Catholic liturgical calendar calculator",
    version = "4.0.0",
    long_about = "Romcal CLI allows you to calculate important liturgical dates of the Catholic calendar, \
                  including Easter, Christmas, liturgical seasons and much more."
)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Show debug information
    #[arg(short, long, global = true)]
    debug: bool,

    /// Output format (json, csv, yaml, lines)
    #[arg(short, long, global = true, default_value = "yaml")]
    format: String,

    /// Easter calculation type (gregorian, julian)
    #[arg(long, global = true, default_value = "gregorian")]
    easter_calculation_type: String,

    /// Calendar scope (gregorian, liturgical)
    #[arg(long, global = true, default_value = "gregorian")]
    scope: String,

    /// Celebrate Ascension on Sunday
    #[arg(long, global = true)]
    ascension_on_sunday: bool,

    /// Celebrate Epiphany on Sunday
    #[arg(long, global = true)]
    epiphany_on_sunday: bool,

    /// Celebrate Corpus Christi on Sunday
    #[arg(long, global = true)]
    corpus_christi_on_sunday: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Calculate liturgical dates
    Dates {
        #[command(subcommand)]
        date_command: dates::DateCommands,

        /// Year for date calculations (default: current year)
        #[arg(short, long)]
        year: Option<i32>,
    },
    /// List available Romcal calendars
    ListCalendars,
    /// List available Romcal locales
    ListLocales,
    /// Display configuration information
    Config,
}

fn main() {
    let cli = Cli::parse();

    if cli.debug {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Debug)
            .init();
    }

    if let Err(e) = run(cli) {
        eprintln!("{} {}", "Error:".red().bold(), e);
        process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), RomcalCliError> {
    match cli.command {
        Commands::Dates { date_command, year } => dates::handle(
            date_command,
            year,
            &cli.format,
            &cli.easter_calculation_type,
            &cli.scope,
            cli.ascension_on_sunday,
            cli.epiphany_on_sunday,
            cli.corpus_christi_on_sunday,
        ),
        Commands::ListCalendars => list::handle_calendars(&cli.format),
        Commands::ListLocales => list::handle_locales(&cli.format),
        Commands::Config => config_cmd::handle(
            &cli.format,
            &cli.easter_calculation_type,
            &cli.scope,
            cli.ascension_on_sunday,
            cli.epiphany_on_sunday,
            cli.corpus_christi_on_sunday,
        ),
    }
}
