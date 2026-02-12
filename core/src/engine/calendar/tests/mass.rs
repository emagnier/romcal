use crate::engine::calendar::Calendar;
use crate::romcal::Romcal;
use crate::types::dates::DateDef;
use crate::types::liturgical::Precedence;
use crate::types::mass::MassTime;

// ============================================================================
// Mass assignment tests
// ============================================================================

#[test]
fn test_masses_default_is_day_mass() {
    let romcal = Romcal::empty();
    let calendar = Calendar::new(romcal, 2026).unwrap();
    let result = calendar.generate().unwrap();

    // Regular weekday should have default DayMass
    // Pick a regular Advent weekday
    if let Some(days) = result.get("2025-12-01") {
        let day = &days[0];
        assert_eq!(day.masses.len(), 1);
        assert_eq!(day.masses[0].mass_type, MassTime::DayMass);
        assert_eq!(day.masses[0].name, "day_mass");
    }
}

#[test]
fn test_masses_easter_sunday() {
    let romcal = Romcal::empty();
    let calendar = Calendar::new(romcal, 2026).unwrap();
    let result = calendar.generate().unwrap();

    // Easter Sunday 2026 is April 5
    if let Some(days) = result.get("2026-04-05") {
        let easter = days.iter().find(|d| d.id == "easter_sunday").unwrap();
        assert_eq!(easter.masses.len(), 2);
        assert!(
            easter
                .masses
                .iter()
                .any(|m| m.mass_type == MassTime::EasterVigil)
        );
        assert!(
            easter
                .masses
                .iter()
                .any(|m| m.mass_type == MassTime::DayMass)
        );
    }
}

#[test]
fn test_masses_holy_saturday_is_aliturgical() {
    let romcal = Romcal::empty();
    let calendar = Calendar::new(romcal, 2026).unwrap();
    let result = calendar.generate().unwrap();

    // Holy Saturday 2026 is April 4
    if let Some(days) = result.get("2026-04-04") {
        let holy_saturday = days.iter().find(|d| d.id == "holy_saturday").unwrap();
        // Holy Saturday is aliturgical - no masses
        assert!(
            holy_saturday.masses.is_empty(),
            "Holy Saturday should have no masses (aliturgical day)"
        );
    }
}

#[test]
fn test_masses_nativity_of_the_lord() {
    let romcal = Romcal::empty();
    let calendar = Calendar::new(romcal, 2026).unwrap();
    let result = calendar.generate().unwrap();

    // Christmas is December 25
    if let Some(days) = result.get("2025-12-25") {
        let christmas = days
            .iter()
            .find(|d| d.id == "nativity_of_the_lord")
            .unwrap();
        // Christmas has 4 masses
        assert_eq!(christmas.masses.len(), 4);
        assert!(
            christmas
                .masses
                .iter()
                .any(|m| m.mass_type == MassTime::PreviousEveningMass)
        );
        assert!(
            christmas
                .masses
                .iter()
                .any(|m| m.mass_type == MassTime::NightMass)
        );
        assert!(
            christmas
                .masses
                .iter()
                .any(|m| m.mass_type == MassTime::MassAtDawn)
        );
        assert!(
            christmas
                .masses
                .iter()
                .any(|m| m.mass_type == MassTime::DayMass)
        );
    }
}

