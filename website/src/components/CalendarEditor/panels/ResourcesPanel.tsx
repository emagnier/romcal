import type { ReactNode } from 'react';
import { useState, useMemo } from 'react';
import { useEditorStore } from '../context/useEditorStore';
import { AVAILABLE_LOCALES } from '../utils/dataLoader';
import type { EntityDefinition } from '../types';
import styles from '../shared/shared.module.css';

interface ResourcesPanelProps {
  locale?: string;
}

export default function ResourcesPanel({ locale }: ResourcesPanelProps): ReactNode {
  const { entities, addEntity, removeEntity, setActivePanel } = useEditorStore();

  const [selectedLocale, setSelectedLocale] = useState(locale || 'en');
  const [searchQuery, setSearchQuery] = useState('');
  const [showNewEntityForm, setShowNewEntityForm] = useState(false);
  const [newEntityId, setNewEntityId] = useState('');

  const localeEntities = entities[selectedLocale] || {};

  const entityEntries = useMemo(() => {
    return Object.entries(localeEntities).sort((a, b) => a[0].localeCompare(b[0]));
  }, [localeEntities]);

  const filteredEntities = useMemo(() => {
    if (!searchQuery) return entityEntries;
    const query = searchQuery.toLowerCase();
    return entityEntries.filter(
      ([id, entity]) =>
        id.toLowerCase().includes(query) ||
        entity.fullname?.toLowerCase().includes(query) ||
        entity.name?.toLowerCase().includes(query)
    );
  }, [entityEntries, searchQuery]);

  const handleCreateEntity = () => {
    if (newEntityId && !localeEntities[newEntityId]) {
      const newEntity: EntityDefinition = {
        fullname: '',
        canonization_level: 'SAINT',
      };
      addEntity(selectedLocale, newEntityId, newEntity);
      setActivePanel({ type: 'entity', id: newEntityId, locale: selectedLocale });
      setShowNewEntityForm(false);
      setNewEntityId('');
    }
  };

  const handleEditEntity = (id: string) => {
    setActivePanel({ type: 'entity', id, locale: selectedLocale });
  };

  const handleDeleteEntity = (id: string) => {
    if (confirm(`Are you sure you want to delete entity "${id}" for locale "${selectedLocale}"?`)) {
      removeEntity(selectedLocale, id);
    }
  };

  const getCanonizationBadge = (level?: string) => {
    if (!level) return null;
    const colors: Record<string, { bg: string; text: string }> = {
      SAINT: { bg: 'rgba(251, 191, 36, 0.2)', text: 'rgb(161, 98, 7)' },
      BLESSED: { bg: 'rgba(147, 51, 234, 0.2)', text: 'rgb(107, 33, 168)' },
    };
    const style = colors[level] || { bg: 'var(--ifm-hover-overlay)', text: 'inherit' };
    return (
      <span
        style={{
          padding: '0.125rem 0.375rem',
          fontSize: '0.6875rem',
          fontWeight: 500,
          background: style.bg,
          color: style.text,
          borderRadius: '4px',
          textTransform: 'uppercase',
        }}
      >
        {level}
      </span>
    );
  };

  return (
    <div>
      <div className={styles.sectionHeader}>
        <h2 className={styles.sectionTitle}>Entity Resources</h2>
        <button
          type="button"
          onClick={() => setShowNewEntityForm(true)}
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
          + Add Entity
        </button>
      </div>

      <p style={{ color: 'var(--ifm-color-content-secondary)', marginBottom: '1.5rem' }}>
        Define saints, blessed, places, and events with translations for each locale.
      </p>

      {/* Locale Selector */}
      <div className={styles.card}>
        <h3 className={styles.cardTitle}>🌐 Locale</h3>
        <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.5rem' }}>
          {AVAILABLE_LOCALES.map((loc) => (
            <button
              key={loc.code}
              type="button"
              onClick={() => setSelectedLocale(loc.code)}
              style={{
                padding: '0.375rem 0.75rem',
                fontSize: '0.875rem',
                background: selectedLocale === loc.code ? 'var(--ifm-color-primary)' : 'var(--ifm-background-color)',
                color: selectedLocale === loc.code ? 'white' : 'var(--ifm-color-content)',
                border: `1px solid ${selectedLocale === loc.code ? 'var(--ifm-color-primary)' : 'var(--ifm-toc-border-color)'}`,
                borderRadius: 'var(--ifm-button-border-radius)',
                cursor: 'pointer',
              }}
            >
              {loc.name}
              {entities[loc.code] && (
                <span style={{ marginLeft: '0.375rem', opacity: 0.7 }}>({Object.keys(entities[loc.code]).length})</span>
              )}
            </button>
          ))}
        </div>
      </div>

      {/* New Entity Form */}
      {showNewEntityForm && (
        <div className={styles.card}>
          <h3 className={styles.cardTitle}>New Entity for {selectedLocale.toUpperCase()}</h3>
          <div className={styles.inlineForm}>
            <div style={{ flex: 1 }}>
              <input
                type="text"
                className={styles.input}
                placeholder="Entity ID (e.g., john_the_apostle)"
                value={newEntityId}
                onChange={(e) => setNewEntityId(e.target.value.toLowerCase().replace(/[^a-z0-9_]/g, '_'))}
                onKeyDown={(e) => e.key === 'Enter' && handleCreateEntity()}
              />
            </div>
            <div className={styles.buttonGroup}>
              <button
                type="button"
                onClick={handleCreateEntity}
                disabled={!newEntityId || !!localeEntities[newEntityId]}
                style={{
                  padding: '0.5rem 1rem',
                  fontSize: '0.875rem',
                  background: 'var(--ifm-color-primary)',
                  color: 'white',
                  border: 'none',
                  borderRadius: 'var(--ifm-button-border-radius)',
                  cursor: 'pointer',
                  opacity: !newEntityId || !!localeEntities[newEntityId] ? 0.5 : 1,
                }}
              >
                Create
              </button>
              <button
                type="button"
                onClick={() => {
                  setShowNewEntityForm(false);
                  setNewEntityId('');
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
          {newEntityId && localeEntities[newEntityId] && (
            <p className={styles.error}>An entity with this ID already exists for this locale</p>
          )}
        </div>
      )}

      {/* Search */}
      <div className={styles.searchBox}>
        <span className={styles.searchIcon}>🔍</span>
        <input
          type="text"
          className={styles.searchInput}
          placeholder="Filter entities..."
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
        />
      </div>

      {/* Entity List */}
      {filteredEntities.length === 0 ? (
        <div className={styles.emptyState}>
          <div className={styles.emptyStateIcon}>👤</div>
          <p className={styles.emptyStateText}>
            {searchQuery ? 'No matching entities found' : `No entities defined for ${selectedLocale.toUpperCase()} yet`}
          </p>
          {!searchQuery && (
            <button
              type="button"
              onClick={() => setShowNewEntityForm(true)}
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
              Add your first entity
            </button>
          )}
        </div>
      ) : (
        <table className={styles.table}>
          <thead>
            <tr>
              <th>ID</th>
              <th>Name</th>
              <th>Level</th>
              <th style={{ width: '100px' }}>Actions</th>
            </tr>
          </thead>
          <tbody>
            {filteredEntities.map(([id, entity]) => (
              <tr key={id}>
                <td style={{ fontFamily: 'monospace', fontSize: '0.875rem' }}>{id}</td>
                <td>
                  {entity.fullname || entity.name || <em style={{ opacity: 0.5 }}>No name</em>}
                  {entity.titles && entity.titles.length > 0 && (
                    <div style={{ fontSize: '0.75rem', color: 'var(--ifm-color-content-secondary)' }}>
                      {entity.titles.slice(0, 3).join(', ')}
                      {entity.titles.length > 3 && ` +${entity.titles.length - 3}`}
                    </div>
                  )}
                </td>
                <td>{getCanonizationBadge(entity.canonization_level)}</td>
                <td>
                  <div className={styles.tableActions}>
                    <button
                      type="button"
                      onClick={() => handleEditEntity(id)}
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
                      onClick={() => handleDeleteEntity(id)}
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
        {filteredEntities.length} entit{filteredEntities.length !== 1 ? 'ies' : 'y'} in {selectedLocale.toUpperCase()}
      </div>
    </div>
  );
}
