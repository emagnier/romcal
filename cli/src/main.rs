use clap::{Args, CommandFactory, Parser, Subcommand};
use clap_complete::{Shell, generate};
use colored::*;
use romcal_core::{CalendarContext, EasterCalculationType, Romcal};
use std::io;
use std::path::PathBuf;
use std::process;

mod commands;
mod config;
mod enums;
mod error;
mod preset;
mod utils;

use commands::bundle;
use commands::calendar;
use commands::date;
use commands::list;
use commands::masses;
use commands::show_preset;
use error::RomcalCliError;

use crate::config::Config;
use crate::enums::FieldPath;
use crate::enums::{CliCalendarContext, CliEasterCalculationType, CliOutputFormat, ValidationType};
use crate::preset::create_romcal;

/// Config file flag
#[derive(Args, Clone, Default)]
struct ConfigArgs {
    /// Path to config file (default: .romcal.toml or ~/.config/romcal/config.toml)
    #[arg(long = "config", short = 'C', global = true)]
    config_path: Option<PathBuf>,
}

/// Preset-related flags
#[derive(Args, Clone, Default)]
struct PresetArgs {
    /// Calendar name to use (e.g., france, united_states)
    #[arg(short = 'c', long)]
    calendar: Option<String>,

    /// Locale name to use (e.g., en, fr, es)
    #[arg(short = 'l', long)]
    locale: Option<String>,

    /// Calendar context
    #[arg(short = 't', long, value_enum)]
    context: Option<CliCalendarContext>,

    /// Easter calculation type
    #[arg(long = "easter-calc", value_enum)]
    easter_calculation_type: Option<CliEasterCalculationType>,

    /// Celebrate Epiphany on Sunday
    #[arg(long)]
    epiphany_on_sunday: bool,

    /// Celebrate Ascension on Sunday
    #[arg(long)]
    ascension_on_sunday: bool,

    /// Celebrate Corpus Christi on Sunday
    #[arg(long)]
    corpus_christi_on_sunday: bool,

    /// Paths to calendar definition JSON files (supports glob patterns)
    #[arg(short = 'd', long = "definitions", value_delimiter = ',')]
    calendar_definitions: Vec<String>,

    /// Paths to resource JSON files (supports glob patterns)
    #[arg(short = 'r', long, value_delimiter = ',')]
    resources: Vec<String>,
}

impl PresetArgs {
    /// Merge CLI args with config, CLI takes priority
    fn into_romcal(self, config: &Config) -> Result<Romcal, RomcalCliError> {
        // CLI args take priority over config
        let calendar = self.calendar.as_deref().or(config.calendar.as_deref());
        let locale = self.locale.as_deref().or(config.locale.as_deref());

        let context = self.context.map(CalendarContext::from).or_else(|| {
            config.context.as_ref().and_then(|s| match s.as_str() {
                "gregorian" => Some(CalendarContext::Gregorian),
                "liturgical" => Some(CalendarContext::Liturgical),
                _ => None,
            })
        });

        let easter_calc = self
            .easter_calculation_type
            .map(EasterCalculationType::from)
            .or_else(|| {
                config
                    .easter_calculation_type
                    .as_ref()
                    .and_then(|s| match s.as_str() {
                        "gregorian" => Some(EasterCalculationType::Gregorian),
                        "julian" => Some(EasterCalculationType::Julian),
                        _ => None,
                    })
            });

        // For booleans: CLI true overrides, otherwise use config
        let epiphany_on_sunday =
            self.epiphany_on_sunday || config.epiphany_on_sunday.unwrap_or(false);
        let ascension_on_sunday =
            self.ascension_on_sunday || config.ascension_on_sunday.unwrap_or(false);
        let corpus_christi_on_sunday =
            self.corpus_christi_on_sunday || config.corpus_christi_on_sunday.unwrap_or(false);

        let definitions = if !self.calendar_definitions.is_empty() {
            utils::collect_json_file_paths(&self.calendar_definitions)?
        } else {
            Vec::new()
        };
        let resources = if !self.resources.is_empty() {
            utils::collect_json_file_paths(&self.resources)?
        } else {
            Vec::new()
        };

        create_romcal(
            calendar,
            locale,
            context,
            easter_calc,
            Some(epiphany_on_sunday),
            Some(ascension_on_sunday),
            Some(corpus_christi_on_sunday),
            &definitions,
            &resources,
        )
    }
}

/// Output format flag
#[derive(Args, Clone, Default)]
struct OutputArgs {
    /// Output format
    #[arg(short = 'f', long, value_enum)]
    format: Option<CliOutputFormat>,
}

impl OutputArgs {
    /// Get format, with config fallback and default
    fn get_format(self, config: &Config) -> CliOutputFormat {
        self.format.unwrap_or_else(|| {
            config
                .format
                .as_ref()
                .and_then(|s| match s.as_str() {
                    "yaml" => Some(CliOutputFormat::Yaml),
                    "json" => Some(CliOutputFormat::Json),
                    "csv" => Some(CliOutputFormat::Csv),
                    "lines" => Some(CliOutputFormat::Lines),
                    _ => None,
                })
                .unwrap_or(CliOutputFormat::Yaml)
        })
    }
}

/// Debug flag
#[derive(Args, Clone)]
struct DebugArgs {
    /// Show debug information
    #[arg(short = 'D', long)]
    debug: bool,
}

impl DebugArgs {
    fn init(&self) {
        if self.debug {
            env_logger::Builder::from_default_env()
                .filter_level(log::LevelFilter::Debug)
                .init();
        }
    }
}

