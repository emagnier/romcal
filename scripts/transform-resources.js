#!/usr/bin/env node

const fs = require("fs");
const path = require("path");

/**
 * Script to transform resources JSON files structure:
 * 1. Rename "items" property to "entities"
 * 2. Transform entities from Record<key, object> to array of {id: key, ...object}
 * 3. Remove "letter" property
 */

const RESOURCES_DIR = path.join(__dirname, "..", "data", "resources");

function transformResourceFile(filePath) {
	console.log(`🔄 Processing: ${path.relative(process.cwd(), filePath)}`);

	try {
		// Read the JSON file
		const content = fs.readFileSync(filePath, "utf8");
		const data = JSON.parse(content);

		// Check if the file has the expected structure
		if (!data.items && !data.entities) {
			console.log(
				`⚠️  Skipping ${filePath} - no "items" or "entities" property found`,
			);
			return;
		}

		// Transform the data
		const transformed = {
			$schema: "../schemas/resources-definition.json",
			locale: data.locale,
		};

		// Transform items to entities array or keep existing entities
		if (data.items && typeof data.items === "object") {
			// Original format: transform items to entities array
			transformed.entities = Object.entries(data.items).map(([key, value]) => ({
				id: key,
				...value,
			}));
		} else if (data.entities && Array.isArray(data.entities)) {
			// Already transformed format: keep entities as is
			transformed.entities = data.entities;
		}

		// Write back the transformed data
		fs.writeFileSync(filePath, JSON.stringify(transformed, null, 2) + "\n");
		console.log(`✅ Transformed: ${path.relative(process.cwd(), filePath)}`);
	} catch (error) {
		console.error(`❌ Error processing ${filePath}:`, error.message);
	}
}

function processDirectory(dirPath) {
	const items = fs.readdirSync(dirPath);

	for (const item of items) {
		const itemPath = path.join(dirPath, item);
		const stat = fs.statSync(itemPath);

		if (stat.isDirectory()) {
			// Recursively process subdirectories
			processDirectory(itemPath);
		} else if (item.endsWith(".json")) {
			// Process JSON files
			transformResourceFile(itemPath);
		}
	}
}

function main() {
	console.log("🚀 Starting resources transformation...");
	console.log(`📁 Resources directory: ${RESOURCES_DIR}`);

	if (!fs.existsSync(RESOURCES_DIR)) {
		console.error(`❌ Resources directory not found: ${RESOURCES_DIR}`);
		process.exit(1);
	}

	processDirectory(RESOURCES_DIR);

	console.log("✅ Resources transformation completed!");
}

if (require.main === module) {
	main();
}

module.exports = { transformResourceFile, processDirectory };
