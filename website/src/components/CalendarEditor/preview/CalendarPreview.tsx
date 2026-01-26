import type { ReactNode } from 'react';
import { useMemo } from 'react';
import { useEditorStore } from '../context/useEditorStore';
import {
  format,
  getYear,
  startOfYear,
  endOfYear,
  eachMonthOfInterval,
  eachDayOfInterval,
  startOfMonth,
  endOfMonth,
} from 'date-fns';
import type { DayDefinition, DateDef, Precedence } from '../types';
import styles from '../shared/shared.module.css';

// Simplified precedence to rank for display
const RANK_DISPLAY: Record<string, string> = {
  TRIDUUM_1: 'Triduum',
  PROPER_OF_TIME_SOLEMNITY_2: 'Solemnity',
  PRIVILEGED_SUNDAY_2: 'Sunday',
  ASH_WEDNESDAY_2: 'Ash Wed',
  WEEKDAY_OF_HOLY_WEEK_2: 'Holy Week',
  WEEKDAY_OF_EASTER_OCTAVE_2: 'Octave',
  GENERAL_SOLEMNITY_3: 'Solemnity',
  COMMEMORATION_OF_ALL_THE_FAITHFUL_DEPARTED_3: 'Commem.',
  PROPER_SOLEMNITY__PRINCIPAL_PATRON_4A: 'Solemnity',
  PROPER_SOLEMNITY__DEDICATION_OF_THE_OWN_CHURCH_4B: 'Solemnity',
  PROPER_SOLEMNITY__TITLE_OF_THE_OWN_CHURCH_4C: 'Solemnity',
  PROPER_SOLEMNITY__TITLE_OR_FOUNDER_OR_PRIMARY_PATRON_OF_A_RELIGIOUS_ORG_4D: 'Solemnity',
  GENERAL_LORD_FEAST_5: 'Feast',
  UNPRIVILEGED_SUNDAY_6: 'Sunday',
  GENERAL_FEAST_7: 'Feast',
  PROPER_FEAST__PRINCIPAL_PATRON_OF_A_DIOCESE_8A: 'Feast',
  PROPER_FEAST__DEDICATION_OF_THE_CATHEDRAL_CHURCH_8B: 'Feast',
  PROPER_FEAST__PRINCIPAL_PATRON_OF_A_REGION_8C: 'Feast',
  PROPER_FEAST__TITLE_OR_FOUNDER_OR_PRIMARY_PATRON_OF_A_RELIGIOUS_ORG_8D: 'Feast',
  PROPER_FEAST__TO_AN_INDIVIDUAL_CHURCH_8E: 'Feast',
  PROPER_FEAST_8F: 'Feast',
  PRIVILEGED_WEEKDAY_9: 'Weekday',
  GENERAL_MEMORIAL_10: 'Memorial',
  PROPER_MEMORIAL__SECOND_PATRON_11A: 'Memorial',
  PROPER_MEMORIAL_11B: 'Memorial',
  OPTIONAL_MEMORIAL_12: 'Opt. Mem.',
  WEEKDAY_13: 'Weekday',
};

// Color based on rank
const RANK_COLORS: Record<string, string> = {
  Triduum: '#dc2626',
  Solemnity: '#c026d3',
  Sunday: '#2563eb',
  'Ash Wed': '#7c3aed',
  'Holy Week': '#7c3aed',
  Octave: '#c026d3',
  'Commem.': '#7c3aed',
  Feast: '#059669',
  Memorial: '#0891b2',
  'Opt. Mem.': '#6b7280',
  Weekday: '#9ca3af',
};

interface PreviewDay {
  id: string;
  date: Date;
  name: string;
  rank: string;
  isLocal: boolean;
}