#[derive(Parser)]
#[command(
    name = "romcal",
    about = "A CLI for calculating Catholic liturgical dates and generating calendars.",
    version = "4.0.0",
    long_about = "Calculate liturgical dates and generate liturgical or mass-centric calendars, \
                  with support for multiple locales and regional calendars."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[command(flatten)]
    config: ConfigArgs,
}

#[derive(Subcommand)]
enum Commands {
    /// Calculate a liturgical date
    Date {
        /// Type of liturgical date to calculate
        /// Available types: easter_sunday, palm_sunday, ash_wednesday, etc.
        date_name: String,

        /// Year for date calculations (default: current year)
        year: Option<i32>,

        #[command(flatten)]
        preset: PresetArgs,

        #[command(flatten)]
        output: OutputArgs,

        #[command(flatten)]
        debug: DebugArgs,
    },
    /// Generate liturgical calendar (organized by liturgical date)
    Calendar {
        /// Year for calendar generation (default: current year)
        year: Option<i32>,

        /// Filter to show only specific properties (supports dot notation for nested fields)
        /// Examples: id, fullname, colors.key, entities.name
        #[arg(long, value_delimiter = ',')]
        filter: Option<Vec<FieldPath>>,

        #[command(flatten)]
        preset: PresetArgs,

        #[command(flatten)]
        output: OutputArgs,

        #[command(flatten)]
        debug: DebugArgs,
    },
    /// Generate mass-centric calendar (organized by civil date and mass time)
    Masses {
        /// Year for mass calendar generation (default: current year)
        year: Option<i32>,

        /// Filter to show only specific properties (supports dot notation for nested fields)
        /// Examples: mass_time, civil_date, optional_celebrations.id
        #[arg(long, value_delimiter = ',')]
        filter: Option<Vec<FieldPath>>,

        #[command(flatten)]
        preset: PresetArgs,

        #[command(flatten)]
        output: OutputArgs,

        #[command(flatten)]
        debug: DebugArgs,
    },
    /// List available calendars and locales
    List {
        #[command(subcommand)]
        element: ListCommand,

        #[command(flatten)]
        output: OutputArgs,
    },
    /// Display current calendar configuration
    Preset {
        #[command(flatten)]
        preset: PresetArgs,

        #[command(flatten)]
        output: OutputArgs,

        #[command(flatten)]
        debug: DebugArgs,
    },
    /// Bundle required data (definitions + resources) for the current preset
    Bundle {
        #[command(flatten)]
        preset: PresetArgs,

        #[command(flatten)]
        debug: DebugArgs,
    },
    /// Validate calendar and resource JSON files
    Validate {
        #[command(subcommand)]
        validation_type: ValidationCommand,
    },
    /// Generate shell completion scripts
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[derive(Subcommand)]
enum ListCommand {
    /// List all available romcal calendars
    Calendars {
        /// Display calendars as a tree structure
        #[arg(long)]
        tree: bool,
    },
    /// List all available romcal locales
    Locales {
        /// Display locales as a tree structure
        #[arg(long)]
        tree: bool,
    },
}

#[derive(Subcommand)]
enum ValidationCommand {
    /// Validate calendar definition JSON files
    Definitions {
        /// Path(s) or pattern(s) to JSON files to validate
        /// Supports glob patterns (e.g., '*.json', '**/*.json') or multiple file paths
        #[arg(required = true)]
        file_paths: Vec<String>,
    },
    /// Validate resource JSON files
    Resources {
        /// Path(s) or pattern(s) to JSON files to validate
        /// Supports glob patterns (e.g., '*.json', '**/*.json') or multiple file paths
        #[arg(required = true)]
        file_paths: Vec<String>,
    },
}

/// Main entry point for the CLI application
fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("{} {}", "Error:".red().bold(), e);
        process::exit(1);
    }
}

/// Execute the CLI command with the provided configuration
fn run(cli: Cli) -> Result<(), RomcalCliError> {
    let config = Config::load(cli.config.config_path.as_deref())?;

    match cli.command {
        Commands::Date {
            date_name,
            year,
            preset,
            output,
            debug,
        } => {
            debug.init();
            date::handle(
                &date_name,
                year,
                output.get_format(&config).into(),
                preset.into_romcal(&config)?,
            )
        }
        Commands::Calendar {
            year,
            filter,
            preset,
            output,
            debug,
        } => {
            debug.init();
            calendar::handle(
                year,
                filter,
                preset.into_romcal(&config)?,
                output.get_format(&config).into(),
            )
        }
        Commands::Masses {
            year,
            filter,
            preset,
            output,
            debug,
        } => {
            debug.init();
            masses::handle(
                year,
                filter,
                preset.into_romcal(&config)?,
                output.get_format(&config).into(),
            )
        }
        Commands::List { element, output } => match element {
            ListCommand::Calendars { tree } => {
                list::handle_calendars(output.get_format(&config).into(), tree)
            }
            ListCommand::Locales { tree } => {
                list::handle_locales(output.get_format(&config).into(), tree)
            }
        },
        Commands::Preset {
            preset,
            output,
            debug,
        } => {
            debug.init();
            show_preset::handle(
                output.get_format(&config).into(),
                preset.into_romcal(&config)?,
            )
        }
        Commands::Bundle { preset, debug } => {
            debug.init();
            bundle::handle(preset.into_romcal(&config)?)
        }
        Commands::Validate { validation_type } => match validation_type {
            ValidationCommand::Definitions { file_paths } => {
                commands::validate::handle(ValidationType::Definitions, &file_paths)
            }
            ValidationCommand::Resources { file_paths } => {
                commands::validate::handle(ValidationType::Resources, &file_paths)
            }
        },
        Commands::Completions { shell } => {
            generate(shell, &mut Cli::command(), "romcal", &mut io::stdout());
            Ok(())
        }
    }
}
