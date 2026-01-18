import type { ReactNode } from 'react';
import { useState } from 'react';
import { useEditorStore } from '../context/useEditorStore';
import { TextInput, Select, Checkbox, RadioGroup } from '../shared/FormField';
import { MultiSelect } from '../shared/MultiSelect';
import type { DateDef, DateFn, Precedence, CommonDefinition, Title, DateDefType } from '../types';
import styles from '../shared/shared.module.css';

// All precedence options
const PRECEDENCE_OPTIONS: { value: Precedence; label: string }[] = [
  { value: 'TRIDUUM_1', label: '1 - Triduum' },
  { value: 'PROPER_OF_TIME_SOLEMNITY_2', label: '2 - Proper Solemnity (Nativity, etc.)' },
  { value: 'PRIVILEGED_SUNDAY_2', label: '2 - Privileged Sunday' },
  { value: 'ASH_WEDNESDAY_2', label: '2 - Ash Wednesday' },
  { value: 'WEEKDAY_OF_HOLY_WEEK_2', label: '2 - Holy Week Weekday' },
  { value: 'WEEKDAY_OF_EASTER_OCTAVE_2', label: '2 - Easter Octave Day' },
  { value: 'GENERAL_SOLEMNITY_3', label: '3 - General Solemnity' },
  { value: 'COMMEMORATION_OF_ALL_THE_FAITHFUL_DEPARTED_3', label: '3 - All Souls' },
  { value: 'PROPER_SOLEMNITY__PRINCIPAL_PATRON_4A', label: '4a - Proper Solemnity (Principal Patron)' },
  { value: 'PROPER_SOLEMNITY__DEDICATION_OF_THE_OWN_CHURCH_4B', label: '4b - Proper Solemnity (Dedication)' },
  { value: 'PROPER_SOLEMNITY__TITLE_OF_THE_OWN_CHURCH_4C', label: '4c - Proper Solemnity (Title)' },
  { value: 'PROPER_SOLEMNITY__TITLE_OR_FOUNDER_OR_PRIMARY_PATRON_OF_A_RELIGIOUS_ORG_4D', label: '4d - Proper Solemnity (Religious Org)' },
  { value: 'GENERAL_LORD_FEAST_5', label: "5 - General Lord's Feast" },
  { value: 'UNPRIVILEGED_SUNDAY_6', label: '6 - Unprivileged Sunday' },
  { value: 'GENERAL_FEAST_7', label: '7 - General Feast' },
  { value: 'PROPER_FEAST__PRINCIPAL_PATRON_OF_A_DIOCESE_8A', label: '8a - Proper Feast (Diocese Patron)' },
  { value: 'PROPER_FEAST__DEDICATION_OF_THE_CATHEDRAL_CHURCH_8B', label: '8b - Proper Feast (Cathedral Dedication)' },
  { value: 'PROPER_FEAST__PRINCIPAL_PATRON_OF_A_REGION_8C', label: '8c - Proper Feast (Region Patron)' },
  { value: 'PROPER_FEAST__TITLE_OR_FOUNDER_OR_PRIMARY_PATRON_OF_A_RELIGIOUS_ORG_8D', label: '8d - Proper Feast (Religious Org)' },
  { value: 'PROPER_FEAST__TO_AN_INDIVIDUAL_CHURCH_8E', label: '8e - Proper Feast (Individual Church)' },
  { value: 'PROPER_FEAST_8F', label: '8f - Proper Feast (Other)' },
  { value: 'PRIVILEGED_WEEKDAY_9', label: '9 - Privileged Weekday' },
  { value: 'GENERAL_MEMORIAL_10', label: '10 - General Memorial' },
  { value: 'PROPER_MEMORIAL__SECOND_PATRON_11A', label: '11a - Proper Memorial (Second Patron)' },
  { value: 'PROPER_MEMORIAL_11B', label: '11b - Proper Memorial' },
  { value: 'OPTIONAL_MEMORIAL_12', label: '12 - Optional Memorial' },
  { value: 'WEEKDAY_13', label: '13 - Weekday' },
];

