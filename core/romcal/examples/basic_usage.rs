use romcal::{LiturgicalConfig, LiturgicalDates};

fn main() {
    // Default configuration
    let config = LiturgicalConfig::default();
    let dates = LiturgicalDates::new(config, 2024).unwrap();

    println!("=== Liturgical date calculations for 2024 ===");

    // Easter
    let easter = dates.get_easter_sunday_date_unwrap(None);
    println!("Easter 2024: {}", easter.format("%d/%m/%Y"));

    // Ash Wednesday
    let ash_wednesday = dates.get_ash_wednesday_date(None);
    println!("Ash Wednesday: {}", ash_wednesday.format("%d/%m/%Y"));

    // First Sunday of Advent
    let first_advent = dates.get_first_sunday_of_advent_date(None);
    println!(
        "First Sunday of Advent: {}",
        first_advent.format("%d/%m/%Y")
    );

    // Christmas
    let christmas = dates.get_christmas_date(None);
    println!("Christmas: {}", christmas.format("%d/%m/%Y"));

    // Epiphany
    let epiphany = dates.get_epiphany_date(None);
    println!("Epiphany: {}", epiphany.format("%d/%m/%Y"));

    // Ascension
    let ascension = dates.get_ascension_date(None);
    println!("Ascension: {}", ascension.format("%d/%m/%Y"));

    // Pentecost
    let pentecost = dates.get_pentecost_sunday_date(None);
    println!("Pentecost: {}", pentecost.format("%d/%m/%Y"));

    // Fixed feasts
    println!("\n=== Fixed feasts ===");
    let mary_mother_god = dates.get_mary_mother_of_god_date(None);
    println!(
        "Mary, Mother of God: {}",
        mary_mother_god.format("%d/%m/%Y")
    );

    let annunciation = dates.get_annunciation_date(None);
    println!("Annunciation: {}", annunciation.format("%d/%m/%Y"));

    let assumption = dates.get_assumption_date(None);
    println!("Assumption: {}", assumption.format("%d/%m/%Y"));

    let all_saints = dates.get_all_saints_date(None);
    println!("All Saints: {}", all_saints.format("%d/%m/%Y"));

    let immaculate_conception = dates.get_immaculate_conception_of_mary_date(None);
    println!(
        "Immaculate Conception: {}",
        immaculate_conception.format("%d/%m/%Y")
    );

    // Advent weekdays
    println!("\n=== Advent weekdays ===");
    if let Some(weekday) = dates.unprivileged_weekday_of_advent(1, 1, None) {
        println!(
            "Monday of 1st week of Advent: {}",
            weekday.format("%d/%m/%Y")
        );
    }
    if let Some(weekday) = dates.privileged_weekday_of_advent(17, None) {
        println!(
            "December 17 (privileged weekday): {}",
            weekday.format("%d/%m/%Y")
        );
    }
    if let Some(sunday) = dates.get_sunday_of_advent_date(2, None) {
        println!("2nd Sunday of Advent: {}", sunday.format("%d/%m/%Y"));
    }

    // Seasons
    println!("\n=== Start of seasons ===");
    let seasons = dates.get_start_of_seasons_dates(None);
    for (season, date) in seasons {
        println!("{:?}: {}", season, date.format("%d/%m/%Y"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn test_easter_2024() {
        let config = LiturgicalConfig::default();
        let dates = LiturgicalDates::new(config, 2024).unwrap();
        let easter = dates.get_easter_sunday_date_unwrap(None);

        // Easter 2024 is March 31
        assert_eq!(easter.day(), 31);
        assert_eq!(easter.month(), 3);
        assert_eq!(easter.year(), 2024);
    }

    #[test]
    fn test_christmas() {
        let config = LiturgicalConfig::default();
        let dates = LiturgicalDates::new(config, 2024).unwrap();
        let christmas = dates.get_christmas_date(None);

        // Christmas is always December 25
        assert_eq!(christmas.day(), 25);
        assert_eq!(christmas.month(), 12);
        assert_eq!(christmas.year(), 2024);
    }

    #[test]
    fn test_fixed_feasts() {
        let config = LiturgicalConfig::default();
        let dates = LiturgicalDates::new(config, 2024).unwrap();

        // Mary, Mother of God - January 1
        let mary_mother_god = dates.get_mary_mother_of_god_date(None);
        assert_eq!(mary_mother_god.day(), 1);
        assert_eq!(mary_mother_god.month(), 1);

        // Assumption - August 15
        let assumption = dates.get_assumption_date(None);
        assert_eq!(assumption.day(), 15);
        assert_eq!(assumption.month(), 8);

        // All Saints - November 1
        let all_saints = dates.get_all_saints_date(None);
        assert_eq!(all_saints.day(), 1);
        assert_eq!(all_saints.month(), 11);
    }
}