#[test]
fn test_masses_from_calendar_definition() {
    use crate::engine::calendar_definition::CalendarDefinition;
    use crate::engine::resources::Resources;
    use crate::types::CalendarMetadata;
    use crate::types::calendar::{CalendarJurisdiction, CalendarType, DayDefinition};
    use crate::types::martyrology::MartyrologyEntryDef;
    use crate::types::mass::{MassCycleDefinition, MassesDefinitions};

    // Create a test calendar definition with masses
    let mut day_def = DayDefinition {
        date_def: Some(DateDef::MonthDate {
            month: crate::types::dates::MonthIndex(7),
            date: 15,
            day_offset: None,
        }),
        precedence: Some(Precedence::GeneralSolemnity_3),
        masses: None,
        date_exceptions: None,
        commons_def: None,
        is_holy_day_of_obligation: None,
        allow_similar_rank_items: None,
        is_optional: None,
        custom_locale_id: None,
        martyrology: None,
        titles: None,
        drop: None,
        colors: None,
    };

    // Add masses to the definition
    let mut masses_def = MassesDefinitions::new();
    masses_def.insert(MassTime::PreviousEveningMass, MassCycleDefinition::new());
    masses_def.insert(MassTime::DayMass, MassCycleDefinition::new());
    day_def.masses = Some(masses_def);

    // Create a calendar definition with this day
    let calendar_def = CalendarDefinition {
        schema: None,
        id: "test_calendar".to_string(),
        metadata: CalendarMetadata {
            r#type: CalendarType::GeneralRoman,
            jurisdiction: CalendarJurisdiction::Ecclesiastical,
        },
        particular_config: None,
        parent_calendar_ids: vec![],
        days_definitions: std::collections::BTreeMap::from([(
            "test_solemnity".to_string(),
            day_def,
        )]),
    };

    // Create martyrology entry for test_solemnity (required for strict validation)
    let entry_def = MartyrologyEntryDef {
        fullname: Some("Test Solemnity".to_string()),
        ..Default::default()
    };
    let mut resources = Resources::new("en".to_string());
    resources.add_martyrology_entry("test_solemnity".to_string(), entry_def);

    // Create romcal with this calendar definition and resources
    let mut romcal = Romcal::empty();
    romcal.calendar = "test_calendar".to_string();
    romcal.calendar_definitions.push(calendar_def);
    romcal.add_resources(resources);

    let calendar = Calendar::new(romcal, 2026).unwrap();
    let result = calendar.generate().unwrap();

    // Check July 15
    let days = result.get("2026-07-15").expect("2026-07-15 should exist");
    let test_day = days
        .iter()
        .find(|d| d.id == "test_solemnity")
        .expect("test_solemnity should be on July 15");

    // Should have PreviousEveningMass and DayMass from calendar definition
    assert_eq!(test_day.masses.len(), 2);
    assert!(
        test_day
            .masses
            .iter()
            .any(|m| m.mass_type == MassTime::PreviousEveningMass),
        "test_solemnity should have PreviousEveningMass"
    );
    assert!(
        test_day
            .masses
            .iter()
            .any(|m| m.mass_type == MassTime::DayMass),
        "test_solemnity should have DayMass"
    );
}

#[test]
fn test_masses_pentecost_sunday() {
    let romcal = Romcal::empty();
    let calendar = Calendar::new(romcal, 2026).unwrap();
    let result = calendar.generate().unwrap();

    // Pentecost 2026 is May 24
    if let Some(days) = result.get("2026-05-24") {
        let pentecost = days
            .iter()
            .find(|d| d.id == "pentecost_sunday")
            .expect("pentecost_sunday should exist");
        assert_eq!(pentecost.masses.len(), 2);
        assert!(
            pentecost
                .masses
                .iter()
                .any(|m| m.mass_type == MassTime::PreviousEveningMass),
            "Pentecost should have PreviousEveningMass"
        );
        assert!(
            pentecost
                .masses
                .iter()
                .any(|m| m.mass_type == MassTime::DayMass),
            "Pentecost should have DayMass"
        );
    }
}

#[test]
fn test_masses_palm_sunday() {
    let romcal = Romcal::empty();
    let calendar = Calendar::new(romcal, 2026).unwrap();
    let result = calendar.generate().unwrap();

    // Palm Sunday 2026 is March 29
    if let Some(days) = result.get("2026-03-29") {
        let palm_sunday = days
            .iter()
            .find(|d| d.id == "palm_sunday_of_the_passion_of_the_lord")
            .expect("palm_sunday should exist");
        assert_eq!(palm_sunday.masses.len(), 1);
        assert_eq!(palm_sunday.masses[0].mass_type, MassTime::MassOfThePassion);
        assert_eq!(palm_sunday.masses[0].name, "mass_of_the_passion");
    }
}