export default function CalendarPreview(): ReactNode {
  const { calendar, previewYear, setPreviewYear } = useEditorStore();

  // Calculate preview days from day definitions
  const previewDays = useMemo(() => {
    const days: PreviewDay[] = [];
    const year = previewYear;

    for (const [id, dayDef] of Object.entries(calendar.days_definitions)) {
      if (dayDef.drop) continue;

      const date = calculateDate(dayDef.date_def, year);
      if (!date) continue;

      days.push({
        id,
        date,
        name: formatDayName(id),
        rank: dayDef.precedence ? RANK_DISPLAY[dayDef.precedence] || 'Unknown' : 'Unknown',
        isLocal: true, // All days in this calendar are "local" (defined here)
      });
    }

    return days.sort((a, b) => a.date.getTime() - b.date.getTime());
  }, [calendar.days_definitions, previewYear]);

  // Group by month
  const daysByMonth = useMemo(() => {
    const groups: Map<number, PreviewDay[]> = new Map();

    for (const day of previewDays) {
      const month = day.date.getMonth();
      if (!groups.has(month)) {
        groups.set(month, []);
      }
      groups.get(month)!.push(day);
    }

    return groups;
  }, [previewDays]);

  const months = [
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

  if (previewDays.length === 0) {
    return (
      <div style={{ textAlign: 'center', color: 'var(--ifm-color-content-secondary)' }}>
        <p>No days defined yet. Add day definitions to see the preview.</p>
      </div>
    );
  }

  return (
    <div>
      <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', marginBottom: '1rem' }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
          <label style={{ fontSize: '0.875rem' }}>Year:</label>
          <select
            value={previewYear}
            onChange={(e) => setPreviewYear(parseInt(e.target.value))}
            style={{
              padding: '0.25rem 0.5rem',
              border: '1px solid var(--ifm-toc-border-color)',
              borderRadius: '4px',
              background: 'var(--ifm-background-color)',
            }}
          >
            {Array.from({ length: 10 }, (_, i) => new Date().getFullYear() - 2 + i).map((y) => (
              <option key={y} value={y}>
                {y}
              </option>
            ))}
          </select>
        </div>
        <div style={{ fontSize: '0.75rem', color: 'var(--ifm-color-content-secondary)' }}>
          {previewDays.length} days
        </div>
      </div>

      <div style={{ maxHeight: '250px', overflowY: 'auto' }}>
        {Array.from(daysByMonth.entries()).map(([month, days]) => (
          <div key={month} style={{ marginBottom: '1rem' }}>
            <h4
              style={{
                fontSize: '0.75rem',
                fontWeight: 600,
                color: 'var(--ifm-color-content-secondary)',
                marginBottom: '0.5rem',
              }}
            >
              {months[month].toUpperCase()}
            </h4>
            {days.map((day) => (
              <div
                key={day.id}
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: '0.75rem',
                  padding: '0.375rem 0',
                  borderBottom: '1px solid var(--ifm-toc-border-color)',
                  fontSize: '0.875rem',
                }}
              >
                <span style={{ width: '45px', fontWeight: 500 }}>
                  {format(day.date, 'd')} {format(day.date, 'EEE').substring(0, 3).toLowerCase()}
                </span>
                <span
                  style={{
                    width: '8px',
                    height: '8px',
                    borderRadius: '50%',
                    background: RANK_COLORS[day.rank] || '#9ca3af',
                  }}
                />
                <span style={{ flex: 1 }}>{day.name}</span>
                <span
                  style={{
                    fontSize: '0.6875rem',
                    padding: '0.125rem 0.375rem',
                    background: 'var(--ifm-hover-overlay)',
                    borderRadius: '4px',
                    color: RANK_COLORS[day.rank] || 'inherit',
                  }}
                >
                  {day.rank}
                </span>
                {day.isLocal && (
                  <span title="Defined in this calendar" style={{ fontSize: '0.75rem' }}>
                    ✨
                  </span>
                )}
              </div>
            ))}
          </div>
        ))}
      </div>

      <div style={{ marginTop: '0.75rem', fontSize: '0.75rem', color: 'var(--ifm-color-content-secondary)' }}>
        ✨ = Defined in this calendar
      </div>
    </div>
  );
}

