import type { ReactNode } from 'react';
import { useState } from 'react';
import { useEditorStore } from '../context/useEditorStore';
import { TextInput, Select, RadioGroup, Checkbox } from '../shared/FormField';
import { MultiSelect } from '../shared/MultiSelect';
import type { Title, CanonizationLevel, EntityType, Sex } from '../types';
import styles from '../shared/shared.module.css';

// Title options
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
  { value: 'PATRIARCH', label: 'Patriarch' },
  { value: 'PILGRIM', label: 'Pilgrim' },
  { value: 'FIRST_BISHOP', label: 'First Bishop' },
  { value: 'EMPRESS', label: 'Empress' },
];

const ENTITY_TYPE_OPTIONS: { value: EntityType; label: string }[] = [
  { value: 'PERSON', label: 'Person' },
  { value: 'PLACE', label: 'Place' },
  { value: 'EVENT', label: 'Event' },
];

const CANONIZATION_OPTIONS: { value: CanonizationLevel; label: string }[] = [
  { value: 'SAINT', label: 'Saint' },
  { value: 'BLESSED', label: 'Blessed' },
];

const SEX_OPTIONS: { value: Sex; label: string }[] = [
  { value: 'MALE', label: 'Male' },
  { value: 'FEMALE', label: 'Female' },
];

interface EntityEditorPanelProps {
  entityId: string;
  locale: string;
}

