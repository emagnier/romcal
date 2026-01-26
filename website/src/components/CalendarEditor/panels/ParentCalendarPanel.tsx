import type { ReactNode } from 'react';
import { useState, useMemo } from 'react';
import { useEditorStore } from '../context/useEditorStore';
import { groupCalendarsByType, getCalendarTypeLabel } from '../utils/dataLoader';
import styles from '../shared/shared.module.css';
import clsx from 'clsx';

export default function ParentCalendarPanel(): ReactNode {
  const { calendar, availableCalendars, setParentCalendarIds } = useEditorStore();

  const [searchQuery, setSearchQuery] = useState('');

  const groupedCalendars = useMemo(() => {
    return groupCalendarsByType(availableCalendars);
  }, [availableCalendars]);

  const filteredCalendars = useMemo(() => {
    if (!searchQuery) return availableCalendars;
    const query = searchQuery.toLowerCase();
    return availableCalendars.filter((c) => c.id.toLowerCase().includes(query) || c.name.toLowerCase().includes(query));
  }, [availableCalendars, searchQuery]);

  const handleAddParent = (id: string) => {
    if (!calendar.parent_calendar_ids.includes(id)) {
      setParentCalendarIds([...calendar.parent_calendar_ids, id]);
    }
  };

  const handleRemoveParent = (id: string) => {
    setParentCalendarIds(calendar.parent_calendar_ids.filter((p) => p !== id));
  };

  const handleMoveUp = (index: number) => {
    if (index === 0) return;
    const newIds = [...calendar.parent_calendar_ids];
    [newIds[index - 1], newIds[index]] = [newIds[index], newIds[index - 1]];
    setParentCalendarIds(newIds);
  };

  const handleMoveDown = (index: number) => {
    if (index === calendar.parent_calendar_ids.length - 1) return;
    const newIds = [...calendar.parent_calendar_ids];
    [newIds[index], newIds[index + 1]] = [newIds[index + 1], newIds[index]];
    setParentCalendarIds(newIds);
  };

  const getCalendarInfo = (id: string) => {
    return availableCalendars.find((c) => c.id === id);
  };

  return (
    <div>
      <h2 className={styles.sectionTitle}>Parent Calendars</h2>
      <p style={{ color: 'var(--ifm-color-content-secondary)', marginBottom: '1.5rem' }}>
        This calendar inherits day definitions from its parent calendars. The order matters: later parents can override
        earlier ones.
      </p>

      {/* Selected Parents */}
      <div className={styles.card}>
        <h3 className={styles.cardTitle}>Selected Parents</h3>

        {calendar.parent_calendar_ids.length === 0 ? (
          <div className={styles.emptyState}>
            <div className={styles.emptyStateIcon}>📋</div>
            <p className={styles.emptyStateText}>No parent calendars selected</p>
          </div>
        ) : (
          <ul className={styles.sortableList}>
            {calendar.parent_calendar_ids.map((id, index) => {
              const info = getCalendarInfo(id);
              return (
                <li key={id} className={styles.sortableItem}>
                  <span className={styles.sortableHandle}>≡</span>
                  <div className={styles.sortableContent}>
                    <strong>
                      {index + 1}. {info?.name || id}
                    </strong>
                    {info && (
                      <span style={{ marginLeft: '0.5rem', opacity: 0.7, fontSize: '0.875rem' }}>
                        ({getCalendarTypeLabel(info.type)})
                      </span>
                    )}
                  </div>
                  <div className={styles.sortableActions}>
                    <button
                      type="button"
                      onClick={() => handleMoveUp(index)}
                      disabled={index === 0}
                      style={{
                        border: 'none',
                        background: 'none',
                        cursor: index === 0 ? 'not-allowed' : 'pointer',
                        opacity: index === 0 ? 0.3 : 1,
                        fontSize: '1rem',
                      }}
                      title="Move up"
                    >
                      ↑
                    </button>
                    <button
                      type="button"
                      onClick={() => handleMoveDown(index)}
                      disabled={index === calendar.parent_calendar_ids.length - 1}
                      style={{
                        border: 'none',
                        background: 'none',
                        cursor: index === calendar.parent_calendar_ids.length - 1 ? 'not-allowed' : 'pointer',
                        opacity: index === calendar.parent_calendar_ids.length - 1 ? 0.3 : 1,
                        fontSize: '1rem',
                      }}
                      title="Move down"
                    >
                      ↓
                    </button>
                    <button
                      type="button"
                      onClick={() => handleRemoveParent(id)}
                      style={{
                        border: 'none',
                        background: 'none',
                        cursor: 'pointer',
                        color: 'var(--ifm-color-danger)',
                        fontSize: '1rem',
                      }}
                      title="Remove"
                    >
                      ×
                    </button>
                  </div>
                </li>
              );
            })}
          </ul>
        )}
      </div>

      {/* Available Calendars */}
      <div className={styles.card}>
        <h3 className={styles.cardTitle}>Available Calendars</h3>

        <div className={styles.searchBox}>
          <span className={styles.searchIcon}>🔍</span>
          <input
            type="text"
            className={styles.searchInput}
            placeholder="Search calendars..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
          />
        </div>

        <div style={{ maxHeight: '400px', overflowY: 'auto' }}>
          {searchQuery ? (
            // Search results
            <div>
              {filteredCalendars.length === 0 ? (
                <p style={{ color: 'var(--ifm-color-content-secondary)', textAlign: 'center' }}>No calendars found</p>
              ) : (
                filteredCalendars.map((cal) => {
                  const isSelected = calendar.parent_calendar_ids.includes(cal.id);
                  const isSelf = cal.id === calendar.id;
                  return (
                    <div
                      key={cal.id}
                      style={{
                        display: 'flex',
                        alignItems: 'center',
                        justifyContent: 'space-between',
                        padding: '0.5rem 0.75rem',
                        borderBottom: '1px solid var(--ifm-toc-border-color)',
                        opacity: isSelf ? 0.5 : 1,
                      }}
                    >
                      <div>
                        <strong>{cal.name}</strong>
                        <span style={{ marginLeft: '0.5rem', opacity: 0.7, fontSize: '0.875rem' }}>({cal.id})</span>
                      </div>
                      {!isSelf && (
                        <button
                          type="button"
                          onClick={() => (isSelected ? handleRemoveParent(cal.id) : handleAddParent(cal.id))}
                          style={{
                            padding: '0.25rem 0.5rem',
                            fontSize: '0.8125rem',
                            border: '1px solid var(--ifm-toc-border-color)',
                            borderRadius: '4px',
                            background: isSelected ? 'var(--ifm-color-danger-lightest)' : 'var(--ifm-background-color)',
                            color: isSelected ? 'var(--ifm-color-danger)' : 'inherit',
                            cursor: 'pointer',
                          }}
                        >
                          {isSelected ? 'Remove' : 'Add'}
                        </button>
                      )}
                    </div>
                  );
                })
              )}
            </div>
          ) : (
            // Grouped view
            <div>
              {Object.entries(groupedCalendars).map(([type, calendars]) => {
                if (calendars.length === 0) return null;
                return (
                  <details key={type} open={type === 'GENERAL_ROMAN' || type === 'REGION' || type === 'COUNTRY'}>
                    <summary style={{ cursor: 'pointer', padding: '0.5rem 0', fontWeight: 500 }}>
                      📁 {getCalendarTypeLabel(type as any)} ({calendars.length})
                    </summary>
                    <div style={{ paddingLeft: '1rem' }}>
                      {calendars.map((cal) => {
                        const isSelected = calendar.parent_calendar_ids.includes(cal.id);
                        const isSelf = cal.id === calendar.id;
                        return (
                          <div
                            key={cal.id}
                            style={{
                              display: 'flex',
                              alignItems: 'center',
                              justifyContent: 'space-between',
                              padding: '0.375rem 0',
                              opacity: isSelf ? 0.5 : 1,
                            }}
                          >
                            <span>
                              {isSelected && '✓ '}
                              {cal.name}
                            </span>
                            {!isSelf && (
                              <button
                                type="button"
                                onClick={() => (isSelected ? handleRemoveParent(cal.id) : handleAddParent(cal.id))}
                                style={{
                                  padding: '0.125rem 0.375rem',
                                  fontSize: '0.75rem',
                                  border: 'none',
                                  background: isSelected
                                    ? 'var(--ifm-color-danger-lightest)'
                                    : 'var(--ifm-color-primary-lightest)',
                                  color: isSelected ? 'var(--ifm-color-danger)' : 'var(--ifm-color-primary)',
                                  borderRadius: '4px',
                                  cursor: 'pointer',
                                }}
                              >
                                {isSelected ? '− Remove' : '+ Add'}
                              </button>
                            )}
                          </div>
                        );
                      })}
                    </div>
                  </details>
                );
              })}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