// Date function options
const DATE_FN_OPTIONS: { value: DateFn; label: string }[] = [
  { value: 'EASTER_SUNDAY', label: 'Easter Sunday' },
  { value: 'PALM_SUNDAY', label: 'Palm Sunday' },
  { value: 'PENTECOST_SUNDAY', label: 'Pentecost Sunday' },
  { value: 'DIVINE_MERCY_SUNDAY', label: 'Divine Mercy Sunday' },
  { value: 'EPIPHANY_SUNDAY', label: 'Epiphany Sunday' },
  { value: 'CORPUS_CHRISTI_SUNDAY', label: 'Corpus Christi Sunday' },
  { value: 'MARY_MOTHER_OF_THE_CHURCH', label: 'Mary Mother of the Church' },
  { value: 'IMMACULATE_HEART_OF_MARY', label: 'Immaculate Heart of Mary' },
  { value: 'PRESENTATION_OF_THE_LORD', label: 'Presentation of the Lord' },
  { value: 'ANNUNCIATION', label: 'Annunciation' },
  { value: 'NATIVITY_OF_JOHN_THE_BAPTIST', label: 'Nativity of John the Baptist' },
  { value: 'PETER_AND_PAUL_APOSTLES', label: 'Peter and Paul Apostles' },
  { value: 'TRANSFIGURATION', label: 'Transfiguration' },
  { value: 'ASSUMPTION', label: 'Assumption' },
  { value: 'EXALTATION_OF_THE_HOLY_CROSS', label: 'Exaltation of the Holy Cross' },
  { value: 'ALL_SAINTS', label: 'All Saints' },
  { value: 'IMMACULATE_CONCEPTION_OF_MARY', label: 'Immaculate Conception of Mary' },
];

// Common definition options
const COMMON_OPTIONS: { value: CommonDefinition; label: string }[] = [
  { value: 'NONE', label: 'None' },
  { value: 'BLESSED_VIRGIN_MARY', label: 'Blessed Virgin Mary' },
  { value: 'MARTYRS', label: 'Martyrs' },
  { value: 'MISSIONARY_MARTYRS', label: 'Missionary Martyrs' },
  { value: 'VIRGIN_MARTYRS', label: 'Virgin Martyrs' },
  { value: 'WOMAN_MARTYRS', label: 'Woman Martyrs' },
  { value: 'PASTORS', label: 'Pastors' },
  { value: 'POPES', label: 'Popes' },
  { value: 'BISHOPS', label: 'Bishops' },
  { value: 'FOUNDERS', label: 'Founders' },
  { value: 'MISSIONARIES', label: 'Missionaries' },
  { value: 'DOCTORS_OF_THE_CHURCH', label: 'Doctors of the Church' },
  { value: 'VIRGINS', label: 'Virgins' },
  { value: 'SAINTS', label: 'Saints' },
  { value: 'ABBOTS', label: 'Abbots' },
  { value: 'MONKS', label: 'Monks' },
  { value: 'NUNS', label: 'Nuns' },
  { value: 'RELIGIOUS', label: 'Religious' },
  { value: 'MERCY_WORKERS', label: 'Works of Mercy' },
  { value: 'EDUCATORS', label: 'Educators' },
  { value: 'HOLY_WOMEN', label: 'Holy Women' },
  { value: 'DEDICATION_ANNIVERSARY__INSIDE', label: 'Dedication (Inside)' },
  { value: 'DEDICATION_ANNIVERSARY__OUTSIDE', label: 'Dedication (Outside)' },
];

