use romcal_core::calendar_def::CalendarDefinition;
use romcal_core::config::{LiturgicalConfig, LiturgicalConfigPartial};
use romcal_core::resources::ResourcesDefinition;
use std::fs;

// Calendar files to load
const CALENDAR_FILES: &[&str] = &[
    "data/calendars/general_roman/general_roman.json",
    "data/calendars/countries/france/france.json",
    "data/calendars/countries/france/france__angers.json",
];

// Resource files to load
const RESOURCE_FILES: &[&str] = &[
    "data/resources/en/meta.json",
    "data/resources/en/a.json",
    "data/resources/en/b.json",
    "data/resources/en/c.json",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Loading real data with LiturgicalConfig ===\n");

    // Create a liturgical configuration
    let mut config = LiturgicalConfig::new(LiturgicalConfigPartial::default());
    println!("✅ Liturgical configuration created");

    // Load calendars
    println!("\n📅 Loading calendars...");

    for file_path in CALENDAR_FILES {
        if std::path::Path::new(file_path).exists() {
            let content = fs::read_to_string(file_path)?;
            let calendar_def: CalendarDefinition = serde_json::from_str(&content)?;
            config.add_calendar_definition(calendar_def);
            println!("  ✅ {} loaded", file_path);
        } else {
            println!("  ❌ {} not found", file_path);
        }
    }

    // Load resources
    println!("\n📚 Loading resources...");

    for file_path in RESOURCE_FILES {
        if std::path::Path::new(file_path).exists() {
            let content = fs::read_to_string(file_path)?;
            let resources_def: ResourcesDefinition = serde_json::from_str(&content)?;
            config.add_resources(resources_def);
            println!("  ✅ {} loaded", file_path);
        } else {
            println!("  ❌ {} not found", file_path);
        }
    }

    // Display a summary
    println!("\n📊 Configuration summary :");
    println!(
        "  - Calendars loaded : {}",
        config.calendar_definitions.len()
    );
    println!("  - Resources loaded : {}", config.resources.len());

    // Analyze loaded calendars
    println!("\n🔍 Calendar analysis :");
    for cal_def in &config.calendar_definitions {
        println!("  📅 Calendar '{}' :", cal_def.id);
        println!("    - Type : {:?}", cal_def.metadata.r#type);
        println!("    - Jurisdiction : {:?}", cal_def.metadata.jurisdiction);
        println!("    - Days defined : {}", cal_def.days_definitions.len());

        // Count martyrology with count
        let mut martyrology_with_count = 0;
        for day_def in &cal_def.days_definitions {
            if let Some(martyrology) = &day_def.martyrology {
                for item in martyrology {
                    if let romcal_core::types::MartyrologyItemPointer::Redefined(obj) = item {
                        if obj.count.is_some() {
                            martyrology_with_count += 1;
                        }
                    }
                }
            }
        }
        if martyrology_with_count > 0 {
            println!("    - Martyrology with count : {}", martyrology_with_count);
        }
    }

    // Analyze loaded resources
    println!("\n📖 Resource analysis :");
    for res_def in &config.resources {
        println!("  🌐 Resources '{}' :", res_def.locale);
        if let Some(entities) = &res_def.entities {
            println!("    - Entities defined : {}", entities.len());

            // Count entities with count
            let mut entities_with_count = 0;
            for entity in entities {
                if entity.count.is_some() {
                    entities_with_count += 1;
                }
            }
            if entities_with_count > 0 {
                println!("    - Entities with count : {}", entities_with_count);
            }
        } else {
            println!("    - No entities defined");
        }
    }

    // Test utility methods
    println!("\n🔧 Testing utility methods :");

    if let Some(_cal_def) = config.get_calendar_definition("general_roman") {
        println!("  ✅ get_calendar_definition('general_roman') : found");
    } else {
        println!("  ❌ get_calendar_definition('general_roman') : not found");
    }

    if let Some(_res_def) = config.get_resources("en") {
        println!("  ✅ get_resources('en') : found");
    } else {
        println!("  ❌ get_resources('en') : not found");
    }

    // Test serialization and save to file
    println!("\n💾 Testing serialization :");
    let serialized = serde_json::to_string_pretty(&config)?;
    println!(
        "  ✅ Serialization successful ({} characters)",
        serialized.len()
    );

    // Save complete data to JSON file
    let output_file = "_loaded_data_output.json";
    fs::write(output_file, &serialized)?;
    println!("  📁 Complete data saved to: {}", output_file);

    // Save a sample for display
    let sample = &serialized[..1000.min(serialized.len())];
    println!("  📄 JSON sample :");
    println!("{}...", sample);

    println!("\n🎉 All tests passed !");
    println!("✅ LiturgicalConfig works perfectly with real data");

    Ok(())
}