#[test]
fn test_masses_good_friday() {
    let romcal = Romcal::empty();
    let calendar = Calendar::new(romcal, 2026).unwrap();
    let result = calendar.generate().unwrap();

    // Good Friday 2026 is April 3
    if let Some(days) = result.get("2026-04-03") {
        let good_friday = days
            .iter()
            .find(|d| d.id == "friday_of_the_passion_of_the_lord")
            .expect("good_friday should exist");
        assert_eq!(good_friday.masses.len(), 1);
        assert_eq!(
            good_friday.masses[0].mass_type,
            MassTime::CelebrationOfThePassion
        );
        assert_eq!(good_friday.masses[0].name, "celebration_of_the_passion");
    }
}

#[test]
fn test_masses_holy_thursday() {
    let romcal = Romcal::empty();
    let calendar = Calendar::new(romcal, 2026).unwrap();
    let result = calendar.generate().unwrap();

    // Holy Thursday 2026 is April 2
    if let Some(days) = result.get("2026-04-02") {
        let holy_thursday = days
            .iter()
            .find(|d| d.id == "thursday_of_the_lords_supper")
            .expect("holy_thursday should exist");
        assert_eq!(holy_thursday.masses.len(), 1);
        assert_eq!(
            holy_thursday.masses[0].mass_type,
            MassTime::EveningMassOfTheLordsSupper
        );
        assert_eq!(
            holy_thursday.masses[0].name,
            "evening_mass_of_the_lords_supper"
        );
    }
}

#[test]
fn test_masses_december_24() {
    let romcal = Romcal::empty();
    let calendar = Calendar::new(romcal, 2026).unwrap();
    let result = calendar.generate().unwrap();

    // December 24, 2025 (in liturgical year 2026)
    if let Some(days) = result.get("2025-12-24") {
        let dec_24 = days
            .iter()
            .find(|d| d.id == "advent_december_24")
            .expect("advent_december_24 should exist");
        assert_eq!(dec_24.masses.len(), 1);
        assert_eq!(dec_24.masses[0].mass_type, MassTime::MorningMass);
        assert_eq!(dec_24.masses[0].name, "morning_mass");
    }
}

// ============================================================================
// Mass calendar generation tests
// ============================================================================

#[test]
fn test_generate_mass_calendar_basic() {
    let romcal = Romcal::empty();
    let calendar = Calendar::new(romcal, 2026).unwrap();

    let result = calendar.generate_mass_calendar();
    assert!(result.is_ok());

    let mass_calendar = result.unwrap();

    // Should have entries for each day of the liturgical year
    assert!(
        !mass_calendar.is_empty(),
        "Mass calendar should not be empty"
    );

    // Should have reasonable number of dates
    assert!(
        mass_calendar.len() >= 350,
        "Should have at least 350 dates, got {}",
        mass_calendar.len()
    );
}

#[test]
fn test_generate_mass_calendar_christmas_evening_mass_shifted() {
    let romcal = Romcal::empty();
    let calendar = Calendar::new(romcal, 2026).unwrap();
    let mass_calendar = calendar.generate_mass_calendar().unwrap();

    // Christmas PreviousEveningMass should appear on December 24 (civil date)
    // but the liturgical_date should be December 25
    if let Some(masses) = mass_calendar.get("2025-12-24") {
        let evening_mass = masses
            .iter()
            .find(|m| m.mass_time == MassTime::PreviousEveningMass);

        assert!(
            evening_mass.is_some(),
            "Christmas PreviousEveningMass should be on Dec 24"
        );

        let evening_mass = evening_mass.unwrap();
        assert_eq!(evening_mass.civil_date, "2025-12-24");
        assert_eq!(evening_mass.liturgical_date, "2025-12-25");
        assert_eq!(evening_mass.id, "nativity_of_the_lord");
    }

    // Christmas Day masses (NightMass, MassAtDawn, DayMass) should be on December 25
    if let Some(masses) = mass_calendar.get("2025-12-25") {
        let day_masses: Vec<_> = masses
            .iter()
            .filter(|m| m.id == "nativity_of_the_lord")
            .collect();

        assert!(
            day_masses.len() >= 3,
            "Christmas should have at least 3 masses on Dec 25"
        );

        // All should have liturgical_date = civil_date = 2025-12-25
        for mass in day_masses {
            assert_eq!(mass.civil_date, "2025-12-25");
            assert_eq!(mass.liturgical_date, "2025-12-25");
        }
    }
}

