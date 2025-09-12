#!/usr/bin/env node

const fs = require("node:fs");
const path = require("node:path");

/**
 * Script to transform calendar JSON files
 * - Adds $schema property at the beginning of each file
 * - Renames parentCalendarIds → parent_calendar_ids
 * - Renames inputs → days_definitions
 * - Transforms days_definitions from object {key: value} to array [{id: key, ...value}]
 */

function calculateSchemaPath(filePath) {
	// Calculate the number of directory levels to go up from data/calendars
	const calendarsDir = path.join(__dirname, "..", "data", "calendars");
	const relativePath = path.relative(calendarsDir, filePath);
	const pathParts = relativePath.split(path.sep);
	const depth = pathParts.length + 1; // Number of directories to go up

	// Build the relative path to the schema
	const schemaPath = `${"../".repeat(depth)}schemas/calendar_definition.json`;
	return schemaPath;
}

function transformCalendarObject(obj, filePath) {
	// Calculate the schema path
	const schemaPath = calculateSchemaPath(filePath);

	// Create a new object with $schema first
	const transformed = {
		$schema: schemaPath,
		...obj,
	};

	// 1. Rename parentCalendarIds to parent_calendar_ids
	if (transformed.parentCalendarIds !== undefined) {
		transformed.parent_calendar_ids = transformed.parentCalendarIds;
		delete transformed.parentCalendarIds;
	}

	// 2. Rename inputs to days_definitions and transform the structure
	if (transformed.inputs !== undefined) {
		const inputs = transformed.inputs;
		const daysDefinitions = [];

		// Transform object {key: value} to array [{id: key, ...value}]
		for (const [key, value] of Object.entries(inputs)) {
			daysDefinitions.push({
				id: key,
				...value,
			});
		}

		transformed.days_definitions = daysDefinitions;
		delete transformed.inputs;
	}

	return transformed;
}

function processFile(filePath) {
	try {
		console.log(`Processing: ${filePath}`);

		// Read the file
		const content = fs.readFileSync(filePath, "utf8");
		const data = JSON.parse(content);

		// Transform the object
		const transformedData = transformCalendarObject(data, filePath);

		// Write the transformed file with a newline at the end
		const transformedContent = `${JSON.stringify(transformedData, null, 2)}\n`;
		fs.writeFileSync(filePath, transformedContent, "utf8");

		console.log(`✓ Successfully transformed: ${filePath}`);
	} catch (error) {
		console.error(`✗ Error processing ${filePath}:`, error.message);
	}
}

function processDirectory(dirPath) {
	try {
		const items = fs.readdirSync(dirPath);

		for (const item of items) {
			const itemPath = path.join(dirPath, item);
			const stat = fs.statSync(itemPath);

			if (stat.isDirectory()) {
				// Process subdirectories recursively
				processDirectory(itemPath);
			} else if (item.endsWith(".json")) {
				// Process JSON files
				processFile(itemPath);
			}
		}
	} catch (error) {
		console.error(`Error reading directory ${dirPath}:`, error.message);
	}
}

function main() {
	const calendarsDir = path.join(__dirname, "..", "data", "calendars");

	if (!fs.existsSync(calendarsDir)) {
		console.error(`Directory ${calendarsDir} does not exist.`);
		process.exit(1);
	}

	console.log("🚀 Starting calendar transformation...");
	console.log(`📁 Source directory: ${calendarsDir}`);
	console.log("");

	processDirectory(calendarsDir);

	console.log("");
	console.log("✅ Transformation completed!");
}

// Execute the script if called directly
if (require.main === module) {
	main();
}

module.exports = {
	transformCalendarObject,
	processFile,
	processDirectory,
};