// Title options (simplified list of common titles)
const TITLE_OPTIONS: { value: Title; label: string }[] = [
  { value: 'MARTYR', label: 'Martyr' },
  { value: 'VIRGIN', label: 'Virgin' },
  { value: 'BISHOP', label: 'Bishop' },
  { value: 'PRIEST', label: 'Priest' },
  { value: 'DEACON', label: 'Deacon' },
  { value: 'POPE', label: 'Pope' },
  { value: 'APOSTLE', label: 'Apostle' },
  { value: 'EVANGELIST', label: 'Evangelist' },
  { value: 'DOCTOR_OF_THE_CHURCH', label: 'Doctor of the Church' },
  { value: 'ABBOT', label: 'Abbot' },
  { value: 'ABBESS', label: 'Abbess' },
  { value: 'MONK', label: 'Monk' },
  { value: 'RELIGIOUS', label: 'Religious' },
  { value: 'HERMIT', label: 'Hermit' },
  { value: 'KING', label: 'King' },
  { value: 'QUEEN', label: 'Queen' },
  { value: 'MISSIONARY', label: 'Missionary' },
  { value: 'PROPHET', label: 'Prophet' },
  { value: 'ARCHANGEL', label: 'Archangel' },
];

// Month options
const MONTH_OPTIONS = [
  { value: '1', label: 'January' },
  { value: '2', label: 'February' },
  { value: '3', label: 'March' },
  { value: '4', label: 'April' },
  { value: '5', label: 'May' },
  { value: '6', label: 'June' },
  { value: '7', label: 'July' },
  { value: '8', label: 'August' },
  { value: '9', label: 'September' },
  { value: '10', label: 'October' },
  { value: '11', label: 'November' },
  { value: '12', label: 'December' },
];

// Weekday options
const WEEKDAY_OPTIONS = [
  { value: '0', label: 'Sunday' },
  { value: '1', label: 'Monday' },
  { value: '2', label: 'Tuesday' },
  { value: '3', label: 'Wednesday' },
  { value: '4', label: 'Thursday' },
  { value: '5', label: 'Friday' },
  { value: '6', label: 'Saturday' },
];

interface DayEditorPanelProps {
  dayId: string;
}

