import type { ReactNode } from 'react';
import { useEditorStore } from '../context/useEditorStore';
import { TextInput, Select, RadioGroup, Checkbox } from '../shared/FormField';
import { validateCalendarId } from '../utils/validation';
import { generateCalendarId, getCalendarTypeLabel } from '../utils/dataLoader';
import type { CalendarType, CalendarJurisdiction } from '../types';
import styles from '../shared/shared.module.css';

const CALENDAR_TYPES: { value: CalendarType; label: string }[] = [
  { value: 'GENERAL_ROMAN', label: 'General Roman' },
  { value: 'REGION', label: 'Region' },
  { value: 'COUNTRY', label: 'Country' },
  { value: 'ARCHDIOCESE', label: 'Archdiocese' },
  { value: 'DIOCESE', label: 'Diocese' },
  { value: 'CITY', label: 'City' },
  { value: 'PARISH', label: 'Parish' },
  { value: 'GENERAL_COMMUNITY', label: 'General Community' },
  { value: 'REGIONAL_COMMUNITY', label: 'Regional Community' },
  { value: 'LOCAL_COMMUNITY', label: 'Local Community' },
  { value: 'OTHER', label: 'Other' },
];

const JURISDICTION_OPTIONS: { value: CalendarJurisdiction; label: string }[] = [
  { value: 'ECCLESIASTICAL', label: 'Ecclesiastical' },
  { value: 'CIVIL', label: 'Civil' },
];

interface MetadataPanelProps {
  showConfig?: boolean;
}

export default function MetadataPanel({ showConfig = false }: MetadataPanelProps): ReactNode {
  const { calendar, updateMetadata, updateParticularConfig, validationErrors } = useEditorStore();

  const handleIdChange = (value: string) => {
    // Update the calendar ID (using setCalendar would reset original)
    useEditorStore.setState((state) => {
      state.calendar.id = value;
      state.isDirty = true;
    });
  };

  const idValidation = validateCalendarId(calendar.id);
  const idError = !idValidation.valid ? idValidation.errors[0]?.message : undefined;

  if (showConfig) {
    return (
      <div>
        <h2 className={styles.sectionTitle}>Particular Configuration</h2>
        <p style={{ color: 'var(--ifm-color-content-secondary)', marginBottom: '1.5rem' }}>
          Configure how certain movable feasts are celebrated in this calendar. These settings override the parent
          calendar configuration.
        </p>

        <div className={styles.card}>
          <h3 className={styles.cardTitle}>Movable Feasts</h3>

          <Checkbox
            label="Epiphany on Sunday"
            hint="Celebrate Epiphany on the Sunday between January 2 and 8"
            checked={calendar.particular_config?.epiphany_on_sunday ?? false}
            onChange={(e) => updateParticularConfig({ epiphany_on_sunday: e.target.checked })}
          />

          <Checkbox
            label="Ascension on Sunday"
            hint="Celebrate Ascension on the 7th Sunday of Easter instead of Thursday"
            checked={calendar.particular_config?.ascension_on_sunday ?? false}
            onChange={(e) => updateParticularConfig({ ascension_on_sunday: e.target.checked })}
          />

          <Checkbox
            label="Corpus Christi on Sunday"
            hint="Celebrate Corpus Christi on Sunday instead of Thursday"
            checked={calendar.particular_config?.corpus_christi_on_sunday ?? false}
            onChange={(e) => updateParticularConfig({ corpus_christi_on_sunday: e.target.checked })}
          />
        </div>

        <div className={styles.card}>
          <h3 className={styles.cardTitle}>Easter Calculation</h3>

          <RadioGroup
            label="Easter Calculation Method"
            name="easter_calculation"
            value={calendar.particular_config?.easter_calculation_type || 'GREGORIAN'}
            options={[
              { value: 'GREGORIAN', label: 'Gregorian (default)' },
              { value: 'JULIAN', label: 'Julian (converted to Gregorian)' },
            ]}
            onChange={(value) =>
              updateParticularConfig({
                easter_calculation_type: value as 'GREGORIAN' | 'JULIAN',
              })
            }
            hint="Most calendars use Gregorian calculation. Julian is used by some Eastern churches."
          />
        </div>
      </div>
    );
  }

  return (
    <div>
      <h2 className={styles.sectionTitle}>Calendar Metadata</h2>
      <p style={{ color: 'var(--ifm-color-content-secondary)', marginBottom: '1.5rem' }}>
        Define the basic information about this liturgical calendar.
      </p>

      <div className={styles.card}>
        <h3 className={styles.cardTitle}>Identification</h3>

        <TextInput
          label="Calendar ID"
          required
          value={calendar.id}
          onChange={(e) => handleIdChange(e.target.value)}
          placeholder="e.g., france__lyon"
          hint="Format: snake_case. Use double underscore (__) for hierarchy (e.g., france__lyon)."
          error={idError}
        />

        <Select
          label="Calendar Type"
          required
          value={calendar.metadata.type}
          options={CALENDAR_TYPES}
          onChange={(e) => updateMetadata({ type: e.target.value as CalendarType })}
          hint="The scope and authority level of this calendar."
        />
      </div>

      <div className={styles.card}>
        <h3 className={styles.cardTitle}>Jurisdiction</h3>

        <RadioGroup
          label="Jurisdiction Type"
          name="jurisdiction"
          required
          value={calendar.metadata.jurisdiction}
          options={JURISDICTION_OPTIONS}
          onChange={(value) => updateMetadata({ jurisdiction: value as CalendarJurisdiction })}
          hint="Ecclesiastical follows church boundaries. Civil follows state/country boundaries."
        />
      </div>

      {calendar.id && (
        <div className={styles.card}>
          <h3 className={styles.cardTitle}>Summary</h3>
          <dl style={{ margin: 0 }}>
            <div style={{ display: 'flex', gap: '1rem', marginBottom: '0.5rem' }}>
              <dt style={{ fontWeight: 500 }}>ID:</dt>
              <dd style={{ margin: 0, fontFamily: 'monospace' }}>{calendar.id}</dd>
            </div>
            <div style={{ display: 'flex', gap: '1rem', marginBottom: '0.5rem' }}>
              <dt style={{ fontWeight: 500 }}>Type:</dt>
              <dd style={{ margin: 0 }}>{getCalendarTypeLabel(calendar.metadata.type)}</dd>
            </div>
            <div style={{ display: 'flex', gap: '1rem' }}>
              <dt style={{ fontWeight: 500 }}>Jurisdiction:</dt>
              <dd style={{ margin: 0 }}>{calendar.metadata.jurisdiction}</dd>
            </div>
          </dl>
        </div>
      )}
    </div>
  );
}
