package com.romcal

import java.io.File

/**
 * Test fixtures for loading calendar definitions and resources.
 */
object Fixtures {

    private val dataDir: File by lazy {
        // Navigate from bindings/kotlin to project root, then to data/
        File(System.getProperty("user.dir")).resolve("../../data")
    }

    /**
     * Load all calendar definitions as a merged JSON string.
     */
    fun loadAllCalendarDefinitionsJson(): String {
        val definitionsDir = File(dataDir, "definitions")
        val files = definitionsDir.walkTopDown()
            .filter { it.isFile && it.extension == "json" }
            .map { it.readText() }
            .toList()
        return Romcal.mergeCalendarDefinitionsJson(files)
    }

    /**
     * Load all resources as a merged JSON array string.
     */
    fun loadAllResourcesJson(): String {
        val resourcesDir = File(dataDir, "resources")
        val filesByLocale = mutableMapOf<String, MutableList<String>>()

        resourcesDir.walkTopDown()
            .filter { it.isFile && it.extension == "json" }
            .forEach { file ->
                val locale = file.parentFile.name
                filesByLocale.getOrPut(locale) { mutableListOf() }.add(file.readText())
            }

        val mergedResources = filesByLocale.map { (locale, files) ->
            Romcal.mergeResourceFilesJson(locale, files)
        }
        return "[${mergedResources.joinToString(",")}]"
    }

    /**
     * Load resources for a specific locale as a merged JSON string.
     */
    fun loadResourcesForLocale(locale: String): String {
        val localeDir = File(dataDir, "resources/$locale")
        if (!localeDir.exists()) {
            throw IllegalArgumentException("Locale directory not found: $locale")
        }

        val files = localeDir.walkTopDown()
            .filter { it.isFile && it.extension == "json" }
            .map { it.readText() }
            .toList()

        return Romcal.mergeResourceFilesJson(locale, files)
    }
}
