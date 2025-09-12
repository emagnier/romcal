use romcal_core::resources::*;
use romcal_core::types::*;
use std::collections::HashMap;

fn main() {
    // Example of using resource types

    // Create an entity definition for a saint
    let mut saint_definition = EntityDefinition::new("john_the_baptist".to_string());
    saint_definition.name = Some("Jean-Baptiste".to_string());
    saint_definition.fullname = Some("Saint Jean-Baptiste, Prophète et Martyr".to_string());
    saint_definition.canonization_level = Some(CanonizationLevel::Saint);
    saint_definition.titles = Some(vec![Title::Prophet, Title::Martyr]);
    saint_definition.sex = Some(Sex::Male);
    saint_definition.date_of_death = Some(SaintDateDef::Date(SaintDate::Year(30)));
    saint_definition.date_of_death_is_approximative = Some(true);
    saint_definition.sources = Some(vec![
        "Martyrologe Romain".to_string(),
        "Évangiles".to_string(),
    ]);

    // Create an entity definition for a place
    let mut place_definition = EntityDefinition::new("lourdes".to_string());
    place_definition.r#type = Some(EntityType::Place);
    place_definition.name = Some("Lourdes".to_string());
    place_definition.fullname = Some("Notre-Dame de Lourdes".to_string());
    place_definition.date_of_dedication = Some(SaintDateDef::Date(SaintDate::YearMonthDay(
        "1858-02-11".to_string(),
    )));

    // Create resource metadata
    let colors = LocaleColors {
        black: Some("noir".to_string()),
        gold: Some("doré".to_string()),
        green: Some("vert".to_string()),
        purple: Some("violet".to_string()),
        red: Some("rouge".to_string()),
        rose: Some("rose".to_string()),
        white: Some("blanc".to_string()),
    };

    let seasons = SeasonsMetadata {
        advent: Some(AdventSeason {
            season: Some("Temps de l'Avent".to_string()),
            weekday: Some("$t(weekdays:{{dow}}, capitalize) de la $t(ordinals:{{week}}, { \"context\": \"feminine\" }) semaine de l'Avent".to_string()),
            sunday: Some("$t(ordinals:{{week}}, capitalize) dimanche de l'Avent".to_string()),
            privileged_weekday: Some("{{day}} $t(months:11)".to_string()),
        }),
        christmas_time: Some(ChristmasTimeSeason {
            season: Some("Temps de Noël".to_string()),
            day: Some("$t(weekdays:{{dow}}, capitalize) dans le Temps de Noël".to_string()),
            octave: Some("{{count}}ᵉ jour dans l'Octave de la Nativité".to_string()),
            before_epiphany: Some("{{day}} $t(months:0)".to_string()),
            second_sunday_after_christmas: Some("Deuxième dimanche après la Nativité".to_string()),
            after_epiphany: Some("$t(weekdays:{{dow}}, capitalize) après l'Épiphanie".to_string()),
        }),
        ordinary_time: None,
        lent: None,
        paschal_triduum: None,
        easter_time: None,
    };

    let metadata = ResourcesMetadata {
        ordinals: Some(create_ordinals()),
        weekdays: Some(create_weekdays()),
        months: Some(create_months()),
        colors: Some(colors),
        seasons: Some(seasons),
        periods: None,
        ranks: None,
        cycles: None,
    };

    // Create the complete resource definition
    let mut resources = ResourcesDefinition::new("fr".to_string());
    resources.schema = Some("https://romcal.org/schemas/resources.json".to_string());
    resources.metadata = Some(metadata);

    // Add entities
    resources.add_entity(saint_definition);
    resources.add_entity(place_definition);

    // Validate entity consistency
    if let Err(errors) = resources.validate_entities() {
        println!("Validation errors: {:?}", errors);
        return;
    }

    // Demonstrate utility methods
    println!("Entity IDs: {:?}", resources.get_entity_ids());

    if let Some(saint) = resources.get_entity("john_the_baptist") {
        println!(
            "Saint found: {}",
            saint.name.as_deref().unwrap_or("No name")
        );
    }

    // Example of entity merging
    let mut additional_resources = ResourcesDefinition::new("fr".to_string());
    let mut additional_saint = EntityDefinition::new("mary".to_string());
    additional_saint.name = Some("Marie".to_string());
    additional_saint.fullname = Some("Sainte Marie, Mère de Dieu".to_string());
    additional_saint.canonization_level = Some(CanonizationLevel::Saint);
    additional_saint.sex = Some(Sex::Female);

    additional_resources.add_entity(additional_saint);

    // Merge entities
    resources.merge_entities(&additional_resources);
    println!(
        "Merge successful! Entity count: {}",
        resources.get_entities().map(|e| e.len()).unwrap_or(0)
    );

    // Serialize to JSON for demonstration
    match serde_json::to_string_pretty(&resources) {
        Ok(json) => println!("Serialized resources:\n{}", json),
        Err(e) => println!("Serialization error: {}", e),
    }
}

fn create_ordinals() -> HashMap<String, String> {
    let mut ordinals = HashMap::new();
    ordinals.insert("1".to_string(), "premier".to_string());
    ordinals.insert("1_feminine".to_string(), "première".to_string());
    ordinals.insert("2".to_string(), "deuxième".to_string());
    ordinals.insert("3".to_string(), "troisième".to_string());
    ordinals.insert("4".to_string(), "quatrième".to_string());
    ordinals.insert("5".to_string(), "cinquième".to_string());
    ordinals
}

fn create_weekdays() -> HashMap<String, String> {
    let mut weekdays = HashMap::new();
    weekdays.insert("0".to_string(), "dimanche".to_string());
    weekdays.insert("1".to_string(), "lundi".to_string());
    weekdays.insert("2".to_string(), "mardi".to_string());
    weekdays.insert("3".to_string(), "mercredi".to_string());
    weekdays.insert("4".to_string(), "jeudi".to_string());
    weekdays.insert("5".to_string(), "vendredi".to_string());
    weekdays.insert("6".to_string(), "samedi".to_string());
    weekdays
}

fn create_months() -> HashMap<String, String> {
    let mut months = HashMap::new();
    months.insert("0".to_string(), "janvier".to_string());
    months.insert("1".to_string(), "février".to_string());
    months.insert("2".to_string(), "mars".to_string());
    months.insert("3".to_string(), "avril".to_string());
    months.insert("4".to_string(), "mai".to_string());
    months.insert("5".to_string(), "juin".to_string());
    months.insert("6".to_string(), "juillet".to_string());
    months.insert("7".to_string(), "août".to_string());
    months.insert("8".to_string(), "septembre".to_string());
    months.insert("9".to_string(), "octobre".to_string());
    months.insert("10".to_string(), "novembre".to_string());
    months.insert("11".to_string(), "décembre".to_string());
    months
}