export default function DayEditorPanel({ dayId }: DayEditorPanelProps): ReactNode {
  const {
    calendar,
    updateDayDefinition,
    renameDayDefinition,
    setActivePanel,
  } = useEditorStore();

  const day = calendar.days_definitions[dayId];
  const [editingId, setEditingId] = useState(false);
  const [newId, setNewId] = useState(dayId);

  if (!day) {
    return (
      <div className={styles.emptyState}>
        <div className={styles.emptyStateIcon}>⚠️</div>
        <p className={styles.emptyStateText}>Day definition not found</p>
        <button
          type="button"
          onClick={() => setActivePanel({ type: 'days' })}
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
          Back to Day Definitions
        </button>
      </div>
    );
  }

  const getDateDefType = (): DateDefType => {
    if (!day.date_def || Object.keys(day.date_def).length === 0) return 'inherited';
    if ('date_fn' in day.date_def) return 'date_fn';
    if ('nth_week_in_month' in day.date_def) return 'nth_weekday';
    if ('last_day_of_week_in_month' in day.date_def) return 'last_weekday';
    if ('month' in day.date_def && 'date' in day.date_def) return 'fixed';
    return 'inherited';
  };

  const [dateType, setDateType] = useState<DateDefType>(getDateDefType());

  const handleIdSave = () => {
    if (newId && newId !== dayId && !calendar.days_definitions[newId]) {
      renameDayDefinition(dayId, newId);
      setActivePanel({ type: 'day', id: newId });
    }
    setEditingId(false);
  };

  const handleDateTypeChange = (type: DateDefType) => {
    setDateType(type);
    let newDateDef: DateDef | null = null;

    switch (type) {
      case 'fixed':
        newDateDef = { month: 1, date: 1 };
        break;
      case 'date_fn':
        newDateDef = { date_fn: 'EASTER_SUNDAY', day_offset: 0 };
        break;
      case 'nth_weekday':
        newDateDef = { month: 1, day_of_week: 0, nth_week_in_month: 1 };
        break;
      case 'last_weekday':
        newDateDef = { month: 1, last_day_of_week_in_month: 0 };
        break;
      case 'inherited':
        newDateDef = {};
        break;
    }

    updateDayDefinition(dayId, { date_def: newDateDef });
  };

  const getCommonValues = (): CommonDefinition[] => {
    if (!day.commons_def) return [];
    return Array.isArray(day.commons_def) ? day.commons_def : [day.commons_def];
  };

  const getTitleValues = (): Title[] => {
    if (!day.titles) return [];
    if (Array.isArray(day.titles)) return day.titles;
    return [...(day.titles.prepend || []), ...(day.titles.append || [])];
  };

  const handleCommonsChange = (values: CommonDefinition[]) => {
    updateDayDefinition(dayId, {
      commons_def: values.length === 0 ? null : values.length === 1 ? values[0] : values,
    });
  };

  const handleTitlesChange = (values: Title[]) => {
    updateDayDefinition(dayId, {
      titles: values.length === 0 ? null : values,
    });
  };

  const handleEntityChange = (value: string, index: number) => {
    const entities = [...(day.entities || [])];
    if (value) {
      entities[index] = value;
    } else {
      entities.splice(index, 1);
    }
    updateDayDefinition(dayId, { entities: entities.length > 0 ? entities : null });
  };

  const addEntity = () => {
    const entities = [...(day.entities || []), ''];
    updateDayDefinition(dayId, { entities });
  };

  return (
    <div>
      <div className={styles.sectionHeader}>
        <div>
          <button
            type="button"
            onClick={() => setActivePanel({ type: 'days' })}
            style={{
              border: 'none',
              background: 'none',
              cursor: 'pointer',
              fontSize: '0.875rem',
              color: 'var(--ifm-color-content-secondary)',
              padding: 0,
              marginBottom: '0.5rem',
            }}
          >
            ← Back to Day Definitions
          </button>
          {editingId ? (
            <div style={{ display: 'flex', gap: '0.5rem', alignItems: 'center' }}>
              <input
                type="text"
                className={styles.input}
                value={newId}
                onChange={(e) => setNewId(e.target.value.toLowerCase().replace(/[^a-z0-9_]/g, '_'))}
                onKeyDown={(e) => e.key === 'Enter' && handleIdSave()}
                style={{ width: '300px' }}
              />
              <button
                type="button"
                onClick={handleIdSave}
                style={{
                  padding: '0.5rem 0.75rem',
                  background: 'var(--ifm-color-primary)',
                  color: 'white',
                  border: 'none',
                  borderRadius: '4px',
                  cursor: 'pointer',
                }}
              >
                Save
              </button>
              <button
                type="button"
                onClick={() => { setEditingId(false); setNewId(dayId); }}
                style={{
                  padding: '0.5rem 0.75rem',
                  background: 'transparent',
                  border: '1px solid var(--ifm-toc-border-color)',
                  borderRadius: '4px',
                  cursor: 'pointer',
                }}
              >
                Cancel
              </button>
            </div>
          ) : (
            <h2 className={styles.sectionTitle} style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
              ✏️ {dayId}
              <button
                type="button"
                onClick={() => setEditingId(true)}
                style={{
                  border: 'none',
                  background: 'none',
                  cursor: 'pointer',
                  fontSize: '0.875rem',
                  color: 'var(--ifm-color-primary)',
                }}
                title="Rename"
              >
                (rename)
              </button>
            </h2>
          )}
        </div>
      </div>

      {/* Date Definition */}
      <div className={styles.card}>
        <h3 className={styles.cardTitle}>📅 Date</h3>

        <RadioGroup
          label="Date Type"
          name="dateType"
          value={dateType}
          options={[
            { value: 'fixed', label: 'Fixed Date' },
            { value: 'date_fn', label: 'Relative to Movable Feast' },
            { value: 'nth_weekday', label: 'Nth Weekday of Month' },
            { value: 'last_weekday', label: 'Last Weekday of Month' },
            { value: 'inherited', label: 'Inherited / None' },
          ]}
          onChange={(v) => handleDateTypeChange(v as DateDefType)}
        />

        {dateType === 'fixed' && day.date_def && 'month' in day.date_def && 'date' in day.date_def && (
          <div className={styles.dateInputGroup}>
            <Select
              label="Month"
              value={String(day.date_def.month)}
              options={MONTH_OPTIONS}
              onChange={(e) => updateDayDefinition(dayId, {
                date_def: { ...day.date_def, month: parseInt(e.target.value) } as DateDef,
              })}
            />
            <TextInput
              label="Day"
              type="number"
              min={1}
              max={31}
              value={String(day.date_def.date)}
              onChange={(e) => updateDayDefinition(dayId, {
                date_def: { ...day.date_def, date: parseInt(e.target.value) || 1 } as DateDef,
              })}
              className={styles.numberInput}
            />
            <TextInput
              label="Offset (days)"
              type="number"
              value={String(day.date_def.day_offset || 0)}
              onChange={(e) => updateDayDefinition(dayId, {
                date_def: { ...day.date_def, day_offset: parseInt(e.target.value) || 0 } as DateDef,
              })}
              className={styles.numberInput}
              hint="Optional"
            />
          </div>
        )}

        {dateType === 'date_fn' && day.date_def && 'date_fn' in day.date_def && (
          <div className={styles.dateInputGroup}>
            <Select
              label="Movable Feast"
              value={day.date_def.date_fn}
              options={DATE_FN_OPTIONS}
              onChange={(e) => updateDayDefinition(dayId, {
                date_def: { date_fn: e.target.value as DateFn, day_offset: day.date_def && 'day_offset' in day.date_def ? day.date_def.day_offset : 0 },
              })}
            />
            <TextInput
              label="Offset (days)"
              type="number"
              value={String(day.date_def.day_offset || 0)}
              onChange={(e) => updateDayDefinition(dayId, {
                date_def: { ...day.date_def, day_offset: parseInt(e.target.value) || 0 } as DateDef,
              })}
              className={styles.numberInput}
              hint="e.g., +50 for 50 days after"
            />
          </div>
        )}

        {dateType === 'nth_weekday' && day.date_def && 'nth_week_in_month' in day.date_def && (
          <div className={styles.dateInputGroup}>
            <TextInput
              label="Nth"
              type="number"
              min={1}
              max={5}
              value={String(day.date_def.nth_week_in_month)}
              onChange={(e) => updateDayDefinition(dayId, {
                date_def: { ...day.date_def, nth_week_in_month: parseInt(e.target.value) || 1 } as DateDef,
              })}
              className={styles.numberInput}
            />
            <Select
              label="Weekday"
              value={String(day.date_def.day_of_week)}
              options={WEEKDAY_OPTIONS}
              onChange={(e) => updateDayDefinition(dayId, {
                date_def: { ...day.date_def, day_of_week: parseInt(e.target.value) } as DateDef,
              })}
            />
            <Select
              label="Month"
              value={String(day.date_def.month)}
              options={MONTH_OPTIONS}
              onChange={(e) => updateDayDefinition(dayId, {
                date_def: { ...day.date_def, month: parseInt(e.target.value) } as DateDef,
              })}
            />
          </div>
        )}

        {dateType === 'last_weekday' && day.date_def && 'last_day_of_week_in_month' in day.date_def && (
          <div className={styles.dateInputGroup}>
            <Select
              label="Weekday"
              value={String(day.date_def.last_day_of_week_in_month)}
              options={WEEKDAY_OPTIONS}
              onChange={(e) => updateDayDefinition(dayId, {
                date_def: { ...day.date_def, last_day_of_week_in_month: parseInt(e.target.value) } as DateDef,
              })}
            />
            <Select
              label="Month"
              value={String(day.date_def.month)}
              options={MONTH_OPTIONS}
              onChange={(e) => updateDayDefinition(dayId, {
                date_def: { ...day.date_def, month: parseInt(e.target.value) } as DateDef,
              })}
            />
          </div>
        )}
      </div>

      {/* Precedence */}
      <div className={styles.card}>
        <h3 className={styles.cardTitle}>📊 Liturgical Rank</h3>

        <Select
          label="Precedence"
          value={day.precedence || ''}
          options={[{ value: '', label: '— Select —' }, ...PRECEDENCE_OPTIONS]}
          onChange={(e) => updateDayDefinition(dayId, {
            precedence: e.target.value ? (e.target.value as Precedence) : null,
          })}
        />

        <div style={{ display: 'flex', gap: '2rem', marginTop: '1rem' }}>
          <Checkbox
            label="Holy Day of Obligation"
            hint="Faithful are expected to attend Mass"
            checked={day.is_holy_day_of_obligation ?? false}
            onChange={(e) => updateDayDefinition(dayId, { is_holy_day_of_obligation: e.target.checked || null })}
          />

          <Checkbox
            label="Optional"
            hint="Can be omitted in favor of weekday"
            checked={day.is_optional ?? false}
            onChange={(e) => updateDayDefinition(dayId, { is_optional: e.target.checked || null })}
          />
        </div>
      </div>

      {/* Commons */}
      <div className={styles.card}>
        <h3 className={styles.cardTitle}>📖 Commons</h3>

        <MultiSelect
          label="Common(s)"
          values={getCommonValues()}
          options={COMMON_OPTIONS}
          onChange={handleCommonsChange}
          hint="Select the appropriate Common for Mass prayers and readings"
        />
      </div>

      {/* Entities */}
      <div className={styles.card}>
        <h3 className={styles.cardTitle}>👤 Associated Entities</h3>

        <p style={{ fontSize: '0.875rem', color: 'var(--ifm-color-content-secondary)', marginBottom: '1rem' }}>
          Link saints, blessed, or other entities to this day.
        </p>

        {(day.entities || []).map((entity, index) => {
          const entityId = typeof entity === 'string' ? entity : entity.id;
          return (
            <div key={index} style={{ display: 'flex', gap: '0.5rem', marginBottom: '0.5rem' }}>
              <input
                type="text"
                className={styles.input}
                value={entityId}
                onChange={(e) => handleEntityChange(e.target.value, index)}
                placeholder="Entity ID (e.g., john_the_apostle)"
              />
              <button
                type="button"
                onClick={() => handleEntityChange('', index)}
                style={{
                  padding: '0.5rem 0.75rem',
                  background: 'transparent',
                  border: '1px solid var(--ifm-color-danger)',
                  color: 'var(--ifm-color-danger)',
                  borderRadius: '4px',
                  cursor: 'pointer',
                }}
              >
                Remove
              </button>
            </div>
          );
        })}

        <button
          type="button"
          onClick={addEntity}
          style={{
            padding: '0.375rem 0.75rem',
            background: 'var(--ifm-color-primary-lightest)',
            color: 'var(--ifm-color-primary)',
            border: 'none',
            borderRadius: '4px',
            cursor: 'pointer',
            fontSize: '0.875rem',
          }}
        >
          + Add Entity
        </button>
      </div>

      {/* Titles */}
      <div className={styles.card}>
        <h3 className={styles.cardTitle}>🏷️ Titles</h3>

        <MultiSelect
          label="Title(s)"
          values={getTitleValues()}
          options={TITLE_OPTIONS}
          onChange={handleTitlesChange}
          hint="Add or override titles for this day's entities"
        />
      </div>

      {/* Advanced Options */}
      <details className={styles.card}>
        <summary style={{ cursor: 'pointer', fontWeight: 600 }}>
          ⚙️ Advanced Options
        </summary>
        <div style={{ marginTop: '1rem' }}>
          <TextInput
            label="Custom Locale ID"
            value={day.custom_locale_id || ''}
            onChange={(e) => updateDayDefinition(dayId, { custom_locale_id: e.target.value || null })}
            hint="Override the locale key for translations"
            placeholder="e.g., saint_john_apostle_custom"
          />

          <Checkbox
            label="Allow Similar Rank Items"
            hint="Allow other days with same rank to coexist"
            checked={day.allow_similar_rank_items ?? false}
            onChange={(e) => updateDayDefinition(dayId, { allow_similar_rank_items: e.target.checked || null })}
          />

          <Checkbox
            label="Drop from Calendar"
            hint="Remove this day from the calendar entirely"
            checked={day.drop ?? false}
            onChange={(e) => updateDayDefinition(dayId, { drop: e.target.checked || null })}
          />
        </div>
      </details>
    </div>
  );
}
