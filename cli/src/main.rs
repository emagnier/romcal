use clap::{Parser, Subcommand, ValueEnum};
use colored::*;
use std::process;

mod commands;
mod config;
mod error;
mod output;

// Import command modules
use commands::config as config_cmd;
use commands::dates;
use commands::generate_bundle;
use commands::list;
use error::RomcalCliError;

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
    // Extract common parameters to avoid duplication
    let common_params = CommonParams {
        calendar: cli.calendar.as_deref(),
        locale: cli.locale.as_deref(),
        format: &cli.format,
        scope: cli.scope.as_deref(),
        easter_calculation_type: cli.easter_calculation_type.as_deref(),
        ascension_on_sunday: Some(cli.ascension_on_sunday),
        epiphany_on_sunday: Some(cli.epiphany_on_sunday),
        corpus_christi_on_sunday: Some(cli.corpus_christi_on_sunday),
    };

    match cli.command {
        Commands::Dates { date_type, year } => dates::handle(
            &date_type,
            year,
            common_params.calendar,
            common_params.locale,
            common_params.format,
            common_params.scope,
            common_params.easter_calculation_type,
            common_params.ascension_on_sunday,
            common_params.epiphany_on_sunday,
            common_params.corpus_christi_on_sunday,
        ),
        Commands::ListCalendars => list::handle_calendars(common_params.format),
        Commands::ListLocales => list::handle_locales(common_params.format),
        Commands::Config => config_cmd::handle(config_cmd::ConfigParams {
            calendar: common_params.calendar.map(|s| s.to_string()),
            locale: common_params.locale.map(|s| s.to_string()),
            format: common_params.format.to_string(),
            scope: common_params.scope.map(|s| s.to_string()),
            easter_calculation_type: common_params.easter_calculation_type.map(|s| s.to_string()),
            ascension_on_sunday: common_params.ascension_on_sunday,
            epiphany_on_sunday: common_params.epiphany_on_sunday,
            corpus_christi_on_sunday: common_params.corpus_christi_on_sunday,
        }),
        Commands::GenerateBundle { out } => {
            generate_bundle::handle_generate_bundle(generate_bundle::GenerateBundleParams {
                calendar: common_params.calendar.map(|s| s.to_string()),
                locale: common_params.locale.map(|s| s.to_string()),
                scope: common_params.scope.map(|s| s.to_string()),
                easter_calculation_type: common_params
                    .easter_calculation_type
                    .map(|s| s.to_string()),
                ascension_on_sunday: common_params.ascension_on_sunday,
                epiphany_on_sunday: common_params.epiphany_on_sunday,
                corpus_christi_on_sunday: common_params.corpus_christi_on_sunday,
                output_file: out.map(|s| s.to_string()),
            })
        }
        Commands::Validate {
            validation_type,
            files,
        } => commands::validate::handle_validate(validation_type, &files),
    }
}

/// Common parameters shared across commands
struct CommonParams<'a> {
    calendar: Option<&'a str>,
    locale: Option<&'a str>,
    format: &'a str,
    scope: Option<&'a str>,
    easter_calculation_type: Option<&'a str>,
    ascension_on_sunday: Option<bool>,
    epiphany_on_sunday: Option<bool>,
    corpus_christi_on_sunday: Option<bool>,
}
