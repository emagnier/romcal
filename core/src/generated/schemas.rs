//! Auto-generated JSON schema constants - Do not modify manually
//! Regenerate with: cargo run --bin generate-schema --features schema-gen

/// JSON Schema for CalendarDefinition validation
pub const CALENDAR_DEFINITION_SCHEMA: &str = r##"{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "title": "CalendarDefinition",
  "description": "Calendar definition",
  "type": "object",
  "properties": {
    "$schema": {
      "type": [
        "string",
        "null"
      ]
    },
    "id": {
      "type": "string"
    },
    "metadata": {
      "$ref": "#/definitions/CalendarMetadata"
    },
    "particular_config": {
      "anyOf": [
        {
          "$ref": "#/definitions/ParticularConfig"
        },
        {
          "type": "null"
        }
      ]
    },
    "parent_calendar_ids": {
      "type": "array",
      "items": {
        "type": "string"
      }
    },
    "days_definitions": {
      "type": "object",
      "additionalProperties": {
        "$ref": "#/definitions/DayDefinition"
      }
    }
  },
  "required": [
    "id",
    "metadata",
    "parent_calendar_ids",
    "days_definitions"
  ],
  "additionalProperties": false,
  "definitions": {
    "CalendarMetadata": {
      "description": "Metadata for a calendar.\nContains essential information about the calendar's type and jurisdiction.",
      "type": "object",
      "properties": {
        "type": {
          "description": "The type of the calendar",
          "$ref": "#/definitions/CalendarType"
        },
        "jurisdiction": {
          "description": "The jurisdiction of the calendar",
          "$ref": "#/definitions/CalendarJurisdiction"
        }
      },
      "required": [
        "type",
        "jurisdiction"
      ],
      "additionalProperties": false
    },
    "CalendarType": {
      "description": "The type of the calendar.\nDefines the scope and authority level of the liturgical calendar.",
      "oneOf": [
        {
          "description": "General Roman Calendar (universal)",
          "type": "string",
          "const": "GENERAL_ROMAN"
        },
        {
          "description": "Regional calendar (multiple countries)",
          "type": "string",
          "const": "REGION"
        },
        {
          "description": "National calendar (single country)",
          "type": "string",
          "const": "COUNTRY"
        },
        {
          "description": "Archdiocesan calendar",
          "type": "string",
          "const": "ARCHDIOCESE"
        },
        {
          "description": "Diocesan calendar",
          "type": "string",
          "const": "DIOCESE"
        },
        {
          "description": "City calendar",
          "type": "string",
          "const": "CITY"
        },
        {
          "description": "Parish calendar",
          "type": "string",
          "const": "PARISH"
        },
        {
          "description": "General religious community calendar",
          "type": "string",
          "const": "GENERAL_COMMUNITY"
        },
        {
          "description": "Regional religious community calendar",
          "type": "string",
          "const": "REGIONAL_COMMUNITY"
        },
        {
          "description": "Local religious community calendar",
          "type": "string",
          "const": "LOCAL_COMMUNITY"
        },
        {
          "description": "Other specialized calendar",
          "type": "string",
          "const": "OTHER"
        }
      ]
    },
    "CalendarJurisdiction": {
      "description": "The jurisdiction of the calendar.\nDetermines whether the calendar follows ecclesiastical or civil authority.",
      "oneOf": [
        {
          "description": "Calendar under ecclesiastical authority (Church)",
          "type": "string",
          "const": "ECCLESIASTICAL"
        },
        {
          "description": "Calendar under civil authority (State)",
          "type": "string",
          "const": "CIVIL"
        }
      ]
    },
    "ParticularConfig": {
      "description": "Configuration options for \"particular\" (local/diocesan) calendars.\n\nIn liturgical terminology, a \"particular\" calendar is one that applies to a specific\nregion, diocese, or religious community, as opposed to the General Roman Calendar\nwhich applies universally.\n\nThese settings can override or extend the default Romcal configuration or any parent\ncalendar configuration.",
      "type": "object",
      "properties": {
        "epiphany_on_sunday": {
          "description": "Epiphany is celebrated on a Sunday",
          "type": [
            "boolean",
            "null"
          ]
        },
        "ascension_on_sunday": {
          "description": "Ascension is celebrated on a Sunday",
          "type": [
            "boolean",
            "null"
          ]
        },
        "corpus_christi_on_sunday": {
          "description": "Corpus Christi is celebrated on a Sunday",
          "type": [
            "boolean",
            "null"
          ]
        },
        "easter_calculation_type": {
          "description": "The type of Easter calculation",
          "anyOf": [
            {
              "$ref": "#/definitions/EasterCalculationType"
            },
            {
              "type": "null"
            }
          ]
        }
      },
      "additionalProperties": false
    },
    "EasterCalculationType": {
      "description": "Easter date calculation method.\n\nDetermines which algorithm to use for calculating the date of Easter Sunday,\nwhich is the basis for most movable feasts in the liturgical calendar.",
      "oneOf": [
        {
          "description": "Gregorian calculation (default)",
          "type": "string",
          "const": "GREGORIAN"
        },
        {
          "description": "Julian calculation converted to Gregorian",
          "type": "string",
          "const": "JULIAN"
        }
      ]
    },
    "DayDefinition": {
      "description": "Definition of a liturgical day with all its properties and configurations.\nIt represents a complete liturgical day definition that can be used\nto generate calendar entries with proper precedence, colors, and entity associations.",
      "type": "object",
      "properties": {
        "date_def": {
          "description": "The date definition for this liturgical day",
          "anyOf": [
            {
              "$ref": "#/definitions/DateDef"
            },
            {
              "type": "null"
            }
          ]
        },
        "date_exceptions": {
          "description": "The date definition exceptions (overrides for specific circumstances)",
          "anyOf": [
            {
              "$ref": "#/definitions/DateDefExceptions"
            },
            {
              "type": "null"
            }
          ]
        },
        "precedence": {
          "description": "The precedence type of the liturgical day",
          "anyOf": [
            {
              "$ref": "#/definitions/Precedence"
            },
            {
              "type": "null"
            }
          ]
        },
        "commons_def": {
          "description": "The **Common** refers to a set of prayers, readings, and chants used for celebrating saints or\nfeasts that belong to a specific category, such as martyrs, virgins, pastors, or the Blessed\nVirgin Mary.",
          "anyOf": [
            {
              "$ref": "#/definitions/CommonsDef"
            },
            {
              "type": "null"
            }
          ]
        },
        "is_holy_day_of_obligation": {
          "description": "Holy days of obligation are days on which the faithful are expected to attend Mass\nand engage in rest from work and recreation",
          "type": [
            "boolean",
            "null"
          ]
        },
        "allow_similar_rank_items": {
          "description": "Allow similar items that have the same rank and the same or lower precedence\nto coexist with this liturgical day without being overwritten",
          "type": [
            "boolean",
            "null"
          ]
        },
        "is_optional": {
          "description": "Specify if this liturgical day is optional within a specific liturgical calendar\n\nUNLY #14:\nMemorials are either obligatory or optional; their observance is integrated into\nthe celebration of the occurring weekday in accordance with the norms set forth in the\nGeneral Instruction of the Roman Missal and of the Liturgy of the Hours\n\nNote: also used for the dedication of consecrated churches, which is an optional solemnity\nthat should not overwrite the default weekday.",
          "type": [
            "boolean",
            "null"
          ]
        },
        "custom_locale_id": {
          "description": "The custom locale ID for this date definition in this calendar",
          "type": [
            "string",
            "null"
          ]
        },
        "entities": {
          "description": "The entities (Saints, Blessed, or Places) linked from the Entity catalog",
          "type": [
            "array",
            "null"
          ],
          "items": {
            "$ref": "#/definitions/EntityRef"
          }
        },
        "titles": {
          "description": "The combined titles of all entities linked to this date definition",
          "anyOf": [
            {
              "$ref": "#/definitions/TitlesDef"
            },
            {
              "type": "null"
            }
          ]
        },
        "drop": {
          "description": "If this liturgical day must be removed from this calendar and from all parent calendars\nin the final calendar generated by romcal",
          "type": [
            "boolean",
            "null"
          ]
        },
        "colors": {
          "description": "The liturgical color(s) of the liturgical day.\n\n**Deprecated:** Rely on the `titles` field of entities instead to determine the liturgical color(s).",
          "anyOf": [
            {
              "$ref": "#/definitions/ColorsDef"
            },
            {
              "type": "null"
            }
          ]
        },
        "masses": {
          "description": "The masses definitions for this liturgical day",
          "anyOf": [
            {
              "$ref": "#/definitions/MassesDefinitions"
            },
            {
              "type": "null"
            }
          ]
        }
      },
      "additionalProperties": false
    },
    "DateDef": {
      "description": "Date definition supporting various date calculation methods.\nProvides flexible ways to specify liturgical dates using different approaches.",
      "anyOf": [
        {
          "description": "Simple month/day specification",
          "type": "object",
          "properties": {
            "month": {
              "description": "The month (1-12)",
              "$ref": "#/definitions/MonthIndex"
            },
            "date": {
              "description": "The day of the month (1-31)",
              "type": "integer",
              "format": "uint8",
              "minimum": 0,
              "maximum": 255
            },
            "day_offset": {
              "description": "Optional day offset for adjustments",
              "type": [
                "integer",
                "null"
              ],
              "format": "int32"
            }
          },
          "required": [
            "month",
            "date"
          ],
          "additionalProperties": false
        },
        {
          "description": "Date function calculation (Easter, Epiphany, etc.)",
          "type": "object",
          "properties": {
            "date_fn": {
              "description": "The date function to calculate the base date",
              "$ref": "#/definitions/DateFn"
            },
            "day_offset": {
              "description": "Optional day offset for adjustments",
              "type": [
                "integer",
                "null"
              ],
              "format": "int32"
            }
          },
          "required": [
            "date_fn"
          ],
          "additionalProperties": false
        },
        {
          "description": "Nth weekday of a specific month",
          "type": "object",
          "properties": {
            "month": {
              "description": "The month (1-12)",
              "$ref": "#/definitions/MonthIndex"
            },
            "day_of_week": {
              "description": "The day of the week (0=Sunday, 1=Monday, etc.)",
              "$ref": "#/definitions/DayOfWeek"
            },
            "nth_week_in_month": {
              "description": "Which occurrence of the weekday (1st, 2nd, 3rd, etc.)",
              "type": "integer",
              "format": "uint8",
              "minimum": 0,
              "maximum": 255
            },
            "day_offset": {
              "description": "Optional day offset for adjustments",
              "type": [
                "integer",
                "null"
              ],
              "format": "int32"
            }
          },
          "required": [
            "month",
            "day_of_week",
            "nth_week_in_month"
          ],
          "additionalProperties": false
        },
        {
          "description": "Last weekday of a specific month",
          "type": "object",
          "properties": {
            "month": {
              "description": "The month (1-12)",
              "$ref": "#/definitions/MonthIndex"
            },
            "last_day_of_week_in_month": {
              "description": "The day of the week to find the last occurrence of",
              "$ref": "#/definitions/DayOfWeek"
            },
            "day_offset": {
              "description": "Optional day offset for adjustments",
              "type": [
                "integer",
                "null"
              ],
              "format": "int32"
            }
          },
          "required": [
            "month",
            "last_day_of_week_in_month"
          ],
          "additionalProperties": false
        },
        {
          "description": "Inherited from the proper of time",
          "type": "object",
          "additionalProperties": false
        }
      ]
    },
    "MonthIndex": {
      "description": "Month index (1-12) with automatic validation\n\nThis type ensures that only valid month values are accepted during\ndeserialization. The value 1 represents January, 2 represents February, etc.",
      "type": "integer",
      "format": "uint8",
      "minimum": 0,
      "maximum": 255
    },
    "DateFn": {
      "description": "Date function for calculating liturgical dates.\n\nRepresents movable feasts and special celebrations that require calculation\nbased on Easter or other variable dates.",
      "oneOf": [
        {
          "description": "Monday after Pentecost.",
          "type": "string",
          "const": "MARY_MOTHER_OF_THE_CHURCH"
        },
        {
          "description": "Sunday between January 2 and 8 (or January 6 if not transferred).",
          "type": "string",
          "const": "EPIPHANY_SUNDAY"
        },
        {
          "description": "February 2 (Candlemas).",
          "type": "string",
          "const": "PRESENTATION_OF_THE_LORD"
        },
        {
          "description": "March 25 (may be transferred if in Holy Week or Easter Octave).",
          "type": "string",
          "const": "ANNUNCIATION"
        },
        {
          "description": "Sunday before Easter.",
          "type": "string",
          "const": "PALM_SUNDAY"
        },
        {
          "description": "First Sunday after the Paschal Full Moon.",
          "type": "string",
          "const": "EASTER_SUNDAY"
        },
        {
          "description": "Second Sunday of Easter.",
          "type": "string",
          "const": "DIVINE_MERCY_SUNDAY"
        },
        {
          "description": "Saturday after the Second Sunday after Pentecost.",
          "type": "string",
          "const": "IMMACULATE_HEART_OF_MARY"
        },
        {
          "description": "Seventh Sunday after Easter.",
          "type": "string",
          "const": "PENTECOST_SUNDAY"
        },
        {
          "description": "Thursday or Sunday after Trinity Sunday.",
          "type": "string",
          "const": "CORPUS_CHRISTI_SUNDAY"
        },
        {
          "description": "June 24.",
          "type": "string",
          "const": "NATIVITY_OF_JOHN_THE_BAPTIST"
        },
        {
          "description": "June 29.",
          "type": "string",
          "const": "PETER_AND_PAUL_APOSTLES"
        },
        {
          "description": "August 6.",
          "type": "string",
          "const": "TRANSFIGURATION"
        },
        {
          "description": "August 15.",
          "type": "string",
          "const": "ASSUMPTION"
        },
        {
          "description": "September 14.",
          "type": "string",
          "const": "EXALTATION_OF_THE_HOLY_CROSS"
        },
        {
          "description": "November 1.",
          "type": "string",
          "const": "ALL_SAINTS"
        },
        {
          "description": "December 8.",
          "type": "string",
          "const": "IMMACULATE_CONCEPTION_OF_MARY"
        }
      ]
    },
    "DayOfWeek": {
      "description": "Day of week (0-6, where 0=Sunday) with automatic validation\n\nThis type ensures that only valid day-of-week values are accepted during\ndeserialization. The value 0 represents Sunday, 1 represents Monday, etc.",
      "type": "integer",
      "format": "uint8",
      "minimum": 0,
      "maximum": 255
    },
    "DateDefExceptions": {
      "description": "Date exceptions that can be either a single exception or multiple exceptions.\nSupports both simple single exceptions and complex multiple exception scenarios.",
      "anyOf": [
        {
          "description": "Single date exception",
          "$ref": "#/definitions/DateDefException"
        },
        {
          "description": "Multiple date exceptions",
          "type": "array",
          "items": {
            "$ref": "#/definitions/DateDefException"
          }
        }
      ]
    },
    "DateDefException": {
      "description": "The liturgical day date exception.\nRepresents a condition and the date to set when that condition is met.",
      "type": "object",
      "properties": {
        "when": {
          "description": "The condition that triggers the exception",
          "$ref": "#/definitions/ExceptionCondition"
        },
        "then": {
          "description": "The date to set when the condition is met",
          "$ref": "#/definitions/DateDefExtended"
        }
      },
      "required": [
        "when",
        "then"
      ],
      "additionalProperties": false
    },
    "ExceptionCondition": {
      "description": "Exception conditions that can trigger a date change.\nDefines various conditions under which a date exception applies.",
      "anyOf": [
        {
          "description": "If the date is between two specified dates",
          "type": "object",
          "properties": {
            "from": {
              "description": "The start date of the range",
              "$ref": "#/definitions/DateDef"
            },
            "to": {
              "description": "The end date of the range",
              "$ref": "#/definitions/DateDef"
            },
            "inclusive": {
              "description": "Whether the range is inclusive of the start date and the end date",
              "type": "boolean"
            }
          },
          "required": [
            "from",
            "to",
            "inclusive"
          ],
          "additionalProperties": false
        },
        {
          "description": "If the date is the same as another specified date",
          "type": "object",
          "properties": {
            "date": {
              "description": "The date to compare against",
              "$ref": "#/definitions/DateDef"
            }
          },
          "required": [
            "date"
          ],
          "additionalProperties": false
        },
        {
          "description": "If the date falls on a specific day of the week",
          "type": "object",
          "properties": {
            "day_of_week": {
              "description": "The day of the week to match",
              "$ref": "#/definitions/DayOfWeek"
            }
          },
          "required": [
            "day_of_week"
          ],
          "additionalProperties": false
        }
      ]
    },
    "DateDefExtended": {
      "description": "Extended date definition supporting both regular dates and offset dates.\nProvides flexibility for date calculations with optional adjustments.",
      "anyOf": [
        {
          "description": "Regular date definition",
          "$ref": "#/definitions/DateDef"
        },
        {
          "description": "Date definition with offset",
          "$ref": "#/definitions/DateDefWithOffset"
        }
      ]
    },
    "DateDefWithOffset": {
      "description": "Date definition with offset for adjustments.\nUsed when a date needs to be shifted by a specific number of days.",
      "type": "object",
      "properties": {
        "day_offset": {
          "description": "The number of days to offset the date",
          "type": "integer",
          "format": "int32"
        }
      },
      "required": [
        "day_offset"
      ],
      "additionalProperties": false
    },
    "Precedence": {
      "description": "Liturgical precedence levels for determining which celebration takes priority.\nDefines the hierarchical order of liturgical celebrations according to UNLY norms.",
      "oneOf": [
        {
          "description": "1 - The Paschal Triduum of the Passion and Resurrection of the Lord.",
          "type": "string",
          "const": "TRIDUUM_1"
        },
        {
          "description": "2 - The Nativity of the Lord, the Epiphany, the Ascension, or Pentecost.",
          "type": "string",
          "const": "PROPER_OF_TIME_SOLEMNITY_2"
        },
        {
          "description": "2 - A Sunday of Advent, Lent, or Easter.",
          "type": "string",
          "const": "PRIVILEGED_SUNDAY_2"
        },
        {
          "description": "2 - Ash Wednesday.",
          "type": "string",
          "const": "ASH_WEDNESDAY_2"
        },
        {
          "description": "2 - A weekday of Holy Week from Monday up to and including Thursday.",
          "type": "string",
          "const": "WEEKDAY_OF_HOLY_WEEK_2"
        },
        {
          "description": "2 - A day within the Octave of Easter.",
          "type": "string",
          "const": "WEEKDAY_OF_EASTER_OCTAVE_2"
        },
        {
          "description": "3 - A Solemnity inscribed in the General Calendar, whether of the Lord, of the Blessed Virgin Mary, or of a Saint.",
          "type": "string",
          "const": "GENERAL_SOLEMNITY_3"
        },
        {
          "description": "3 - The Commemoration of All the Faithful Departed.",
          "type": "string",
          "const": "COMMEMORATION_OF_ALL_THE_FAITHFUL_DEPARTED_3"
        },
        {
          "description": "4a - A proper Solemnity of the principal Patron of the place, city, or state.",
          "type": "string",
          "const": "PROPER_SOLEMNITY__PRINCIPAL_PATRON_4A"
        },
        {
          "description": "4b - The Solemnity of the dedication and of the anniversary of the dedication of the own church.",
          "type": "string",
          "const": "PROPER_SOLEMNITY__DEDICATION_OF_THE_OWN_CHURCH_4B"
        },
        {
          "description": "4c - The solemnity of the title of the own church.",
          "type": "string",
          "const": "PROPER_SOLEMNITY__TITLE_OF_THE_OWN_CHURCH_4C"
        },
        {
          "description": "4d - A Solemnity either of the Title or of the Founder or of the principal Patron of an Order or Congregation.",
          "type": "string",
          "const": "PROPER_SOLEMNITY__TITLE_OR_FOUNDER_OR_PRIMARY_PATRON_OF_A_RELIGIOUS_ORG_4D"
        },
        {
          "description": "5 - A Feast of the Lord inscribed in the General Calendar.",
          "type": "string",
          "const": "GENERAL_LORD_FEAST_5"
        },
        {
          "description": "6 - A Sunday of Christmas Time or a Sunday in Ordinary Time.",
          "type": "string",
          "const": "UNPRIVILEGED_SUNDAY_6"
        },
        {
          "description": "7 - A Feast of the Blessed Virgin Mary or of a Saint in the General Calendar.",
          "type": "string",
          "const": "GENERAL_FEAST_7"
        },
        {
          "description": "8a - The Proper Feast of the principal Patron of the diocese.",
          "type": "string",
          "const": "PROPER_FEAST__PRINCIPAL_PATRON_OF_A_DIOCESE_8A"
        },
        {
          "description": "8b - The Proper Feast of the anniversary of the dedication of the cathedral church.",
          "type": "string",
          "const": "PROPER_FEAST__DEDICATION_OF_THE_CATHEDRAL_CHURCH_8B"
        },
        {
          "description": "8c - The Proper Feast of the principal Patron of a region or province, or a country, or of a wider territory.",
          "type": "string",
          "const": "PROPER_FEAST__PRINCIPAL_PATRON_OF_A_REGION_8C"
        },
        {
          "description": "8d - The Proper Feast of the Title, Founder, or principal Patron of an Order or Congregation.",
          "type": "string",
          "const": "PROPER_FEAST__TITLE_OR_FOUNDER_OR_PRIMARY_PATRON_OF_A_RELIGIOUS_ORG_8D"
        },
        {
          "description": "8e - Other Feast, proper to an individual church.",
          "type": "string",
          "const": "PROPER_FEAST__TO_AN_INDIVIDUAL_CHURCH_8E"
        },
        {
          "description": "8f - Other Proper Feast inscribed in the Calendar of each diocese or Order or Congregation.",
          "type": "string",
          "const": "PROPER_FEAST_8F"
        },
        {
          "description": "9 - Privileged Weekday.",
          "type": "string",
          "const": "PRIVILEGED_WEEKDAY_9"
        },
        {
          "description": "10 - Obligatory Memorials in the General Calendar.",
          "type": "string",
          "const": "GENERAL_MEMORIAL_10"
        },
        {
          "description": "11a - Proper Obligatory Memorial of a secondary Patron of the place, diocese, region, or religious province.",
          "type": "string",
          "const": "PROPER_MEMORIAL__SECOND_PATRON_11A"
        },
        {
          "description": "11b - Other Proper Obligatory Memorial inscribed in the Calendar of each diocese, or Order or congregation.",
          "type": "string",
          "const": "PROPER_MEMORIAL_11B"
        },
        {
          "description": "12 - Optional Memorial.",
          "type": "string",
          "const": "OPTIONAL_MEMORIAL_12"
        },
        {
          "description": "13 - Weekday.",
          "type": "string",
          "const": "WEEKDAY_13"
        }
      ]
    },
    "CommonsDef": {
      "anyOf": [
        {
          "$ref": "#/definitions/CommonDefinition"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/definitions/CommonDefinition"
          }
        }
      ]
    },
    "CommonDefinition": {
      "description": "Common definition for simplified categorization.\nProvides a simplified version of the Common enum for easier classification.",
      "oneOf": [
        {
          "description": "No common.",
          "type": "string",
          "const": "NONE"
        },
        {
          "description": "Dedication anniversary (in the Church that was Dedicated).",
          "type": "string",
          "const": "DEDICATION_ANNIVERSARY__INSIDE"
        },
        {
          "description": "Dedication anniversary (outside the Church that was Dedicated).",
          "type": "string",
          "const": "DEDICATION_ANNIVERSARY__OUTSIDE"
        },
        {
          "description": "Common of the Blessed Virgin Mary.",
          "type": "string",
          "const": "BLESSED_VIRGIN_MARY"
        },
        {
          "description": "Common for Martyrs.",
          "type": "string",
          "const": "MARTYRS"
        },
        {
          "description": "Common for Missionary Martyrs.",
          "type": "string",
          "const": "MISSIONARY_MARTYRS"
        },
        {
          "description": "Common for Virgin Martyrs.",
          "type": "string",
          "const": "VIRGIN_MARTYRS"
        },
        {
          "description": "Common for Holy Woman Martyrs.",
          "type": "string",
          "const": "WOMAN_MARTYRS"
        },
        {
          "description": "Common for Pastors.",
          "type": "string",
          "const": "PASTORS"
        },
        {
          "description": "Common for Popes.",
          "type": "string",
          "const": "POPES"
        },
        {
          "description": "Common for Bishops.",
          "type": "string",
          "const": "BISHOPS"
        },
        {
          "description": "Common for Founders.",
          "type": "string",
          "const": "FOUNDERS"
        },
        {
          "description": "Common for Missionaries.",
          "type": "string",
          "const": "MISSIONARIES"
        },
        {
          "description": "Common for Doctors of the Church.",
          "type": "string",
          "const": "DOCTORS_OF_THE_CHURCH"
        },
        {
          "description": "Common for Virgins.",
          "type": "string",
          "const": "VIRGINS"
        },
        {
          "description": "Common for Holy Men and Women.",
          "type": "string",
          "const": "SAINTS"
        },
        {
          "description": "Common for Abbots.",
          "type": "string",
          "const": "ABBOTS"
        },
        {
          "description": "Common for Monks.",
          "type": "string",
          "const": "MONKS"
        },
        {
          "description": "Common for Nuns.",
          "type": "string",
          "const": "NUNS"
        },
        {
          "description": "Common for Religious.",
          "type": "string",
          "const": "RELIGIOUS"
        },
        {
          "description": "Common for Those Who Practiced Works of Mercy.",
          "type": "string",
          "const": "MERCY_WORKERS"
        },
        {
          "description": "Common for Educators.",
          "type": "string",
          "const": "EDUCATORS"
        },
        {
          "description": "Common for Holy Women.",
          "type": "string",
          "const": "HOLY_WOMEN"
        }
      ]
    },
    "EntityRef": {
      "description": "A reference to an entity in the entity catalog.\nCan either reference an existing entity by ID or define a custom entity with additional properties.",
      "anyOf": [
        {
          "description": "Reference to an existing entity by its ID",
          "type": "string"
        },
        {
          "description": "Custom entity definition with additional properties specific to a liturgical day",
          "$ref": "#/definitions/EntityOverride"
        }
      ]
    },
    "EntityOverride": {
      "description": "Custom entity definition that extends or overrides properties from the entity catalog.\nUsed when a liturgical day needs specific entity properties that differ from the base entity.",
      "type": "object",
      "properties": {
        "id": {
          "description": "The ID of the entity item (must reference an existing entity in the catalog)",
          "type": "string"
        },
        "titles": {
          "description": "The custom titles for this entity in the context of this liturgical day",
          "anyOf": [
            {
              "$ref": "#/definitions/TitlesDef"
            },
            {
              "type": "null"
            }
          ]
        },
        "hide_titles": {
          "description": "Whether to hide titles when displaying this entity (useful when titles are already included in the entity name)",
          "type": [
            "boolean",
            "null"
          ]
        },
        "count": {
          "description": "The number of persons this entity represents (useful for groups of martyrs or saints)",
          "anyOf": [
            {
              "$ref": "#/definitions/SaintCount"
            },
            {
              "type": "null"
            }
          ]
        }
      },
      "required": [
        "id"
      ],
      "additionalProperties": false
    },
    "TitlesDef": {
      "description": "Title definition that can be either a simple list or a compound definition.\nSupports both direct title lists and compound title operations.",
      "anyOf": [
        {
          "description": "Simple list of titles",
          "type": "array",
          "items": {
            "$ref": "#/definitions/Title"
          }
        },
        {
          "description": "Compound title definition with append/prepend operations",
          "$ref": "#/definitions/CompoundTitle"
        }
      ]
    },
    "Title": {
      "description": "Titles and patronages associated with saints and blessed.\nRepresents the various ecclesiastical titles and patronages that can be assigned to entities.",
      "type": "string",
      "enum": [
        "ABBESS",
        "ABBOT",
        "APOSTLE",
        "ARCHANGEL",
        "BISHOP",
        "DEACON",
        "DOCTOR_OF_THE_CHURCH",
        "EMPRESS",
        "EVANGELIST",
        "FIRST_BISHOP",
        "HERMIT",
        "KING",
        "MARTYR",
        "MISSIONARY",
        "MONK",
        "MOTHER_AND_QUEEN_OF_CHILE",
        "PARENTS_OF_THE_BLESSED_VIRGIN_MARY",
        "POPE",
        "PATRIARCH",
        "PILGRIM",
        "PRIEST",
        "PROPHET",
        "PROTO_MARTYR_OF_OCEANIA",
        "QUEEN",
        "QUEEN_OF_POLAND",
        "RELIGIOUS",
        "SLAVIC_MISSIONARY",
        "SPOUSE_OF_THE_BLESSED_VIRGIN_MARY",
        "THE_FIRST_MARTYR",
        "VIRGIN",
        "COPATRON_OF_EUROPE",
        "COPATRON_OF_IRELAND",
        "COPATRON_OF_CANADA",
        "COPATRONESS_OF_EUROPE",
        "COPATRONESS_OF_FRANCE",
        "COPATRONESS_OF_IRELAND",
        "COPATRONESS_OF_ITALY_AND_EUROPE",
        "COPATRONESS_OF_THE_PHILIPPINES",
        "PATRON_OF_CANADA",
        "PATRON_OF_ENGLAND",
        "PATRON_OF_EUROPE",
        "PATRON_OF_FRANCE",
        "PATRON_OF_IRELAND",
        "PATRON_OF_ITALY",
        "PATRON_OF_OCEANIA",
        "PATRON_OF_POLAND",
        "PATRON_OF_RUSSIA",
        "PATRON_OF_SCOTLAND",
        "PATRON_OF_SPAIN",
        "PATRON_OF_THE_CZECH_NATION",
        "PATRON_OF_THE_DIOCESE",
        "PATRON_OF_WALES",
        "PATRONESS_OF_ALSACE",
        "PATRONESS_OF_ARGENTINA",
        "PATRONESS_OF_BRAZIL",
        "PATRONESS_OF_HUNGARY",
        "PATRONESS_OF_PUERTO_RICO",
        "PATRONESS_OF_SLOVAKIA",
        "PATRONESS_OF_THE_AMERICAS",
        "PATRONESS_OF_THE_PHILIPPINES",
        "PATRONESS_OF_THE_PROVINCE_OF_QUEBEC",
        "PATRONESS_OF_THE_USA",
        "PATRON_OF_THE_CLERGY_OF_THE_ARCHDIOCESE_OF_LYON",
        "PATRON_OF_THE_CITY_OF_LYON",
        "PATRONESS_OF_COSTA_RICA",
        "PRINCIPAL_PATRON_OF_THE_DIOCESE",
        "SECOND_PATRON_OF_THE_DIOCESE"
      ]
    },
    "CompoundTitle": {
      "description": "Compound title definition for combining multiple titles.\nAllows adding titles to the beginning or end of an existing title list.",
      "type": "object",
      "properties": {
        "append": {
          "description": "The title(s) to add to the end of the existing list of title(s)",
          "type": [
            "array",
            "null"
          ],
          "items": {
            "$ref": "#/definitions/Title"
          }
        },
        "prepend": {
          "description": "The title(s) to add to the beginning of the existing list of title(s)",
          "type": [
            "array",
            "null"
          ],
          "items": {
            "$ref": "#/definitions/Title"
          }
        }
      },
      "additionalProperties": false
    },
    "SaintCount": {
      "description": "Represents the number of saints for an entity or a group of entities.\n\nCan be either a specific number (u32) or \"MANY\" to indicate\nan indeterminate number of saints.\n\n# Serialization\n- `Number(n)` serializes as integer `n`\n- `Many` serializes as string `\"MANY\"`\n\n# Deserialization\n- Integers are converted to `Number(u32)`\n- String `\"MANY\"` is converted to `Many`\n- All other types generate an error",
      "anyOf": [
        {
          "type": "integer",
          "format": "uint32",
          "minimum": 0
        },
        {
          "type": "string",
          "const": "MANY"
        },
        {
          "type": "null"
        }
      ]
    },
    "ColorsDef": {
      "anyOf": [
        {
          "$ref": "#/definitions/Color"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/definitions/Color"
          }
        }
      ]
    },
    "Color": {
      "description": "Liturgical colors used in the celebration of Mass and other liturgical services.\nEach color has specific liturgical significance and is used during particular seasons or celebrations.",
      "oneOf": [
        {
          "description": "Red - used for martyrs, Pentecost, and Palm Sunday",
          "type": "string",
          "const": "RED"
        },
        {
          "description": "Rose - used on Gaudete Sunday (3rd Advent) and Laetare Sunday (4th Lent)",
          "type": "string",
          "const": "ROSE"
        },
        {
          "description": "Purple - used during Advent and Lent",
          "type": "string",
          "const": "PURPLE"
        },
        {
          "description": "Green - used during Ordinary Time",
          "type": "string",
          "const": "GREEN"
        },
        {
          "description": "White - used for Christmas, Easter, and most feasts",
          "type": "string",
          "const": "WHITE"
        },
        {
          "description": "Gold - used for solemn celebrations and special occasions",
          "type": "string",
          "const": "GOLD"
        },
        {
          "description": "Black - used for funerals and All Souls' Day",
          "type": "string",
          "const": "BLACK"
        }
      ]
    },
    "MassesDefinitions": {
      "description": "All mass definitions for a liturgical day",
      "type": "object",
      "properties": {
        "celebration_of_the_passion": {
          "description": "Celebration of the Passion - special celebration of Christ's passion",
          "$ref": "#/definitions/MassCycleDefinition"
        },
        "chrism_mass": {
          "description": "Chrism Mass - Mass where holy oils are blessed, typically on Holy Thursday morning",
          "$ref": "#/definitions/MassCycleDefinition"
        },
        "day_mass": {
          "description": "Day Mass - regular Mass celebrated during the day",
          "$ref": "#/definitions/MassCycleDefinition"
        },
        "easter_vigil": {
          "description": "Easter Vigil - the most important Mass of the liturgical year, celebrated on Holy Saturday night",
          "$ref": "#/definitions/MassCycleDefinition"
        },
        "evening_mass_of_the_lords_supper": {
          "description": "Evening Mass of the Lord's Supper - Mass celebrated on Holy Thursday evening",
          "$ref": "#/definitions/MassCycleDefinition"
        },
        "mass_at_dawn": {
          "description": "Mass at Dawn - Mass celebrated at dawn, particularly on Easter Sunday",
          "$ref": "#/definitions/MassCycleDefinition"
        },
        "mass_of_the_passion": {
          "description": "Mass of the Passion - Mass focusing on Christ's passion, beginning with the procession with palms",
          "$ref": "#/definitions/MassCycleDefinition"
        },
        "morning_mass": {
          "description": "Morning Mass - Mass celebrated in the morning",
          "$ref": "#/definitions/MassCycleDefinition"
        },
        "night_mass": {
          "description": "Night Mass - Mass celebrated during the night hours",
          "$ref": "#/definitions/MassCycleDefinition"
        },
        "previous_evening_mass": {
          "description": "Previous Evening Mass - Mass celebrated the evening before a major feast",
          "$ref": "#/definitions/MassCycleDefinition"
        }
      },
      "additionalProperties": false
    },
    "MassCycleDefinition": {
      "description": "Mass contents for a specific mass time, organized by liturgical cycle",
      "type": "object",
      "properties": {
        "invariant": {
          "description": "Invariant content that applies to all cycles",
          "$ref": "#/definitions/MassContent"
        },
        "year_1": {
          "description": "Year 1 of the weekday cycle (Cycle I)",
          "$ref": "#/definitions/MassContent"
        },
        "year_2": {
          "description": "Year 2 of the weekday cycle (Cycle II)",
          "$ref": "#/definitions/MassContent"
        },
        "year_a": {
          "description": "Year A of the Sunday cycle",
          "$ref": "#/definitions/MassContent"
        },
        "year_a_b": {
          "description": "Combined years A and B of the Sunday cycle",
          "$ref": "#/definitions/MassContent"
        },
        "year_a_c": {
          "description": "Combined years A and C of the Sunday cycle",
          "$ref": "#/definitions/MassContent"
        },
        "year_b": {
          "description": "Year B of the Sunday cycle",
          "$ref": "#/definitions/MassContent"
        },
        "year_b_c": {
          "description": "Combined years B and C of the Sunday cycle",
          "$ref": "#/definitions/MassContent"
        },
        "year_c": {
          "description": "Year C of the Sunday cycle",
          "$ref": "#/definitions/MassContent"
        }
      },
      "additionalProperties": false
    },
    "MassContent": {
      "description": "Content of a mass for a specific liturgical cycle\nMaps mass parts (readings, psalms, prayers, antiphons, etc.) to their texts",
      "type": "object",
      "properties": {
        "alleluia": {
          "description": "Alleluia - acclamation before the Gospel",
          "type": "string"
        },
        "canticle": {
          "description": "Canticle - biblical canticle",
          "type": "string"
        },
        "collect": {
          "description": "Collect - opening prayer of the Mass",
          "type": "string"
        },
        "communion_antiphon": {
          "description": "Communion Antiphon - chant during communion",
          "type": "string"
        },
        "easter_vigil_canticle_3": {
          "description": "Canticle 3 (Easter Vigil)",
          "type": "string"
        },
        "easter_vigil_canticle_5": {
          "description": "Canticle 5 (Easter Vigil)",
          "type": "string"
        },
        "easter_vigil_epistle": {
          "description": "Epistle - reading from the epistles (Easter Vigil)",
          "type": "string"
        },
        "easter_vigil_psalm_2": {
          "description": "Psalm 2 (Easter Vigil)",
          "type": "string"
        },
        "easter_vigil_psalm_4": {
          "description": "Psalm 4 (Easter Vigil)",
          "type": "string"
        },
        "easter_vigil_psalm_6": {
          "description": "Psalm 6 (Easter Vigil)",
          "type": "string"
        },
        "easter_vigil_psalm_7": {
          "description": "Psalm 7 (Easter Vigil)",
          "type": "string"
        },
        "easter_vigil_reading_3": {
          "description": "Reading 3 - third reading (Easter Vigil)",
          "type": "string"
        },
        "easter_vigil_reading_4": {
          "description": "Reading 4 - fourth reading (Easter Vigil)",
          "type": "string"
        },
        "easter_vigil_reading_5": {
          "description": "Reading 5 - fifth reading (Easter Vigil)",
          "type": "string"
        },
        "easter_vigil_reading_6": {
          "description": "Reading 6 - sixth reading (Easter Vigil)",
          "type": "string"
        },
        "easter_vigil_reading_7": {
          "description": "Reading 7 - seventh reading (Easter Vigil)",
          "type": "string"
        },
        "entrance_antiphon": {
          "description": "Entrance Antiphon - opening chant of the Mass",
          "type": "string"
        },
        "gospel": {
          "description": "Gospel - reading from the Gospels",
          "type": "string"
        },
        "messianic_entry": {
          "description": "Messianic entry reading (during the procession with palms, before the Mass of the Passion)",
          "type": "string"
        },
        "prayer_after_communion": {
          "description": "Prayer after Communion - concluding prayer",
          "type": "string"
        },
        "prayer_over_the_offerings": {
          "description": "Prayer over the Offerings - prayer during the offertory",
          "type": "string"
        },
        "prayer_over_the_people": {
          "description": "Prayer over the People - blessing over the congregation",
          "type": "string"
        },
        "preface": {
          "description": "Preface - introduction to the Eucharistic Prayer",
          "type": "string"
        },
        "psalm": {
          "description": "Psalm - responsorial psalm",
          "type": "string"
        },
        "reading_1": {
          "description": "Reading 1 - first reading (usually from the Old Testament)",
          "type": "string"
        },
        "reading_2": {
          "description": "Reading 2 - second reading (usually from the New Testament)",
          "type": "string"
        },
        "sequence": {
          "description": "Sequence - special chant on certain feasts",
          "type": "string"
        },
        "solemn_blessing": {
          "description": "Solemn Blessing - special blessing on certain occasions",
          "type": "string"
        }
      },
      "additionalProperties": false
    }
  }
}"##;

