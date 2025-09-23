use romcal_core::types::entity::{Title, TitlesDef};
use romcal_core::types::{
    Color, ColorInfo, Common, CommonInfo, DateDef, DayOfWeek, MonthIndex, Period, PeriodInfo,
    Precedence, PsalterWeekCycle, PsalterWeekCycleInfo, Rank, Season, SeasonInfo, SundayCycle,
    SundayCycleInfo, WeekdayCycle, WeekdayCycleInfo,
};
use romcal_core::LiturgicalDay;

fn main() {
    // Example 1: Create a basic liturgical day
    let basic_day = LiturgicalDay::new(
        "easter_sunday_2024".to_string(),
        "Easter Sunday".to_string(),
        "2024-03-31".to_string(),
        "general_roman".to_string(),
    );

    println!("Basic liturgical day: {}", basic_day.get_display_name());
    println!("Date: {}", basic_day.get_date());
    println!("Is holy day: {}", basic_day.is_holy_day());

    // Example 2: Create a liturgical day with all required fields
    let easter_sunday = LiturgicalDay::with_required_fields(
        "easter_sunday_2024".to_string(),
        "Easter Sunday".to_string(),
        "2024-03-31".to_string(),
        DateDef::MonthDate {
            month: MonthIndex(3),
            date: 31,
            day_offset: None,
        },
        Precedence::Triduum_1,
        Rank::Solemnity,
        "Solemnity".to_string(),
        "general_roman".to_string(),
    );

    println!("\nEaster Sunday with required fields:");
    println!("ID: {}", easter_sunday.id);
    println!("Full name: {}", easter_sunday.fullname);
    println!("Date: {}", easter_sunday.date);
    println!("Precedence: {:?}", easter_sunday.precedence);
    println!("Rank: {:?}", easter_sunday.rank);
    println!("Is holy day: {}", easter_sunday.is_holy_day());

    // Example 3: Use builder pattern to create a complex liturgical day
    let complex_day = LiturgicalDay::new(
        "christmas_2024".to_string(),
        "Christmas Day".to_string(),
        "2024-12-25".to_string(),
        "general_roman".to_string(),
    )
    .with_seasons(vec![SeasonInfo {
        key: Season::ChristmasTime,
        name: "Christmas Time".to_string(),
    }])
    .with_periods(vec![PeriodInfo {
        key: Period::ChristmasOctave,
        name: "Christmas Octave".to_string(),
    }])
    .with_colors(vec![ColorInfo {
        key: Color::White,
        name: "White".to_string(),
    }])
    .with_commons(vec![CommonInfo {
        key: Common::BlessedVirginMary_Christmas,
        name: "Blessed Virgin Mary - Christmas".to_string(),
    }])
    .with_cycles(
        SundayCycleInfo {
            key: SundayCycle::YearB,
            name: "Year B".to_string(),
        },
        WeekdayCycleInfo {
            key: WeekdayCycle::Year1,
            name: "Year 1".to_string(),
        },
        PsalterWeekCycleInfo {
            key: PsalterWeekCycle::Week1,
            name: "Week 1".to_string(),
        },
    )
    .with_day_of_week(DayOfWeek(3)) // Wednesday
    .with_season_position(1, 1) // First week, first day
    .with_nth_day_of_week_in_month(4) // 4th Wednesday of December
    .with_liturgical_year_boundaries(
        "2024-12-01".to_string(), // First Sunday of Advent
        "2025-11-29".to_string(), // Last Saturday of Ordinary Time
    )
    .with_season_boundaries(
        "2024-12-25".to_string(), // Christmas Day
        "2025-01-12".to_string(), // Baptism of the Lord
    )
    .with_flags(true, false, false) // Holy day, not optional, doesn't allow similar items
    .with_titles(TitlesDef::Titles(vec![Title::Apostle]));

    println!(
        "\nComplex liturgical day: {}",
        complex_day.get_display_name()
    );
    println!("Seasons: {:?}", complex_day.seasons);
    println!("Colors: {:?}", complex_day.colors);
    println!("Is holy day: {}", complex_day.is_holy_day());
    println!("Is optional: {}", complex_day.is_optional_day());
    println!(
        "Parent overrides count: {}",
        complex_day.parent_override_count()
    );

    // Example 4: Add parent overrides
    let mut day_with_overrides = complex_day;
    day_with_overrides.add_parent_override(LiturgicalDay::new(
        "local_override".to_string(),
        "Local Override".to_string(),
        "2024-12-25".to_string(),
        "local_calendar".to_string(),
    ));

    println!("\nAfter adding parent override:");
    println!(
        "Parent overrides count: {}",
        day_with_overrides.parent_override_count()
    );
}
