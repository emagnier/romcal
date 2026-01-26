import type { ReactNode } from 'react';
import { useState, useMemo } from 'react';
import { useEditorStore } from '../context/useEditorStore';
import type { Precedence, DayDefinition } from '../types';
import styles from '../shared/shared.module.css';
import clsx from 'clsx';

// Precedence display names
const PRECEDENCE_LABELS: Record<Precedence, string> = {
  TRIDUUM_1: 'Triduum',
  PROPER_OF_TIME_SOLEMNITY_2: 'Solemnity (Proper)',
  PRIVILEGED_SUNDAY_2: 'Privileged Sunday',
  ASH_WEDNESDAY_2: 'Ash Wednesday',
  WEEKDAY_OF_HOLY_WEEK_2: 'Holy Week',
  WEEKDAY_OF_EASTER_OCTAVE_2: 'Easter Octave',
  GENERAL_SOLEMNITY_3: 'Solemnity (General)',
  COMMEMORATION_OF_ALL_THE_FAITHFUL_DEPARTED_3: 'All Souls',
  PROPER_SOLEMNITY__PRINCIPAL_PATRON_4A: 'Solemnity (Patron)',
  PROPER_SOLEMNITY__DEDICATION_OF_THE_OWN_CHURCH_4B: 'Solemnity (Dedication)',
  PROPER_SOLEMNITY__TITLE_OF_THE_OWN_CHURCH_4C: 'Solemnity (Title)',
  PROPER_SOLEMNITY__TITLE_OR_FOUNDER_OR_PRIMARY_PATRON_OF_A_RELIGIOUS_ORG_4D: 'Solemnity (Religious)',
  GENERAL_LORD_FEAST_5: "Feast (Lord's)",
  UNPRIVILEGED_SUNDAY_6: 'Sunday',
  GENERAL_FEAST_7: 'Feast (General)',
  PROPER_FEAST__PRINCIPAL_PATRON_OF_A_DIOCESE_8A: 'Feast (Diocese)',
  PROPER_FEAST__DEDICATION_OF_THE_CATHEDRAL_CHURCH_8B: 'Feast (Cathedral)',
  PROPER_FEAST__PRINCIPAL_PATRON_OF_A_REGION_8C: 'Feast (Region)',
  PROPER_FEAST__TITLE_OR_FOUNDER_OR_PRIMARY_PATRON_OF_A_RELIGIOUS_ORG_8D: 'Feast (Religious)',
  PROPER_FEAST__TO_AN_INDIVIDUAL_CHURCH_8E: 'Feast (Church)',
  PROPER_FEAST_8F: 'Feast (Proper)',
  PRIVILEGED_WEEKDAY_9: 'Privileged Weekday',
  GENERAL_MEMORIAL_10: 'Memorial (General)',
  PROPER_MEMORIAL__SECOND_PATRON_11A: 'Memorial (2nd Patron)',
  PROPER_MEMORIAL_11B: 'Memorial (Proper)',
  OPTIONAL_MEMORIAL_12: 'Optional Memorial',
  WEEKDAY_13: 'Weekday',
};

// Month names
const MONTHS = [
  'January',
  'February',
  'March',
  'April',
  'May',
  'June',
  'July',
  'August',
  'September',
  'October',
  'November',
  'December',
];

