use clap::{Parser, Subcommand, ValueEnum};
use colored::*;
use romcal_core::LiturgicalConfig;
use std::process;

mod commands;
mod config;
mod error;
mod output;
mod utils;

// Import command modules
use commands::dates;
use commands::generate_bundle;
use commands::list;
use commands::output_config;
use error::RomcalCliError;

use crate::config::create_liturgical_config;
use crate::output::validate_format;
use crate::output::OutputFormat;

/// Type of validation to perform
#[derive(ValueEnum, Clone, Debug)]
pub enum ValidationType {
    /// Validate calendar definition JSON file
    CalendarDef,
    /// Validate resource JSON file
    Resource,
}

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

    /// Calendar to use (e.g., 'general_roman', 'france', 'united_states')
    #[arg(short, long, global = true)]
    calendar: Option<String>,

    /// Locale to use (e.g., 'en', 'fr', 'es')
    #[arg(short, long, global = true)]
    locale: Option<String>,

    /// Output format (json, csv, yaml, lines)
    #[arg(short, long, global = true, default_value = "yaml")]
    format: String,

    /// Calendar scope (gregorian, liturgical)
    #[arg(short, long, global = true)]
    scope: Option<String>,

    /// Easter calculation type (gregorian, julian)
    #[arg(long, global = true)]
    easter_calculation_type: Option<String>,

    /// Celebrate Ascension on Sunday
    #[arg(long, global = true, action = clap::ArgAction::SetTrue)]
    ascension_on_sunday: bool,

    /// Celebrate Epiphany on Sunday
    #[arg(long, global = true, action = clap::ArgAction::SetTrue)]
    epiphany_on_sunday: bool,

    /// Celebrate Corpus Christi on Sunday
    #[arg(long, global = true, action = clap::ArgAction::SetTrue)]
    corpus_christi_on_sunday: bool,

    /// Paths to calendar definition JSON files (supports glob patterns)
    #[arg(long, global = true, value_delimiter = ',')]
    calendar_definitions: Vec<String>,

    /// Paths to resource JSON files (supports glob patterns)
    #[arg(long, global = true, value_delimiter = ',')]
    resources: Vec<String>,

    /// Show debug information
    #[arg(short, long, global = true)]
    debug: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Calculate liturgical dates
    Dates {
        /// Type of liturgical date to calculate (e.g., easter_sunday, palm_sunday)
        date_type: String,

        /// Year for date calculations (default: current year)
        year: Option<i32>,
    },
    /// List available Romcal calendars
    ListCalendars,
    /// List available Romcal locales
    ListLocales,
    /// Display configuration information
    Config,
    /// Generate a JSON bundle of the current configuration
    GenerateBundle {
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
        files: Vec<String>,
    },
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

    // Validate and parse output format
    let format = cli.format.to_lowercase();
    validate_format(&format)?;
    let output_format: OutputFormat = match format.as_str() {
        "json" => OutputFormat::Json,
        "csv" => OutputFormat::Csv,
        "yaml" => OutputFormat::Yaml,
        "lines" => OutputFormat::Lines,
        _ => unreachable!(), // Already validated above
    };

    let liturgical_config: LiturgicalConfig = create_liturgical_config(
        cli.calendar.as_deref(),
        cli.locale.as_deref(),
        cli.scope.as_deref(),
        cli.easter_calculation_type.as_deref(),
        Some(cli.ascension_on_sunday),
        Some(cli.corpus_christi_on_sunday),
        Some(cli.epiphany_on_sunday),
        &calendar_definitions_paths,
        &resources_paths,
    )?;

    match cli.command {
        Commands::Dates { date_type, year } => {
            dates::handle_dates(&date_type, year, output_format, liturgical_config)
        }
        Commands::ListCalendars => list::handle_calendars(output_format),
        Commands::ListLocales => list::handle_locales(output_format),
        Commands::Config => output_config::handle_output_config(output_format, liturgical_config),
        Commands::GenerateBundle { out } => {
            generate_bundle::handle_generate_bundle(liturgical_config, out.map(|s| s.to_string()))
        }
        Commands::Validate {
            validation_type,
            files,
        } => commands::validate::handle_validate(validation_type, &files),
    }
}
