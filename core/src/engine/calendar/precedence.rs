use strum::IntoEnumIterator;

use super::Calendar;
use crate::engine::liturgical_day::LiturgicalDay;
use crate::types::liturgical::{Precedence, Rank, Season};

impl Calendar {
    /// Applies precedence rules according to GNLY #49
    ///
    /// This function sorts and filters liturgical days for a single date
    /// based on their precedence, handling special cases like:
    /// - Holy Thursday (two liturgical days on the same date)
    /// - Optional memorials
    /// - allowSimilarRankItems flag
    pub(super) fn apply_precedence_rules(&self, days: &mut [LiturgicalDay]) -> Vec<LiturgicalDay> {
        if days.is_empty() {
            return Vec::new();
        }

        if days.len() == 1 {
            return days.to_owned();
        }

        // Sort by precedence (lower enum variant = higher precedence)
        days.sort_by(|a, b| self.compare_precedence(a, b));

        // Get the highest precedence day
        let highest = &days[0];

        // Check for special cases
        // Holy Thursday: both "holy_thursday" and "thursday_of_the_lords_supper" appear
        let is_holy_thursday = days.iter().any(|d| d.id == "holy_thursday")
            && days.iter().any(|d| d.id == "thursday_of_the_lords_supper");

        if is_holy_thursday {
            // Return both days for Holy Thursday
            return days
                .iter()
                .filter(|d| d.id == "holy_thursday" || d.id == "thursday_of_the_lords_supper")
                .cloned()
                .collect();
        }

        // Detect weekday_13 and optional memorials
        let weekday_13 = days
            .iter()
            .find(|d| d.precedence == Precedence::Weekday_13)
            .cloned();

        let mut optional_memorials: Vec<LiturgicalDay> = days
            .iter()
            .filter(|d| d.precedence == Precedence::OptionalMemorial_12)
            .cloned()
            .collect();

        // Sort optional memorials by calendar priority (more general first)
        optional_memorials.sort_by_key(|d| {
            self.calendar_priority
                .get(&d.from_calendar_id)
                .copied()
                .unwrap_or(usize::MAX)
        });

        // Base result starts with the highest precedence day
        let mut result = vec![highest.clone()];

        // Handle allowSimilarRankItems
        if highest.allow_similar_rank_items {
            for day in days.iter().skip(1) {
                if day.rank == highest.rank && !result.iter().any(|d| d.id == day.id) {
                    result.push(day.clone());
                }
            }
        }

        // During Lent, obligatory memorials become optional (GNLY #14)
        if let Some(Season::Lent) = highest.season {
            for day in days.iter().skip(1) {
                if day.rank == Rank::Memorial && !result.iter().any(|d| d.id == day.id) {
                    let mut optional_day = day.clone();
                    optional_day.is_optional = true;
                    optional_day.rank = Rank::OptionalMemorial;
                    result.push(optional_day);
                }
            }
        }

        // Optional memorial handling with weekday inclusion
        let highest_allows_optional = self.can_have_optional_memorials(highest);
        let highest_is_optional = highest.precedence == Precedence::OptionalMemorial_12;

        if (highest_allows_optional || highest_is_optional) && !optional_memorials.is_empty() {
            let mut ordered: Vec<LiturgicalDay> = Vec::new();

            if let Some(weekday) = weekday_13.clone()
                && !ordered.iter().any(|d| d.id == weekday.id)
            {
                ordered.push(weekday);
            }

            if highest.precedence != Precedence::Weekday_13
                && highest.precedence != Precedence::OptionalMemorial_12
                && !ordered.iter().any(|d| d.id == highest.id)
            {
                ordered.push(highest.clone());
            }

            for day in optional_memorials {
                if !ordered.iter().any(|d| d.id == day.id) {
                    ordered.push(day);
                }
            }

            for day in result {
                if !ordered.iter().any(|d| d.id == day.id) {
                    ordered.push(day);
                }
            }

            return ordered;
        }

        result
    }

    /// Compares two LiturgicalDay objects by precedence
    pub(super) fn compare_precedence(
        &self,
        a: &LiturgicalDay,
        b: &LiturgicalDay,
    ) -> std::cmp::Ordering {
        // Get the position of each precedence in the enum order
        let precedences: Vec<Precedence> = Precedence::iter().collect();

        let pos_a = precedences
            .iter()
            .position(|p| *p == a.precedence)
            .unwrap_or(usize::MAX);
        let pos_b = precedences
            .iter()
            .position(|p| *p == b.precedence)
            .unwrap_or(usize::MAX);

        // Lower position = higher precedence
        let cmp = pos_a.cmp(&pos_b);

        if cmp == std::cmp::Ordering::Equal {
            // If same precedence, non-optional comes before optional
            match (a.is_optional, b.is_optional) {
                (false, true) => std::cmp::Ordering::Less,
                (true, false) => std::cmp::Ordering::Greater,
                _ => std::cmp::Ordering::Equal,
            }
        } else {
            cmp
        }
    }

    /// Determines if a day can have optional memorials added to it
    ///
    /// According to GNLY #14 and GIRM #355:
    /// - On privileged weekdays (GNLY #59 9)
    /// - On ferias (GNLY #59 13)
    fn can_have_optional_memorials(&self, day: &LiturgicalDay) -> bool {
        matches!(
            day.precedence,
            Precedence::PrivilegedWeekday_9 | Precedence::Weekday_13
        )
    }
}
