plugins {
  kotlin("jvm") version "2.1.0"
  kotlin("plugin.serialization") version "2.1.0"
  id("org.jlleitschuh.gradle.ktlint") version "12.1.2"
  `maven-publish`
}

group = "dev.romcal"
version = "4.0.0-beta.3"

repositories {
  mavenCentral()
}

dependencies {
  // JSON serialization
  implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.7.3")

  // JNA for UniFFI native library loading
  implementation("net.java.dev.jna:jna:5.15.0")

  testImplementation(kotlin("test"))
}

tasks.test {
  useJUnitPlatform()
  // Set library path for native libraries during tests
  systemProperty("jna.library.path", "$projectDir/src/main/resources")
}

kotlin {
  jvmToolchain(21)
}

ktlint {
  filter {
    // Exclude generated files from linting
    exclude { it.file.path.contains("/ffi/") }
    exclude { it.file.path.contains("/types/Types.kt") }
  }
}

publishing {
  publications {
    create<MavenPublication>("maven") {
      from(components["java"])

      pom {
        name.set("Romcal Types")
        description.set("Liturgical calendar types for romcal")
        url.set("https://romcal.dev")

        licenses {
          license {
            name.set("Apache License, Version 2.0")
            url.set("https://www.apache.org/licenses/LICENSE-2.0")
          }
        }

        developers {
          developer {
            id.set("emagnier")
            name.set("Étienne Magnier")
            email.set("etienne.magnier@gmail.com")
          }
        }

        scm {
          connection.set("scm:git:git://github.com/romcal/romcal.git")
          developerConnection.set("scm:git:ssh://github.com/romcal/romcal.git")
          url.set("https://github.com/romcal/romcal")
        }
      }
    }
  }
}