/// JSON Schema for Resources validation
pub const RESOURCES_SCHEMA: &str = r##"{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "title": "Resources",
  "description": "Resources definition",
  "type": "object",
  "properties": {
    "$schema": {
      "type": [
        "string",
        "null"
      ]
    },
    "locale": {
      "description": "Locale code of the resources, in BCP-47 IETF tag format",
      "type": "string"
    },
    "metadata": {
      "description": "Metadata of the resources",
      "anyOf": [
        {
          "$ref": "#/definitions/ResourcesMetadata"
        },
        {
          "type": "null"
        }
      ]
    },
    "entities": {
      "description": "Entities of the resources: a person, a place, an event, etc.",
      "type": [
        "object",
        "null"
      ],
      "additionalProperties": {
        "$ref": "#/definitions/Entity"
      }
    }
  },
  "required": [
    "locale"
  ],
  "additionalProperties": false,
  "definitions": {
    "ResourcesMetadata": {
      "description": "Metadata for localized resources.\nContains all the localized strings and configurations for a specific locale.",
      "type": "object",
      "properties": {
        "ordinal_format": {
          "description": "Format for displaying ordinal numbers (defaults to Numeric if not specified)",
          "anyOf": [
            {
              "$ref": "#/definitions/OrdinalFormat"
            },
            {
              "type": "null"
            }
          ]
        },
        "ordinals_letters": {
          "description": "Ordinal numbers as words (first, second, third, etc.) in the locale language",
          "type": [
            "object",
            "null"
          ],
          "additionalProperties": {
            "type": "string"
          }
        },
        "ordinals_numeric": {
          "description": "Ordinal numbers as numeric with suffix (1st, 2nd, 3rd, etc.) in the locale language",
          "type": [
            "object",
            "null"
          ],
          "additionalProperties": {
            "type": "string"
          }
        },
        "weekdays": {
          "description": "Weekday names (Sunday, Monday, etc.) in the locale language",
          "type": [
            "object",
            "null"
          ],
          "additionalProperties": {
            "type": "string"
          }
        },
        "months": {
          "description": "Month names (January, February, etc.) in the locale language",
          "type": [
            "object",
            "null"
          ],
          "additionalProperties": {
            "type": "string"
          }
        },
        "colors": {
          "description": "Liturgical color names in the locale language",
          "anyOf": [
            {
              "$ref": "#/definitions/LocaleColors"
            },
            {
              "type": "null"
            }
          ]
        },
        "seasons": {
          "description": "Liturgical season names and descriptions in the locale language",
          "anyOf": [
            {
              "$ref": "#/definitions/SeasonsMetadata"
            },
            {
              "type": "null"
            }
          ]
        },
        "periods": {
          "description": "Liturgical period names in the locale language",
          "anyOf": [
            {
              "$ref": "#/definitions/PeriodsMetadata"
            },
            {
              "type": "null"
            }
          ]
        },
        "ranks": {
          "description": "Liturgical rank names in the locale language",
          "anyOf": [
            {
              "$ref": "#/definitions/RanksMetadata"
            },
            {
              "type": "null"
            }
          ]
        },
        "cycles": {
          "description": "Liturgical cycle names in the locale language",
          "anyOf": [
            {
              "$ref": "#/definitions/CyclesMetadata"
            },
            {
              "type": "null"
            }
          ]
        }
      },
      "additionalProperties": false
    },
    "OrdinalFormat": {
      "description": "Format for displaying ordinal numbers.\n\n- `Letters`: Display ordinals as words (e.g., \"first\", \"second\", \"premier\", \"deuxième\")\n- `Numeric`: Display ordinals as numbers with suffixes (e.g., \"1st\", \"2nd\", \"1er\", \"2e\")",
      "oneOf": [
        {
          "description": "Ordinals displayed as words",
          "type": "string",
          "const": "letters"
        },
        {
          "description": "Ordinals displayed as numbers with suffixes (default)",
          "type": "string",
          "const": "numeric"
        }
      ]
    },
    "LocaleColors": {
      "description": "Liturgical color names in the locale language.\nProvides localized names for each liturgical color.",
      "type": "object",
      "properties": {
        "black": {
          "description": "Black color name in the locale language",
          "type": [
            "string",
            "null"
          ]
        },
        "gold": {
          "description": "Gold color name in the locale language",
          "type": [
            "string",
            "null"
          ]
        },
        "green": {
          "description": "Green color name in the locale language",
          "type": [
            "string",
            "null"
          ]
        },
        "purple": {
          "description": "Purple color name in the locale language",
          "type": [
            "string",
            "null"
          ]
        },
        "red": {
          "description": "Red color name in the locale language",
          "type": [
            "string",
            "null"
          ]
        },
        "rose": {
          "description": "Rose color name in the locale language",
          "type": [
            "string",
            "null"
          ]
        },
        "white": {
          "description": "White color name in the locale language",
          "type": [
            "string",
            "null"
          ]
        }
      },
      "additionalProperties": false
    },
    "SeasonsMetadata": {
      "description": "Liturgical season names and descriptions in the locale language.\nProvides localized names for each liturgical season and their components.",
      "type": "object",
      "properties": {
        "advent": {
          "description": "Advent season names and descriptions",
          "anyOf": [
            {
              "$ref": "#/definitions/AdventSeason"
            },
            {
              "type": "null"
            }
          ]
        },
        "christmas_time": {
          "description": "Christmas Time season names and descriptions",
          "anyOf": [
            {
              "$ref": "#/definitions/ChristmasTimeSeason"
            },
            {
              "type": "null"
            }
          ]
        },
        "ordinary_time": {
          "description": "Ordinary Time season names and descriptions",
          "anyOf": [
            {
              "$ref": "#/definitions/OrdinaryTimeSeason"
            },
            {
              "type": "null"
            }
          ]
        },
        "lent": {
          "description": "Lent season names and descriptions",
          "anyOf": [
            {
              "$ref": "#/definitions/LentSeason"
            },
            {
              "type": "null"
            }
          ]
        },
        "paschal_triduum": {
          "description": "Paschal Triduum season names and descriptions",
          "anyOf": [
            {
              "$ref": "#/definitions/PaschalTriduumSeason"
            },
            {
              "type": "null"
            }
          ]
        },
        "easter_time": {
          "description": "Easter Time season names and descriptions",
          "anyOf": [
            {
              "$ref": "#/definitions/EasterTimeSeason"
            },
            {
              "type": "null"
            }
          ]
        }
      },
      "additionalProperties": false
    },
    "AdventSeason": {
      "description": "Advent season localized names and descriptions.\nProvides specific terminology for the Advent season in the locale language.",
      "type": "object",
      "properties": {
        "season": {
          "description": "General season name for Advent",
          "type": [
            "string",
            "null"
          ]
        },
        "weekday": {
          "description": "Weekday terminology during Advent",
          "type": [
            "string",
            "null"
          ]
        },
        "sunday": {
          "description": "Sunday terminology during Advent",
          "type": [
            "string",
            "null"
          ]
        },
        "privileged_weekday": {
          "description": "Privileged weekday terminology during Advent",
          "type": [
            "string",
            "null"
          ]
        }
      },
      "additionalProperties": false
    },
    "ChristmasTimeSeason": {
      "description": "Christmas Time season localized names and descriptions.",
      "type": "object",
      "properties": {
        "season": {
          "description": "General season name for Christmas Time",
          "type": [
            "string",
            "null"
          ]
        },
        "day": {
          "description": "Day terminology during Christmas Time",
          "type": [
            "string",
            "null"
          ]
        },
        "octave": {
          "description": "Octave terminology during Christmas Time",
          "type": [
            "string",
            "null"
          ]
        },
        "before_epiphany": {
          "description": "Before Epiphany terminology",
          "type": [
            "string",
            "null"
          ]
        },
        "second_sunday_after_christmas": {
          "description": "Second Sunday after Christmas terminology",
          "type": [
            "string",
            "null"
          ]
        },
        "after_epiphany": {
          "description": "After Epiphany terminology",
          "type": [
            "string",
            "null"
          ]
        }
      },
      "additionalProperties": false
    },
    "OrdinaryTimeSeason": {
      "description": "Ordinary Time season localized names and descriptions.",
      "type": "object",
      "properties": {
        "season": {
          "description": "General season name for Ordinary Time",
          "type": [
            "string",
            "null"
          ]
        },
        "weekday": {
          "description": "Weekday terminology during Ordinary Time",
          "type": [
            "string",
            "null"
          ]
        },
        "sunday": {
          "description": "Sunday terminology during Ordinary Time",
          "type": [
            "string",
            "null"
          ]
        }
      },
      "additionalProperties": false
    },
    "LentSeason": {
      "description": "Lent season localized names and descriptions.",
      "type": "object",
      "properties": {
        "season": {
          "description": "General season name for Lent",
          "type": [
            "string",
            "null"
          ]
        },
        "weekday": {
          "description": "Weekday terminology during Lent",
          "type": [
            "string",
            "null"
          ]
        },
        "sunday": {
          "description": "Sunday terminology during Lent",
          "type": [
            "string",
            "null"
          ]
        },
        "day_after_ash_wed": {
          "description": "Day after Ash Wednesday terminology",
          "type": [
            "string",
            "null"
          ]
        },
        "holy_week_day": {
          "description": "Holy Week day terminology",
          "type": [
            "string",
            "null"
          ]
        }
      },
      "additionalProperties": false
    },
    "PaschalTriduumSeason": {
      "description": "Paschal Triduum season localized names and descriptions.",
      "type": "object",
      "properties": {
        "season": {
          "description": "General season name for Paschal Triduum",
          "type": [
            "string",
            "null"
          ]
        }
      },
      "additionalProperties": false
    },
    "EasterTimeSeason": {
      "description": "Easter Time season localized names and descriptions.",
      "type": "object",
      "properties": {
        "season": {
          "description": "General season name for Easter Time",
          "type": [
            "string",
            "null"
          ]
        },
        "weekday": {
          "description": "Weekday terminology during Easter Time",
          "type": [
            "string",
            "null"
          ]
        },
        "sunday": {
          "description": "Sunday terminology during Easter Time",
          "type": [
            "string",
            "null"
          ]
        },
        "octave": {
          "description": "Octave terminology during Easter Time",
          "type": [
            "string",
            "null"
          ]
        }
      },
      "additionalProperties": false
    },
    "PeriodsMetadata": {
      "description": "Liturgical period names in the locale language.",
      "type": "object",
      "properties": {
        "christmas_octave": {
          "description": "Christmas Octave period name",
          "type": [
            "string",
            "null"
          ]
        },
        "days_before_epiphany": {
          "description": "Days before Epiphany period name",
          "type": [
            "string",
            "null"
          ]
        },
        "days_from_epiphany": {
          "description": "Days from Epiphany period name",
          "type": [
            "string",
            "null"
          ]
        },
        "christmas_to_presentation_of_the_lord": {
          "description": "Christmas to Presentation of the Lord period name",
          "type": [
            "string",
            "null"
          ]
        },
        "presentation_of_the_lord_to_holy_thursday": {
          "description": "Presentation of the Lord to Holy Thursday period name",
          "type": [
            "string",
            "null"
          ]
        },
        "holy_week": {
          "description": "Holy Week period name",
          "type": [
            "string",
            "null"
          ]
        },
        "paschal_triduum": {
          "description": "Paschal Triduum period name",
          "type": [
            "string",
            "null"
          ]
        },
        "easter_octave": {
          "description": "Easter Octave period name",
          "type": [
            "string",
            "null"
          ]
        },
        "early_ordinary_time": {
          "description": "Early Ordinary Time period name",
          "type": [
            "string",
            "null"
          ]
        },
        "late_ordinary_time": {
          "description": "Late Ordinary Time period name",
          "type": [
            "string",
            "null"
          ]
        }
      },
      "additionalProperties": false
    },
    "RanksMetadata": {
      "description": "Liturgical rank names in the locale language.",
      "type": "object",
      "properties": {
        "solemnity": {
          "description": "Solemnity rank name",
          "type": [
            "string",
            "null"
          ]
        },
        "sunday": {
          "description": "Sunday rank name",
          "type": [
            "string",
            "null"
          ]
        },
        "feast": {
          "description": "Feast rank name",
          "type": [
            "string",
            "null"
          ]
        },
        "memorial": {
          "description": "Memorial rank name",
          "type": [
            "string",
            "null"
          ]
        },
        "optional_memorial": {
          "description": "Optional memorial rank name",
          "type": [
            "string",
            "null"
          ]
        },
        "weekday": {
          "description": "Weekday rank name",
          "type": [
            "string",
            "null"
          ]
        }
      },
      "additionalProperties": false
    },
    "CyclesMetadata": {
      "description": "Liturgical cycle names in the locale language.",
      "type": "object",
      "properties": {
        "proper_of_time": {
          "description": "Proper of Time cycle name",
          "type": [
            "string",
            "null"
          ]
        },
        "proper_of_saints": {
          "description": "Proper of Saints cycle name",
          "type": [
            "string",
            "null"
          ]
        },
        "sunday_year_a": {
          "description": "Sunday Year A cycle name",
          "type": [
            "string",
            "null"
          ]
        },
        "sunday_year_b": {
          "description": "Sunday Year B cycle name",
          "type": [
            "string",
            "null"
          ]
        },
        "sunday_year_c": {
          "description": "Sunday Year C cycle name",
          "type": [
            "string",
            "null"
          ]
        },
        "weekday_year_1": {
          "description": "Weekday Year 1 cycle name",
          "type": [
            "string",
            "null"
          ]
        },
        "weekday_year_2": {
          "description": "Weekday Year 2 cycle name",
          "type": [
            "string",
            "null"
          ]
        },
        "psalter_week_1": {
          "description": "Psalter Week 1 cycle name",
          "type": [
            "string",
            "null"
          ]
        },
        "psalter_week_2": {
          "description": "Psalter Week 2 cycle name",
          "type": [
            "string",
            "null"
          ]
        },
        "psalter_week_3": {
          "description": "Psalter Week 3 cycle name",
          "type": [
            "string",
            "null"
          ]
        },
        "psalter_week_4": {
          "description": "Psalter Week 4 cycle name",
          "type": [
            "string",
            "null"
          ]
        }
      },
      "additionalProperties": false
    },
    "Entity": {
      "type": "object",
      "properties": {
        "id": {
          "description": "The unique identifier of the entity",
          "type": [
            "string",
            "null"
          ]
        },
        "type": {
          "description": "The type of the entity.\n\nDefaults to `EntityType::Person`.",
          "anyOf": [
            {
              "$ref": "#/definitions/EntityType"
            },
            {
              "type": "null"
            }
          ],
          "default": "PERSON"
        },
        "fullname": {
          "description": "The full name of the entity.",
          "type": [
            "string",
            "null"
          ],
          "writeOnly": true
        },
        "name": {
          "description": "The short name of the entity, without the canonization level and titles.",
          "type": [
            "string",
            "null"
          ]
        },
        "canonization_level": {
          "description": "The canonization level of a person.",
          "anyOf": [
            {
              "$ref": "#/definitions/CanonizationLevel"
            },
            {
              "type": "null"
            }
          ]
        },
        "date_of_canonization": {
          "description": "Date of Canonization, as a Number (year), a String (in 'YYYY-MM' or 'YYYY-MM-DD' format),\nor an object describing date range, multiple possible date, or a century.",
          "anyOf": [
            {
              "$ref": "#/definitions/SaintDateDef"
            },
            {
              "type": "null"
            }
          ]
        },
        "date_of_canonization_is_approximative": {
          "description": "Specify whether an approximate indicator should be added, when the date is displayed.\nFor example in English: 'c. 201'.",
          "type": [
            "boolean",
            "null"
          ]
        },
        "date_of_beatification": {
          "description": "Date of Beatification, as a Number (year), a String (in 'YYYY-MM' or 'YYYY-MM-DD' format),\nor an object describing date range, multiple possible date, or a century.",
          "anyOf": [
            {
              "$ref": "#/definitions/SaintDateDef"
            },
            {
              "type": "null"
            }
          ]
        },
        "date_of_beatification_is_approximative": {
          "description": "Specify whether an approximate indicator should be added, when the date is displayed.\nFor example in English: 'c. 201'.",
          "type": [
            "boolean",
            "null"
          ]
        },
        "hide_canonization_level": {
          "description": "Specify if the canonization level should not be displayed.\nIt's generally the case when the canonization are already included in the name.",
          "type": [
            "boolean",
            "null"
          ]
        },
        "titles": {
          "description": "Titles of the Saint or the Blessed",
          "type": [
            "array",
            "null"
          ],
          "items": {
            "$ref": "#/definitions/Title"
          }
        },
        "sex": {
          "description": "Determine if the Saint or the Blessed is a male or a female.",
          "anyOf": [
            {
              "$ref": "#/definitions/Sex"
            },
            {
              "type": "null"
            }
          ]
        },
        "hide_titles": {
          "description": "Specify if the titles should not be displayed.\nIt's generally the case when titles are already included in the name.",
          "type": [
            "boolean",
            "null"
          ]
        },
        "date_of_dedication": {
          "description": "Date of Dedication of a church, basilica, or cathedral (or other place of worship),\nas a Number (year), a String (in 'YYYY-MM' or 'YYYY-MM-DD' format),\nor an object describing date range, multiple possible date, or a century.",
          "anyOf": [
            {
              "$ref": "#/definitions/SaintDateDef"
            },
            {
              "type": "null"
            }
          ]
        },
        "date_of_birth": {
          "description": "Date of Birth, as a Number (year), a String (in 'YYYY-MM' or 'YYYY-MM-DD' format),\nor an object describing date range, multiple possible date, or a century.",
          "anyOf": [
            {
              "$ref": "#/definitions/SaintDateDef"
            },
            {
              "type": "null"
            }
          ]
        },
        "date_of_birth_is_approximative": {
          "description": "Specify whether an approximate indicator should be added, when the date is displayed.\nFor example in English: 'c. 201'.",
          "type": [
            "boolean",
            "null"
          ]
        },
        "date_of_death": {
          "description": "Date of Death, as a Number (year), a String (in 'YYYY-MM' or 'YYYY-MM-DD' format),\nor an object describing date range, multiple possible date, or a century.",
          "anyOf": [
            {
              "$ref": "#/definitions/SaintDateDef"
            },
            {
              "type": "null"
            }
          ]
        },
        "date_of_death_is_approximative": {
          "description": "Specify whether an approximate indicator should be added, when the date is displayed.\nFor example in English: 'c. 201'.",
          "type": [
            "boolean",
            "null"
          ]
        },
        "count": {
          "description": "Number of person that this definition represent.\nIt could be set as 'many' if the number is not defined.",
          "anyOf": [
            {
              "$ref": "#/definitions/SaintCount"
            },
            {
              "type": "null"
            }
          ]
        },
        "sources": {
          "description": "Sources for the information about this entity",
          "type": [
            "array",
            "null"
          ],
          "items": {
            "type": "string"
          }
        },
        "_todo": {
          "description": "Internal notes (not serialized).",
          "type": [
            "array",
            "null"
          ],
          "items": {
            "type": "string"
          },
          "writeOnly": true
        }
      },
      "additionalProperties": false
    },
    "EntityType": {
      "description": "The type of entity in the liturgical calendar.\nDefines whether the entity represents a person, place, or event.",
      "oneOf": [
        {
          "description": "A person (saint, blessed, or other individual)",
          "type": "string",
          "const": "PERSON"
        },
        {
          "description": "A place (shrine, city, or geographical location)",
          "type": "string",
          "const": "PLACE"
        },
        {
          "description": "An event (historical or liturgical occurrence)",
          "type": "string",
          "const": "EVENT"
        }
      ]
    },
    "CanonizationLevel": {
      "description": "Canonization level indicating the official recognition status of a person.\nDefines whether someone is beatified (Blessed) or canonized (Saint).",
      "oneOf": [
        {
          "description": "Beatified person (Blessed) - first step toward sainthood",
          "type": "string",
          "const": "BLESSED"
        },
        {
          "description": "Canonized person (Saint) - fully recognized as a saint",
          "type": "string",
          "const": "SAINT"
        }
      ]
    },
    "SaintDateDef": {
      "description": "Saint date definition supporting various date specifications.\nAllows single dates, date ranges, multiple alternatives, or century specifications.",
      "anyOf": [
        {
          "description": "Single date specification",
          "$ref": "#/definitions/SaintDate"
        },
        {
          "description": "Date range between two dates",
          "type": "object",
          "properties": {
            "between": {
              "description": "The date range (start and end dates)",
              "type": "array",
              "items": {
                "$ref": "#/definitions/SaintDate"
              },
              "minItems": 2,
              "maxItems": 2
            }
          },
          "required": [
            "between"
          ],
          "additionalProperties": false
        },
        {
          "description": "Multiple alternative dates (any one of them)",
          "type": "object",
          "properties": {
            "or": {
              "description": "The list of alternative dates",
              "type": "array",
              "items": {
                "$ref": "#/definitions/SaintDate"
              }
            }
          },
          "required": [
            "or"
          ],
          "additionalProperties": false
        },
        {
          "description": "Century specification (e.g., 12 for 12th century)",
          "type": "object",
          "properties": {
            "century": {
              "description": "The century number",
              "type": "integer",
              "format": "uint32",
              "minimum": 0
            }
          },
          "required": [
            "century"
          ],
          "additionalProperties": false
        }
      ]
    },
    "SaintDate": {
      "description": "Saint date representation with different precision levels.\nSupports year-only, year-month, or full date specifications.",
      "anyOf": [
        {
          "description": "Year only (e.g., 1234)",
          "type": "integer",
          "format": "uint32",
          "minimum": 0
        },
        {
          "description": "Year and month in \"YYYY-MM\" format (e.g., \"1234-05\")",
          "type": "string"
        },
        {
          "description": "Full date in \"YYYY-MM-DD\" format (e.g., \"1234-05-15\")",
          "type": "string"
        }
      ]
    },
    "Title": {
      "description": "Titles and patronages associated with saints and blessed.\nRepresents the various ecclesiastical titles and patronages that can be assigned to entities.",
      "type": "string",
      "enum": [
        "ABBESS",
        "ABBOT",
        "APOSTLE",
        "ARCHANGEL",
        "BISHOP",
        "DEACON",
        "DOCTOR_OF_THE_CHURCH",
        "EMPRESS",
        "EVANGELIST",
        "FIRST_BISHOP",
        "HERMIT",
        "KING",
        "MARTYR",
        "MISSIONARY",
        "MONK",
        "MOTHER_AND_QUEEN_OF_CHILE",
        "PARENTS_OF_THE_BLESSED_VIRGIN_MARY",
        "POPE",
        "PATRIARCH",
        "PILGRIM",
        "PRIEST",
        "PROPHET",
        "PROTO_MARTYR_OF_OCEANIA",
        "QUEEN",
        "QUEEN_OF_POLAND",
        "RELIGIOUS",
        "SLAVIC_MISSIONARY",
        "SPOUSE_OF_THE_BLESSED_VIRGIN_MARY",
        "THE_FIRST_MARTYR",
        "VIRGIN",
        "COPATRON_OF_EUROPE",
        "COPATRON_OF_IRELAND",
        "COPATRON_OF_CANADA",
        "COPATRONESS_OF_EUROPE",
        "COPATRONESS_OF_FRANCE",
        "COPATRONESS_OF_IRELAND",
        "COPATRONESS_OF_ITALY_AND_EUROPE",
        "COPATRONESS_OF_THE_PHILIPPINES",
        "PATRON_OF_CANADA",
        "PATRON_OF_ENGLAND",
        "PATRON_OF_EUROPE",
        "PATRON_OF_FRANCE",
        "PATRON_OF_IRELAND",
        "PATRON_OF_ITALY",
        "PATRON_OF_OCEANIA",
        "PATRON_OF_POLAND",
        "PATRON_OF_RUSSIA",
        "PATRON_OF_SCOTLAND",
        "PATRON_OF_SPAIN",
        "PATRON_OF_THE_CZECH_NATION",
        "PATRON_OF_THE_DIOCESE",
        "PATRON_OF_WALES",
        "PATRONESS_OF_ALSACE",
        "PATRONESS_OF_ARGENTINA",
        "PATRONESS_OF_BRAZIL",
        "PATRONESS_OF_HUNGARY",
        "PATRONESS_OF_PUERTO_RICO",
        "PATRONESS_OF_SLOVAKIA",
        "PATRONESS_OF_THE_AMERICAS",
        "PATRONESS_OF_THE_PHILIPPINES",
        "PATRONESS_OF_THE_PROVINCE_OF_QUEBEC",
        "PATRONESS_OF_THE_USA",
        "PATRON_OF_THE_CLERGY_OF_THE_ARCHDIOCESE_OF_LYON",
        "PATRON_OF_THE_CITY_OF_LYON",
        "PATRONESS_OF_COSTA_RICA",
        "PRINCIPAL_PATRON_OF_THE_DIOCESE",
        "SECOND_PATRON_OF_THE_DIOCESE"
      ]
    },
    "Sex": {
      "description": "Sex of a person.",
      "oneOf": [
        {
          "description": "Male person",
          "type": "string",
          "const": "MALE"
        },
        {
          "description": "Female person",
          "type": "string",
          "const": "FEMALE"
        }
      ]
    },
    "SaintCount": {
      "description": "Represents the number of saints for an entity or a group of entities.\n\nCan be either a specific number (u32) or \"MANY\" to indicate\nan indeterminate number of saints.\n\n# Serialization\n- `Number(n)` serializes as integer `n`\n- `Many` serializes as string `\"MANY\"`\n\n# Deserialization\n- Integers are converted to `Number(u32)`\n- String `\"MANY\"` is converted to `Many`\n- All other types generate an error",
      "anyOf": [
        {
          "type": "integer",
          "format": "uint32",
          "minimum": 0
        },
        {
          "type": "string",
          "const": "MANY"
        },
        {
          "type": "null"
        }
      ]
    }
  }
}"##;