export default function DayDefinitionsPanel(): ReactNode {
  const { calendar, addDayDefinition, removeDayDefinition, setActivePanel } = useEditorStore();

  const [searchQuery, setSearchQuery] = useState('');
  const [showNewDayForm, setShowNewDayForm] = useState(false);
  const [newDayId, setNewDayId] = useState('');

  const dayEntries = useMemo(() => {
    return Object.entries(calendar.days_definitions).sort((a, b) => {
      // Sort by date if available, then by ID
      const dateA = getDateSortKey(a[1]);
      const dateB = getDateSortKey(b[1]);
      if (dateA !== dateB) return dateA - dateB;
      return a[0].localeCompare(b[0]);
    });
  }, [calendar.days_definitions]);

  const filteredDays = useMemo(() => {
    if (!searchQuery) return dayEntries;
    const query = searchQuery.toLowerCase();
    return dayEntries.filter(
      ([id, day]) => id.toLowerCase().includes(query) || day.custom_locale_id?.toLowerCase().includes(query)
    );
  }, [dayEntries, searchQuery]);

  const handleCreateDay = () => {
    if (newDayId && !calendar.days_definitions[newDayId]) {
      addDayDefinition(newDayId, {
        precedence: 'OPTIONAL_MEMORIAL_12',
      });
      setActivePanel({ type: 'day', id: newDayId });
      setShowNewDayForm(false);
      setNewDayId('');
    }
  };

  const handleEditDay = (id: string) => {
    setActivePanel({ type: 'day', id });
  };

  const handleDeleteDay = (id: string) => {
    if (confirm(`Are you sure you want to delete "${id}"?`)) {
      removeDayDefinition(id);
    }
  };

  return (
    <div>
      <div className={styles.sectionHeader}>
        <h2 className={styles.sectionTitle}>Day Definitions</h2>
        <button
          type="button"
          onClick={() => setShowNewDayForm(true)}
          style={{
            padding: '0.5rem 1rem',
            fontSize: '0.875rem',
            background: 'var(--ifm-color-primary)',
            color: 'white',
            border: 'none',
            borderRadius: 'var(--ifm-button-border-radius)',
            cursor: 'pointer',
          }}
        >
          + Add Day
        </button>
      </div>

      {showNewDayForm && (
        <div className={styles.card} style={{ marginBottom: '1rem' }}>
          <h3 className={styles.cardTitle}>New Day Definition</h3>
          <div className={styles.inlineForm}>
            <div style={{ flex: 1 }}>
              <input
                type="text"
                className={styles.input}
                placeholder="Day ID (e.g., saint_john_apostle)"
                value={newDayId}
                onChange={(e) => setNewDayId(e.target.value.toLowerCase().replace(/[^a-z0-9_]/g, '_'))}
                onKeyDown={(e) => e.key === 'Enter' && handleCreateDay()}
              />
            </div>
            <div className={styles.buttonGroup}>
              <button
                type="button"
                onClick={handleCreateDay}
                disabled={!newDayId || !!calendar.days_definitions[newDayId]}
                style={{
                  padding: '0.5rem 1rem',
                  fontSize: '0.875rem',
                  background: 'var(--ifm-color-primary)',
                  color: 'white',
                  border: 'none',
                  borderRadius: 'var(--ifm-button-border-radius)',
                  cursor: 'pointer',
                  opacity: !newDayId || !!calendar.days_definitions[newDayId] ? 0.5 : 1,
                }}
              >
                Create
              </button>
              <button
                type="button"
                onClick={() => {
                  setShowNewDayForm(false);
                  setNewDayId('');
                }}
                style={{
                  padding: '0.5rem 1rem',
                  fontSize: '0.875rem',
                  background: 'transparent',
                  border: '1px solid var(--ifm-toc-border-color)',
                  borderRadius: 'var(--ifm-button-border-radius)',
                  cursor: 'pointer',
                }}
              >
                Cancel
              </button>
            </div>
          </div>
          {newDayId && calendar.days_definitions[newDayId] && (
            <p className={styles.error}>A day with this ID already exists</p>
          )}
        </div>
      )}

      <div className={styles.searchBox}>
        <span className={styles.searchIcon}>🔍</span>
        <input
          type="text"
          className={styles.searchInput}
          placeholder="Filter days..."
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
        />
      </div>

      {filteredDays.length === 0 ? (
        <div className={styles.emptyState}>
          <div className={styles.emptyStateIcon}>📅</div>
          <p className={styles.emptyStateText}>{searchQuery ? 'No matching days found' : 'No day definitions yet'}</p>
          {!searchQuery && (
            <button
              type="button"
              onClick={() => setShowNewDayForm(true)}
              style={{
                padding: '0.5rem 1rem',
                fontSize: '0.875rem',
                background: 'var(--ifm-color-primary)',
                color: 'white',
                border: 'none',
                borderRadius: 'var(--ifm-button-border-radius)',
                cursor: 'pointer',
              }}
            >
              Add your first day
            </button>
          )}
        </div>
      ) : (
        <table className={styles.table}>
          <thead>
            <tr>
              <th>Date</th>
              <th>ID / Name</th>
              <th>Rank</th>
              <th style={{ width: '100px' }}>Actions</th>
            </tr>
          </thead>
          <tbody>
            {filteredDays.map(([id, day]) => (
              <tr key={id}>
                <td>{formatDate(day)}</td>
                <td>
                  <strong>{id}</strong>
                  {day.custom_locale_id && (
                    <div style={{ fontSize: '0.8125rem', color: 'var(--ifm-color-content-secondary)' }}>
                      {day.custom_locale_id}
                    </div>
                  )}
                </td>
                <td>
                  <span
                    style={{
                      padding: '0.125rem 0.5rem',
                      fontSize: '0.75rem',
                      background: 'var(--ifm-hover-overlay)',
                      borderRadius: '4px',
                    }}
                  >
                    {day.precedence ? PRECEDENCE_LABELS[day.precedence] || day.precedence : '—'}
                  </span>
                </td>
                <td>
                  <div className={styles.tableActions}>
                    <button
                      type="button"
                      onClick={() => handleEditDay(id)}
                      title="Edit"
                      style={{
                        border: 'none',
                        background: 'none',
                        cursor: 'pointer',
                        fontSize: '1rem',
                      }}
                    >
                      ✏️
                    </button>
                    <button
                      type="button"
                      onClick={() => handleDeleteDay(id)}
                      title="Delete"
                      style={{
                        border: 'none',
                        background: 'none',
                        cursor: 'pointer',
                        fontSize: '1rem',
                      }}
                    >
                      🗑️
                    </button>
                  </div>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      )}

      <div style={{ marginTop: '1rem', fontSize: '0.875rem', color: 'var(--ifm-color-content-secondary)' }}>
        {filteredDays.length} day{filteredDays.length !== 1 ? 's' : ''} defined
      </div>
    </div>
  );
}

function getDateSortKey(day: DayDefinition): number {
  if (!day.date_def) return 9999;

  if ('month' in day.date_def && 'date' in day.date_def) {
    return day.date_def.month * 100 + day.date_def.date;
  }

  if ('date_fn' in day.date_def) {
    // Approximate sort keys for date functions
    const fnOrder: Record<string, number> = {
      EPIPHANY_SUNDAY: 107,
      PRESENTATION_OF_THE_LORD: 202,
      ANNUNCIATION: 325,
      PALM_SUNDAY: 400,
      EASTER_SUNDAY: 415,
      DIVINE_MERCY_SUNDAY: 422,
      PENTECOST_SUNDAY: 600,
      MARY_MOTHER_OF_THE_CHURCH: 601,
      CORPUS_CHRISTI_SUNDAY: 608,
      IMMACULATE_HEART_OF_MARY: 615,
      NATIVITY_OF_JOHN_THE_BAPTIST: 624,
      PETER_AND_PAUL_APOSTLES: 629,
      TRANSFIGURATION: 806,
      ASSUMPTION: 815,
      EXALTATION_OF_THE_HOLY_CROSS: 914,
      ALL_SAINTS: 1101,
      IMMACULATE_CONCEPTION_OF_MARY: 1208,
    };
    return fnOrder[day.date_def.date_fn] || 500;
  }

  return 9999;
}

function formatDate(day: DayDefinition): string {
  if (!day.date_def) return '—';

  if ('month' in day.date_def && 'date' in day.date_def) {
    const month = MONTHS[day.date_def.month - 1] || '?';
    return `${day.date_def.date} ${month.substring(0, 3)}`;
  }

  if ('date_fn' in day.date_def) {
    const offset = day.date_def.day_offset || 0;
    const fnName = day.date_def.date_fn.replace(/_/g, ' ').toLowerCase();
    if (offset === 0) return fnName;
    return `${fnName} ${offset > 0 ? '+' : ''}${offset}`;
  }

  if ('nth_week_in_month' in day.date_def && 'day_of_week' in day.date_def) {
    const weekdays = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
    const month = MONTHS[day.date_def.month - 1] || '?';
    const ordinals = ['', '1st', '2nd', '3rd', '4th', '5th'];
    return `${ordinals[day.date_def.nth_week_in_month]} ${weekdays[day.date_def.day_of_week]} of ${month.substring(0, 3)}`;
  }

  if ('last_day_of_week_in_month' in day.date_def) {
    const weekdays = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
    const month = MONTHS[day.date_def.month - 1] || '?';
    return `Last ${weekdays[day.date_def.last_day_of_week_in_month]} of ${month.substring(0, 3)}`;
  }

  return 'Inherited';
}