export default function EntityEditorPanel({ entityId, locale }: EntityEditorPanelProps): ReactNode {
  const {
    entities,
    updateEntity,
    renameEntity,
    setActivePanel,
  } = useEditorStore();

  const entity = entities[locale]?.[entityId];
  const [editingId, setEditingId] = useState(false);
  const [newId, setNewId] = useState(entityId);

  if (!entity) {
    return (
      <div className={styles.emptyState}>
        <div className={styles.emptyStateIcon}>⚠️</div>
        <p className={styles.emptyStateText}>Entity not found</p>
        <button
          type="button"
          onClick={() => setActivePanel({ type: 'entities', locale })}
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
          Back to Entities
        </button>
      </div>
    );
  }

  const handleIdSave = () => {
    if (newId && newId !== entityId && !entities[locale]?.[newId]) {
      renameEntity(locale, entityId, newId);
      setActivePanel({ type: 'entity', id: newId, locale });
    }
    setEditingId(false);
  };

  const handleTitlesChange = (values: Title[]) => {
    updateEntity(locale, entityId, {
      titles: values.length > 0 ? values : undefined,
    });
  };

  const handleSourceAdd = () => {
    const sources = [...(entity.sources || []), ''];
    updateEntity(locale, entityId, { sources });
  };

  const handleSourceChange = (index: number, value: string) => {
    const sources = [...(entity.sources || [])];
    if (value) {
      sources[index] = value;
    } else {
      sources.splice(index, 1);
    }
    updateEntity(locale, entityId, { sources: sources.length > 0 ? sources : undefined });
  };

  return (
    <div>
      <div className={styles.sectionHeader}>
        <div>
          <button
            type="button"
            onClick={() => setActivePanel({ type: 'entities', locale })}
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
            ← Back to Entities ({locale.toUpperCase()})
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
                onClick={() => { setEditingId(false); setNewId(entityId); }}
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
              👤 {entityId}
              <span style={{ fontSize: '0.75rem', background: 'var(--ifm-hover-overlay)', padding: '0.125rem 0.5rem', borderRadius: '4px' }}>
                {locale.toUpperCase()}
              </span>
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

      {/* Entity Type */}
      <div className={styles.card}>
        <h3 className={styles.cardTitle}>🏷️ Entity Type</h3>

        <RadioGroup
          label="Type"
          name="entityType"
          value={entity.type || 'PERSON'}
          options={ENTITY_TYPE_OPTIONS}
          onChange={(value) => updateEntity(locale, entityId, { type: value as EntityType })}
          hint="Most entities are Persons (saints, blessed). Use Place for churches or Event for historical occurrences."
        />
      </div>

      {/* Identity */}
      <div className={styles.card}>
        <h3 className={styles.cardTitle}>📝 Identity</h3>

        <TextInput
          label="Full Name"
          value={entity.fullname || ''}
          onChange={(e) => updateEntity(locale, entityId, { fullname: e.target.value || undefined })}
          placeholder="e.g., Saint John, Apostle and Evangelist"
          hint="The complete name including canonization level and titles"
        />

        <TextInput
          label="Short Name"
          value={entity.name || ''}
          onChange={(e) => updateEntity(locale, entityId, { name: e.target.value || undefined })}
          placeholder="e.g., John"
          hint="Name without canonization level or titles"
        />

        {entity.type !== 'PLACE' && entity.type !== 'EVENT' && (
          <>
            <RadioGroup
              label="Canonization Level"
              name="canonizationLevel"
              value={entity.canonization_level || ''}
              options={[{ value: '', label: 'None' }, ...CANONIZATION_OPTIONS]}
              onChange={(value) => updateEntity(locale, entityId, {
                canonization_level: value ? (value as CanonizationLevel) : undefined,
              })}
            />

            <Checkbox
              label="Hide canonization level in display"
              hint="When the level is already included in the name"
              checked={entity.hide_canonization_level ?? false}
              onChange={(e) => updateEntity(locale, entityId, {
                hide_canonization_level: e.target.checked || undefined,
              })}
            />
          </>
        )}
      </div>

      {/* Titles */}
      {entity.type !== 'PLACE' && entity.type !== 'EVENT' && (
        <div className={styles.card}>
          <h3 className={styles.cardTitle}>👑 Titles</h3>

          <MultiSelect
            label="Titles"
            values={entity.titles || []}
            options={TITLE_OPTIONS}
            onChange={handleTitlesChange}
            hint="Select ecclesiastical titles (Martyr, Bishop, Virgin, etc.)"
          />

          <Checkbox
            label="Hide titles in display"
            hint="When titles are already included in the name"
            checked={entity.hide_titles ?? false}
            onChange={(e) => updateEntity(locale, entityId, {
              hide_titles: e.target.checked || undefined,
            })}
          />
        </div>
      )}

      {/* Personal Info */}
      {entity.type !== 'PLACE' && entity.type !== 'EVENT' && (
        <div className={styles.card}>
          <h3 className={styles.cardTitle}>👤 Personal Information</h3>

          <RadioGroup
            label="Sex"
            name="sex"
            value={entity.sex || ''}
            options={[{ value: '', label: 'Not specified' }, ...SEX_OPTIONS]}
            onChange={(value) => updateEntity(locale, entityId, {
              sex: value ? (value as Sex) : undefined,
            })}
          />

          <TextInput
            label="Number of Persons"
            type="text"
            value={entity.count === 'MANY' ? 'MANY' : String(entity.count || '')}
            onChange={(e) => {
              const val = e.target.value.toUpperCase();
              if (val === 'MANY') {
                updateEntity(locale, entityId, { count: 'MANY' });
              } else if (val === '' || val === '0') {
                updateEntity(locale, entityId, { count: undefined });
              } else {
                const num = parseInt(val);
                if (!isNaN(num) && num > 0) {
                  updateEntity(locale, entityId, { count: num });
                }
              }
            }}
            placeholder="e.g., 12 or MANY"
            hint="For groups of martyrs, enter the count or 'MANY'"
          />
        </div>
      )}

      {/* Dates */}
      <div className={styles.card}>
        <h3 className={styles.cardTitle}>📅 Dates</h3>

        {entity.type === 'PLACE' ? (
          <>
            <TextInput
              label="Date of Dedication"
              value={String(entity.date_of_dedication || '')}
              onChange={(e) => {
                const val = e.target.value;
                if (!val) {
                  updateEntity(locale, entityId, { date_of_dedication: undefined });
                } else if (/^\d{4}(-\d{2})?(-\d{2})?$/.test(val)) {
                  updateEntity(locale, entityId, { date_of_dedication: val.includes('-') ? val : parseInt(val) });
                }
              }}
              placeholder="YYYY or YYYY-MM or YYYY-MM-DD"
              hint="When the church/place was dedicated"
            />
          </>
        ) : entity.type !== 'EVENT' ? (
          <>
            <div className={styles.dateInputGroup}>
              <div style={{ flex: 1 }}>
                <TextInput
                  label="Date of Birth"
                  value={String(entity.date_of_birth || '')}
                  onChange={(e) => {
                    const val = e.target.value;
                    if (!val) {
                      updateEntity(locale, entityId, { date_of_birth: undefined });
                    } else if (/^\d{1,4}(-\d{2})?(-\d{2})?$/.test(val)) {
                      updateEntity(locale, entityId, { date_of_birth: val.includes('-') ? val : parseInt(val) });
                    }
                  }}
                  placeholder="YYYY or YYYY-MM or YYYY-MM-DD"
                />
                <Checkbox
                  label="Approximate"
                  checked={entity.date_of_birth_is_approximative ?? false}
                  onChange={(e) => updateEntity(locale, entityId, {
                    date_of_birth_is_approximative: e.target.checked || undefined,
                  })}
                />
              </div>
              <div style={{ flex: 1 }}>
                <TextInput
                  label="Date of Death"
                  value={String(entity.date_of_death || '')}
                  onChange={(e) => {
                    const val = e.target.value;
                    if (!val) {
                      updateEntity(locale, entityId, { date_of_death: undefined });
                    } else if (/^\d{1,4}(-\d{2})?(-\d{2})?$/.test(val)) {
                      updateEntity(locale, entityId, { date_of_death: val.includes('-') ? val : parseInt(val) });
                    }
                  }}
                  placeholder="YYYY or YYYY-MM or YYYY-MM-DD"
                />
                <Checkbox
                  label="Approximate"
                  checked={entity.date_of_death_is_approximative ?? false}
                  onChange={(e) => updateEntity(locale, entityId, {
                    date_of_death_is_approximative: e.target.checked || undefined,
                  })}
                />
              </div>
            </div>

            <div className={styles.dateInputGroup} style={{ marginTop: '1rem' }}>
              <div style={{ flex: 1 }}>
                <TextInput
                  label="Date of Beatification"
                  value={String(entity.date_of_beatification || '')}
                  onChange={(e) => {
                    const val = e.target.value;
                    if (!val) {
                      updateEntity(locale, entityId, { date_of_beatification: undefined });
                    } else if (/^\d{4}(-\d{2})?(-\d{2})?$/.test(val)) {
                      updateEntity(locale, entityId, { date_of_beatification: val.includes('-') ? val : parseInt(val) });
                    }
                  }}
                  placeholder="YYYY or YYYY-MM or YYYY-MM-DD"
                />
                <Checkbox
                  label="Approximate"
                  checked={entity.date_of_beatification_is_approximative ?? false}
                  onChange={(e) => updateEntity(locale, entityId, {
                    date_of_beatification_is_approximative: e.target.checked || undefined,
                  })}
                />
              </div>
              <div style={{ flex: 1 }}>
                <TextInput
                  label="Date of Canonization"
                  value={String(entity.date_of_canonization || '')}
                  onChange={(e) => {
                    const val = e.target.value;
                    if (!val) {
                      updateEntity(locale, entityId, { date_of_canonization: undefined });
                    } else if (/^\d{4}(-\d{2})?(-\d{2})?$/.test(val)) {
                      updateEntity(locale, entityId, { date_of_canonization: val.includes('-') ? val : parseInt(val) });
                    }
                  }}
                  placeholder="YYYY or YYYY-MM or YYYY-MM-DD"
                />
                <Checkbox
                  label="Approximate"
                  checked={entity.date_of_canonization_is_approximative ?? false}
                  onChange={(e) => updateEntity(locale, entityId, {
                    date_of_canonization_is_approximative: e.target.checked || undefined,
                  })}
                />
              </div>
            </div>
          </>
        ) : null}
      </div>

      {/* Sources */}
      <div className={styles.card}>
        <h3 className={styles.cardTitle}>📚 Sources</h3>

        <p style={{ fontSize: '0.875rem', color: 'var(--ifm-color-content-secondary)', marginBottom: '1rem' }}>
          Add references and links to sources about this entity.
        </p>

        {(entity.sources || []).map((source, index) => (
          <div key={index} style={{ display: 'flex', gap: '0.5rem', marginBottom: '0.5rem' }}>
            <input
              type="text"
              className={styles.input}
              value={source}
              onChange={(e) => handleSourceChange(index, e.target.value)}
              placeholder="URL or reference"
            />
            <button
              type="button"
              onClick={() => handleSourceChange(index, '')}
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
        ))}

        <button
          type="button"
          onClick={handleSourceAdd}
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
          + Add Source
        </button>
      </div>
    </div>
  );
}