function calculateDate(dateDef: DateDef | null | undefined, year: number): Date | null {
  if (!dateDef || Object.keys(dateDef).length === 0) return null;

  // Fixed date
  if ('month' in dateDef && 'date' in dateDef && !('day_of_week' in dateDef)) {
    const date = new Date(year, dateDef.month - 1, dateDef.date);
    if (dateDef.day_offset) {
      date.setDate(date.getDate() + dateDef.day_offset);
    }
    return date;
  }

  // Date function - simplified Easter calculation
  if ('date_fn' in dateDef) {
    let baseDate = calculateEaster(year);

    switch (dateDef.date_fn) {
      case 'PALM_SUNDAY':
        baseDate = new Date(baseDate);
        baseDate.setDate(baseDate.getDate() - 7);
        break;
      case 'PENTECOST_SUNDAY':
        baseDate = new Date(baseDate);
        baseDate.setDate(baseDate.getDate() + 49);
        break;
      case 'DIVINE_MERCY_SUNDAY':
        baseDate = new Date(baseDate);
        baseDate.setDate(baseDate.getDate() + 7);
        break;
      case 'MARY_MOTHER_OF_THE_CHURCH':
        baseDate = new Date(baseDate);
        baseDate.setDate(baseDate.getDate() + 50);
        break;
      case 'CORPUS_CHRISTI_SUNDAY':
        baseDate = new Date(baseDate);
        baseDate.setDate(baseDate.getDate() + 60);
        break;
      case 'IMMACULATE_HEART_OF_MARY':
        baseDate = new Date(baseDate);
        baseDate.setDate(baseDate.getDate() + 68);
        break;
      // Fixed dates for other functions
      case 'EPIPHANY_SUNDAY':
        return new Date(year, 0, 6); // Simplified
      case 'PRESENTATION_OF_THE_LORD':
        return new Date(year, 1, 2);
      case 'ANNUNCIATION':
        return new Date(year, 2, 25);
      case 'NATIVITY_OF_JOHN_THE_BAPTIST':
        return new Date(year, 5, 24);
      case 'PETER_AND_PAUL_APOSTLES':
        return new Date(year, 5, 29);
      case 'TRANSFIGURATION':
        return new Date(year, 7, 6);
      case 'ASSUMPTION':
        return new Date(year, 7, 15);
      case 'EXALTATION_OF_THE_HOLY_CROSS':
        return new Date(year, 8, 14);
      case 'ALL_SAINTS':
        return new Date(year, 10, 1);
      case 'IMMACULATE_CONCEPTION_OF_MARY':
        return new Date(year, 11, 8);
      default:
        break;
    }

    if (dateDef.day_offset) {
      baseDate.setDate(baseDate.getDate() + dateDef.day_offset);
    }
    return baseDate;
  }

  // Nth weekday of month
  if ('nth_week_in_month' in dateDef) {
    const firstDay = new Date(year, dateDef.month - 1, 1);
    const firstWeekday = firstDay.getDay();
    let targetDay = dateDef.day_of_week - firstWeekday;
    if (targetDay < 0) targetDay += 7;
    targetDay += (dateDef.nth_week_in_month - 1) * 7 + 1;
    const date = new Date(year, dateDef.month - 1, targetDay);
    if (dateDef.day_offset) {
      date.setDate(date.getDate() + dateDef.day_offset);
    }
    return date;
  }

  // Last weekday of month
  if ('last_day_of_week_in_month' in dateDef) {
    const lastDay = new Date(year, dateDef.month, 0);
    let diff = lastDay.getDay() - dateDef.last_day_of_week_in_month;
    if (diff < 0) diff += 7;
    const date = new Date(year, dateDef.month - 1, lastDay.getDate() - diff);
    if (dateDef.day_offset) {
      date.setDate(date.getDate() + dateDef.day_offset);
    }
    return date;
  }

  return null;
}

// Simplified Easter calculation (Anonymous Gregorian algorithm)
function calculateEaster(year: number): Date {
  const a = year % 19;
  const b = Math.floor(year / 100);
  const c = year % 100;
  const d = Math.floor(b / 4);
  const e = b % 4;
  const f = Math.floor((b + 8) / 25);
  const g = Math.floor((b - f + 1) / 3);
  const h = (19 * a + b - d - g + 15) % 30;
  const i = Math.floor(c / 4);
  const k = c % 4;
  const l = (32 + 2 * e + 2 * i - h - k) % 7;
  const m = Math.floor((a + 11 * h + 22 * l) / 451);
  const month = Math.floor((h + l - 7 * m + 114) / 31);
  const day = ((h + l - 7 * m + 114) % 31) + 1;
  return new Date(year, month - 1, day);
}

function formatDayName(id: string): string {
  return id.replace(/_/g, ' ').replace(/\b\w/g, (c) => c.toUpperCase());
}