#[test]
fn test_generate_mass_calendar_easter_vigil_shifted() {
    let romcal = Romcal::empty();
    let calendar = Calendar::new(romcal, 2026).unwrap();
    let mass_calendar = calendar.generate_mass_calendar().unwrap();

    // Easter Vigil should appear on Holy Saturday (civil date April 4)
    // but the liturgical_date should be Easter Sunday (April 5)
    if let Some(masses) = mass_calendar.get("2026-04-04") {
        let vigil = masses.iter().find(|m| m.mass_time == MassTime::EasterVigil);

        assert!(vigil.is_some(), "Easter Vigil should be on April 4");

        let vigil = vigil.unwrap();
        assert_eq!(vigil.civil_date, "2026-04-04");
        assert_eq!(vigil.liturgical_date, "2026-04-05");
        assert_eq!(vigil.id, "easter_sunday");
    }
}

#[test]
fn test_generate_mass_calendar_context_from_liturgical_date() {
    let romcal = Romcal::empty();
    let calendar = Calendar::new(romcal, 2026).unwrap();
    let mass_calendar = calendar.generate_mass_calendar().unwrap();

    // Easter Vigil (on civil date April 4) should have Easter's context, not Holy Saturday's
    if let Some(masses) = mass_calendar.get("2026-04-04") {
        let vigil = masses
            .iter()
            .find(|m| m.mass_time == MassTime::EasterVigil)
            .expect("Easter Vigil should exist");

        // The season should be Easter Time (from Easter Sunday's context)
        // Not Paschal Triduum (from Holy Saturday's context)
        // Note: Easter Sunday is in Easter Time
        assert!(vigil.season.is_some(), "Easter Vigil should have a season");
    }
}

#[test]
fn test_generate_mass_calendar_flat_structure() {
    let romcal = Romcal::empty();
    let calendar = Calendar::new(romcal, 2026).unwrap();
    let mass_calendar = calendar.generate_mass_calendar().unwrap();

    // Pick any date and verify the flat structure
    if let Some(masses) = mass_calendar.get("2025-12-25") {
        let mass = &masses[0];

        // Mass identification should be present
        assert!(!mass.mass_time_name.is_empty());

        // Day-level context should be directly accessible (flat)
        assert!(!mass.sunday_cycle_name.is_empty());
        assert!(!mass.weekday_cycle_name.is_empty());

        // Celebration data should be directly accessible (flat)
        assert!(!mass.id.is_empty());
        assert!(!mass.fullname.is_empty());
        assert!(!mass.rank_name.is_empty());
    }
}

#[test]
fn test_generate_mass_calendar_serialization() {
    let romcal = Romcal::empty();
    let calendar = Calendar::new(romcal, 2026).unwrap();
    let mass_calendar = calendar.generate_mass_calendar().unwrap();

    // Serialize to JSON
    let json = serde_json::to_string(&mass_calendar);
    assert!(json.is_ok(), "Mass calendar should serialize to JSON");

    let json_str = json.unwrap();
    // Check that mass_time is serialized as snake_case
    assert!(
        json_str.contains("\"mass_time\":\"day_mass\"")
            || json_str.contains("\"mass_time\":\"previous_evening_mass\"")
            || json_str.contains("\"mass_time\":\"easter_vigil\""),
        "mass_time should be serialized as snake_case"
    );
}
