use crate::config::CliConfig;
use crate::error::RomcalCliError;
use crate::output::OutputFormat;
use clap::Subcommand;
use romcal_core::LiturgicalDates;

/// Subcommands for date calculations
#[derive(Subcommand)]
pub enum DateCommands {
    /// Mary Mother of the Church
    #[command(name = "mary_mother_of_the_church")]
    MaryMotherOfTheChurch,
    /// Epiphany Sunday
    #[command(name = "epiphany_sunday")]
    EpiphanySunday,
    /// Presentation of the Lord
    #[command(name = "presentation_of_the_lord")]
    PresentationOfTheLord,
    /// Annunciation
    #[command(name = "annunciation")]
    Annunciation,
    /// Palm Sunday
    #[command(name = "palm_sunday")]
    PalmSunday,
    /// Easter Sunday
    #[command(name = "easter_sunday")]
    EasterSunday,
    /// Divine Mercy Sunday
    #[command(name = "divine_mercy_sunday")]
    DivineMercySunday,
    /// Immaculate Heart of Mary
    #[command(name = "immaculate_heart_of_mary")]
    ImmaculateHeartOfMary,
    /// Pentecost Sunday
    #[command(name = "pentecost_sunday")]
    PentecostSunday,
    /// Corpus Christi Sunday
    #[command(name = "corpus_christi_sunday")]
    CorpusChristiSunday,
    /// Nativity of John the Baptist
    #[command(name = "nativity_of_john_the_baptist")]
    NativityOfJohnTheBaptist,
    /// Peter and Paul Apostles
    #[command(name = "peter_and_paul_apostles")]
    PeterAndPaulApostles,
    /// Transfiguration
    #[command(name = "transfiguration")]
    Transfiguration,
    /// Assumption
    #[command(name = "assumption")]
    Assumption,
    /// Exaltation of the Holy Cross
    #[command(name = "exaltation_of_the_holy_cross")]
    ExaltationOfTheHolyCross,
    /// All Saints
    #[command(name = "all_saints")]
    AllSaints,
    /// Immaculate Conception of Mary
    #[command(name = "immaculate_conception_of_mary")]
    ImmaculateConceptionOfMary,
}

/// Handle dates command
#[allow(clippy::too_many_arguments)]
pub fn handle(
    date_command: DateCommands,
    year: Option<i32>,
    format: &str,
    easter_calculation_type: &str,
    scope: &str,
    ascension_on_sunday: bool,
    epiphany_on_sunday: bool,
    corpus_christi_on_sunday: bool,
) -> Result<(), RomcalCliError> {
    let config = CliConfig::default();
    let year = year.unwrap_or_else(CliConfig::current_year);

    // Validate year
    CliConfig::validate_year(year)?;

    // Validate easter calculation type
    match easter_calculation_type.to_lowercase().as_str() {
        "gregorian" | "julian" => {}
        _ => {
            return Err(RomcalCliError::config_error(
                "Invalid easter calculation type. Must be 'gregorian' or 'julian'".to_string(),
            ))
        }
    }

    // Validate scope
    match scope.to_lowercase().as_str() {
        "gregorian" | "liturgical" => {}
        _ => {
            return Err(RomcalCliError::config_error(
                "Invalid scope. Must be 'gregorian' or 'liturgical'".to_string(),
            ))
        }
    }

    RomcalCliError::validate_format(format)?;

    // Parse output format
    let output_format = match format.to_lowercase().as_str() {
        "json" => OutputFormat::Json,
        "csv" => OutputFormat::Csv,
        "yaml" => OutputFormat::Yaml,
        "lines" => OutputFormat::Lines,
        _ => unreachable!(), // Already validated above
    };

    // Create liturgical configuration
    let liturgical_config = config.create_liturgical_config(
        None, // calendar
        None, // locale
        Some(easter_calculation_type),
        Some(ascension_on_sunday),
        Some(corpus_christi_on_sunday),
        Some(epiphany_on_sunday),
    )?;

    // Create liturgical dates instance
    let dates = LiturgicalDates::new(liturgical_config, year)?;

    // Calculate the requested date
    let date_result = match date_command {
        DateCommands::MaryMotherOfTheChurch => dates.get_mary_mother_of_the_church_date(None),
        DateCommands::EpiphanySunday => dates.get_epiphany_date(None),
        DateCommands::PresentationOfTheLord => dates.get_presentation_of_the_lord_date(None),
        DateCommands::Annunciation => dates.get_annunciation_date(None),
        DateCommands::PalmSunday => dates.get_palm_sunday_date(None),
        DateCommands::EasterSunday => dates.get_easter_sunday_date_unwrap(None),
        DateCommands::DivineMercySunday => dates.get_divine_mercy_sunday_date(None),
        DateCommands::ImmaculateHeartOfMary => dates.get_immaculate_heart_of_mary_date(None),
        DateCommands::PentecostSunday => dates.get_pentecost_sunday_date(None),
        DateCommands::CorpusChristiSunday => dates.get_corpus_christi_date(None),
        DateCommands::NativityOfJohnTheBaptist => dates.get_nativity_of_john_the_baptist_date(None),
        DateCommands::PeterAndPaulApostles => dates.get_peter_and_paul_apostles_date(None),
        DateCommands::Transfiguration => dates.get_transfiguration_date(None),
        DateCommands::Assumption => dates.get_assumption_date(None),
        DateCommands::ExaltationOfTheHolyCross => dates.get_exaltation_of_the_holy_cross_date(None),
        DateCommands::AllSaints => dates.get_all_saints_date(None),
        DateCommands::ImmaculateConceptionOfMary => {
            dates.get_immaculate_conception_of_mary_date(None)
        }
    };

    // Output the result - only the date in YYYY-MM-DD format
    let date_string = date_result.format("%Y-%m-%d").to_string();

    match output_format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&date_string).unwrap());
        }
        OutputFormat::Csv => {
            println!("{}", date_string);
        }
        OutputFormat::Yaml => {
            println!("{}", serde_yaml::to_string(&date_string).unwrap());
        }
        OutputFormat::Lines => {
            println!("{}", date_string);
        }
    }

    Ok(())
}
