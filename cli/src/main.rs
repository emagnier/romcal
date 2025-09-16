use clap::{Parser, Subcommand};
use colored::*;
use romcal_core::{CalendarContext, EasterCalculationType, Preset};
use std::process;

mod commands;
mod enums;
mod error;
mod preset;
mod utils;

// Import command modules
use commands::dates;
use commands::list;
use commands::optimize_preset;
use commands::show_preset;
use error::RomcalCliError;

use crate::enums::{
    CliCalendarContext, CliEasterCalculationType, CliOutputFormat, OutputFormat, ValidationType,
};
use crate::preset::create_preset;

#[derive(Parser)]
#[command(
    name = "romcal",
    about = "A CLI for Catholic liturgical calendars.",
    version = "4.0.0",
    long_about = "Romcal CLI calculates liturgical dates and generates Catholic calendars, \
                  including Easter, Christmas, liturgical seasons, and complete liturgical years."
)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Calendar name to use (e.g., france, united_states)
    #[arg(short = 'c', long, global = true, default_value = "general_roman")]
    calendar: Option<String>,

    /// Locale name to use (e.g., en, fr, es)
    #[arg(short = 'l', long, global = true, default_value = "en")]
    locale: Option<String>,

    /// Output format
    #[arg(short = 'f', long, global = true, value_enum, default_value = "yaml")]
    format: CliOutputFormat,

    /// Calendar context
    #[arg(
        short = 't',
        long,
        global = true,
        value_enum,
        default_value = "gregorian"
    )]
    context: Option<CliCalendarContext>,

    /// Celebrate Epiphany on Sunday
    #[arg(long, global = true, action = clap::ArgAction::SetTrue)]
    epiphany_on_sunday: bool,

    /// Celebrate Ascension on Sunday
    #[arg(long, global = true, action = clap::ArgAction::SetTrue)]
    ascension_on_sunday: bool,

    /// Celebrate Corpus Christi on Sunday
    #[arg(long, global = true, action = clap::ArgAction::SetTrue)]
    corpus_christi_on_sunday: bool,

    /// Easter calculation type
    #[arg(long, global = true, value_enum)]
    easter_calculation_type: Option<CliEasterCalculationType>,

    /// Paths to calendar definition JSON files (supports glob patterns)
    #[arg(short = 'd', long, global = true, value_delimiter = ',')]
    calendar_definitions: Vec<String>,

    /// Paths to resource JSON files (supports glob patterns)
    #[arg(short = 'r', long, global = true, value_delimiter = ',')]
    resources: Vec<String>,

    /// Show debug information
    #[arg(short = 'D', long, global = true)]
    debug: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Calculate liturgical dates
    Dates {
        /// Type of liturgical date to calculate
        /// Available types: easter_sunday, palm_sunday, ash_wednesday, etc.
        date_name: String,

        /// Year for date calculations (default: current year)
        year: Option<i32>,
    },
    /// List available romcal calendars
    ListCalendars,
    /// List available romcal locales
    ListLocales,
    /// Show the current preset information
    Preset,
    /// Optimize the current preset and generate a JSON bundle
    OptimizePreset {
        /// Output file path (if not specified, prints to stdout)
        #[arg(short, long)]
        out: Option<String>,
    },
    /// Validate JSON files against schemas
    Validate {
        /// Type of validation to perform
        #[arg(value_enum)]
        validation_type: ValidationType,
        /// Path(s) or pattern(s) to JSON files to validate
        /// Supports glob patterns (e.g., '*.json', '**/*.json') or multiple file paths
        #[arg(required = true)]
        file_paths: Vec<String>,
    },
}

/// Main entry point for the CLI application
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

/// Execute the CLI command with the provided configuration
fn run(cli: Cli) -> Result<(), RomcalCliError> {
    // Validate and expand glob patterns for calendar definitions and resources
    let calendar_definitions_paths = if !cli.calendar_definitions.is_empty() {
        utils::collect_json_file_paths(&cli.calendar_definitions)?
    } else {
        Vec::new()
    };

    let resources_paths = if !cli.resources.is_empty() {
        utils::collect_json_file_paths(&cli.resources)?
    } else {
        Vec::new()
    };

    // Convert CLI output format to internal format
    let output_format: OutputFormat = OutputFormat::from(cli.format);

    let preset: Preset = create_preset(
        cli.calendar.as_deref(),
        cli.locale.as_deref(),
        cli.context.map(CalendarContext::from),
        cli.easter_calculation_type.map(EasterCalculationType::from),
        Some(cli.epiphany_on_sunday),
        Some(cli.ascension_on_sunday),
        Some(cli.corpus_christi_on_sunday),
        &calendar_definitions_paths,
        &resources_paths,
    )?;

    match cli.command {
        Commands::Dates { date_name, year } => {
            dates::handle(&date_name, year, output_format, preset)
        }
        Commands::ListCalendars => list::handle_calendars(output_format),
        Commands::ListLocales => list::handle_locales(output_format),
        Commands::Preset => show_preset::handle(output_format, preset),
        Commands::OptimizePreset { out } => {
            optimize_preset::handle(preset, out.map(|s| s.to_string()))
        }
        Commands::Validate {
            validation_type,
            file_paths,
        } => commands::validate::handle(validation_type, &file_paths),
    }
}
