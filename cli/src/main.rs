use clap::{Args, Parser, Subcommand};
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
use commands::days;
use commands::list;
use commands::optimize_preset;
use commands::show_preset;
use error::RomcalCliError;

use crate::enums::liturgical_day_filter::LiturgicalDayFilterWrapper;
use crate::enums::{
    CliCalendarContext, CliEasterCalculationType, CliOutputFormat, ValidationType,
};
use crate::preset::create_preset;

/// Preset-related flags
#[derive(Args, Clone)]
struct PresetArgs {
    /// Calendar name to use (e.g., france, united_states)
    #[arg(short = 'c', long, default_value = "general_roman")]
    calendar: Option<String>,

    /// Locale name to use (e.g., en, fr, es)
    #[arg(short = 'l', long, default_value = "en")]
    locale: Option<String>,

    /// Calendar context
    #[arg(short = 't', long, value_enum, default_value = "gregorian")]
    context: Option<CliCalendarContext>,

    /// Easter calculation type
    #[arg(long = "easter-calc", value_enum)]
    easter_calculation_type: Option<CliEasterCalculationType>,

    /// Celebrate Epiphany on Sunday
    #[arg(long, action = clap::ArgAction::SetTrue)]
    epiphany_on_sunday: bool,

    /// Celebrate Ascension on Sunday
    #[arg(long, action = clap::ArgAction::SetTrue)]
    ascension_on_sunday: bool,

    /// Celebrate Corpus Christi on Sunday
    #[arg(long, action = clap::ArgAction::SetTrue)]
    corpus_christi_on_sunday: bool,

    /// Paths to calendar definition JSON files (supports glob patterns)
    #[arg(short = 'd', long = "definitions", value_delimiter = ',')]
    calendar_definitions: Vec<String>,

    /// Paths to resource JSON files (supports glob patterns)
    #[arg(short = 'r', long, value_delimiter = ',')]
    resources: Vec<String>,
}

impl PresetArgs {
    fn to_preset(self) -> Result<Preset, RomcalCliError> {
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
            self.calendar.as_deref(),
            self.locale.as_deref(),
            self.context.map(CalendarContext::from),
            self.easter_calculation_type.map(EasterCalculationType::from),
            Some(self.epiphany_on_sunday),
            Some(self.ascension_on_sunday),
            Some(self.corpus_christi_on_sunday),
            &definitions,
            &resources,
        )
    }
}

/// Output format flag
#[derive(Args, Clone)]
struct OutputArgs {
    /// Output format
    #[arg(short = 'f', long, value_enum, default_value = "yaml")]
    format: CliOutputFormat,
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
    match cli.command {
        Commands::Dates {
            date_name,
            year,
            preset,
            output,
            debug,
        } => {
            debug.init();
            dates::handle(&date_name, year, output.format.into(), preset.to_preset()?)
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
            days::handle(year, converted_filter, preset.to_preset()?, output.format.into())
        }
        Commands::List { element, output } => match element {
            ListCommand::Calendars { tree } => list::handle_calendars(output.format.into(), tree),
            ListCommand::Locales { tree } => list::handle_locales(output.format.into(), tree),
        },
        Commands::Preset {
            preset,
            output,
            debug,
        } => {
            debug.init();
            show_preset::handle(output.format.into(), preset.to_preset()?)
        }
        Commands::OptimizePreset { out, preset, debug } => {
            debug.init();
            optimize_preset::handle(preset.to_preset()?, out)
        }
        Commands::Validate { validation_type } => match validation_type {
            ValidationCommand::Definitions { file_paths } => {
                commands::validate::handle(ValidationType::Definitions, &file_paths)
            }
            ValidationCommand::Resources { file_paths } => {
                commands::validate::handle(ValidationType::Resources, &file_paths)
            }
        },
    }
}
