use clap::{Args, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use colored::*;
use romcal_core::{CalendarContext, EasterCalculationType, Preset};
use std::io;
use std::path::PathBuf;
use std::process;

mod commands;
mod config;
mod enums;
mod error;
mod preset;
mod utils;

// Import command modules
use commands::dates;
use commands::days;
use commands::list;
use commands::optimize_preset;
use commands::show_preset;
use error::RomcalCliError;

use crate::config::Config;
use crate::enums::liturgical_day_filter::LiturgicalDayFilterWrapper;
use crate::enums::{
    CliCalendarContext, CliEasterCalculationType, CliOutputFormat, ValidationType,
};
use crate::preset::create_preset;

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
    fn into_preset(self, config: &Config) -> Result<Preset, RomcalCliError> {
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
        let epiphany_on_sunday = self.epiphany_on_sunday
            || config.epiphany_on_sunday.unwrap_or(false);
        let ascension_on_sunday = self.ascension_on_sunday
            || config.ascension_on_sunday.unwrap_or(false);
        let corpus_christi_on_sunday = self.corpus_christi_on_sunday
            || config.corpus_christi_on_sunday.unwrap_or(false);

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

        create_preset(
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
    about = "A CLI for Catholic liturgical calendars.",
    version = "4.0.0",
    long_about = "Romcal CLI calculates liturgical dates and generates Catholic calendars, \
                  including Easter, Christmas, liturgical seasons, and complete liturgical years."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[command(flatten)]
    config: ConfigArgs,
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

        #[command(flatten)]
        preset: PresetArgs,

        #[command(flatten)]
        output: OutputArgs,

        #[command(flatten)]
        debug: DebugArgs,
    },
    /// Generate liturgical days for the Proper of Time
    Days {
        /// Year for liturgical days generation (default: current year)
        year: Option<i32>,

        /// Filter to show only specific properties of liturgical days
        /// Can be specified multiple times to include multiple properties
        #[arg(long, value_delimiter = ',')]
        filter: Option<Vec<LiturgicalDayFilterWrapper>>,

        #[command(flatten)]
        preset: PresetArgs,

        #[command(flatten)]
        output: OutputArgs,

        #[command(flatten)]
        debug: DebugArgs,
    },
    /// List various romcal elements
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
    /// Optimize the current preset and generate a JSON bundle
    OptimizePreset {
        /// Output file path (if not specified, prints to stdout)
        #[arg(short, long)]
        out: Option<String>,

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
    // Load config file
    let config = Config::load(cli.config.config_path.as_deref())?;

    match cli.command {
        Commands::Dates {
            date_name,
            year,
            preset,
            output,
            debug,
        } => {
            debug.init();
            dates::handle(
                &date_name,
                year,
                output.get_format(&config).into(),
                preset.into_preset(&config)?,
            )
        }
        Commands::Days {
            year,
            filter,
            preset,
            output,
            debug,
        } => {
            debug.init();
            let converted_filter =
                filter.map(|filters| filters.into_iter().map(|wrapper| wrapper.0).collect());
            days::handle(
                year,
                converted_filter,
                preset.into_preset(&config)?,
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
            show_preset::handle(output.get_format(&config).into(), preset.into_preset(&config)?)
        }
        Commands::OptimizePreset { out, preset, debug } => {
            debug.init();
            optimize_preset::handle(preset.into_preset(&config)?, out)
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
