import type { ReactNode } from 'react';
import { useEffect, useState, useMemo } from 'react';
import clsx from 'clsx';
import { useEditorStore, selectDayCount, selectIsFileSystemSupported } from './context/useEditorStore';
import MetadataPanel from './panels/MetadataPanel';
import ParentCalendarPanel from './panels/ParentCalendarPanel';
import DayDefinitionsPanel from './panels/DayDefinitionsPanel';
import DayEditorPanel from './panels/DayEditorPanel';
import ResourcesPanel from './panels/ResourcesPanel';
import EntityEditorPanel from './panels/EntityEditorPanel';
import CalendarPreview from './preview/CalendarPreview';
import ExportPanel from './preview/ExportPanel';
import {
  loadAvailableCalendars,
  loadAvailableEntities,
  scanRepositoryCalendars,
  loadCalendarFromFileSystem,
  groupCalendarsByType,
  getCalendarTypeLabel,
} from './utils/dataLoader';
import type { EditorCalendarInfo } from './types';
import styles from './CalendarEditor.module.css';

export default function CalendarEditor(): ReactNode {
  const {
    calendar,
    activePanel,
    directoryHandle,
    isConnected,
    repoPath,
    isDirty,
    isLoading,
    isPreviewCollapsed,
    setActivePanel,
    setAvailableCalendars,
    setAvailableEntities,
    setDirectoryHandle,
    setRepoPath,
    setCalendar,
    setIsLoading,
    createNewCalendar,
    togglePreviewCollapsed,
  } = useEditorStore();

  const dayCount = useEditorStore(selectDayCount);
  const isFileSystemSupported = selectIsFileSystemSupported();

  // State for calendar picker
  const [showCalendarPicker, setShowCalendarPicker] = useState(false);
  const [repoCalendars, setRepoCalendars] = useState<EditorCalendarInfo[]>([]);
  const [searchQuery, setSearchQuery] = useState('');
  const [scanError, setScanError] = useState<string | null>(null);

  // Load available calendars and entities on mount
  useEffect(() => {
    const loadData = async () => {
      const calendars = await loadAvailableCalendars();
      setAvailableCalendars(calendars);

      const entities = await loadAvailableEntities();
      setAvailableEntities(entities);
    };
    loadData();
  }, [setAvailableCalendars, setAvailableEntities]);

  const handleConnectRepo = async () => {
    if (!isFileSystemSupported) return;

    try {
      const handle = await window.showDirectoryPicker({
        mode: 'readwrite',
      });

      setIsLoading(true);
      setScanError(null);

      // Scan the repository for calendars
      const calendars = await scanRepositoryCalendars(handle);

      if (calendars.length === 0) {
        setScanError(
          'No calendars found in this repository. Make sure you selected the romcal root folder containing data/definitions/.'
        );
        setIsLoading(false);
        return;
      }

      setDirectoryHandle(handle);
      setRepoPath(handle.name);
      setRepoCalendars(calendars);
      setShowCalendarPicker(true);
      setIsLoading(false);
    } catch (err) {
      // User cancelled or error
      console.error('Failed to connect to repository:', err);
      setScanError(err instanceof Error ? err.message : 'Failed to connect');
      setIsLoading(false);
    }
  };

  const handleDisconnectRepo = () => {
    setDirectoryHandle(null);
    setRepoPath(null);
    setRepoCalendars([]);
    setShowCalendarPicker(false);
  };

  const handleSelectCalendar = async (calendarInfo: EditorCalendarInfo) => {
    if (!directoryHandle) return;

    setIsLoading(true);
    try {
      const calendarData = await loadCalendarFromFileSystem(directoryHandle, calendarInfo.path);
      if (calendarData) {
        setCalendar(calendarData);
        setShowCalendarPicker(false);
        setActivePanel({ type: 'metadata' });
      }
    } catch (err) {
      console.error('Failed to load calendar:', err);
    } finally {
      setIsLoading(false);
    }
  };

  const handleCreateNewInRepo = () => {
    createNewCalendar();
    setShowCalendarPicker(false);
  };

  // Filter and group calendars for picker
  const filteredRepoCalendars = useMemo(() => {
    if (!searchQuery) return repoCalendars;
    const query = searchQuery.toLowerCase();
    return repoCalendars.filter((c) => c.id.toLowerCase().includes(query) || c.name.toLowerCase().includes(query));
  }, [repoCalendars, searchQuery]);

  const groupedRepoCalendars = useMemo(() => {
    return groupCalendarsByType(filteredRepoCalendars);
  }, [filteredRepoCalendars]);

  const renderPanel = () => {
    switch (activePanel.type) {
      case 'metadata':
        return <MetadataPanel />;
      case 'parents':
        return <ParentCalendarPanel />;
      case 'config':
        return <MetadataPanel showConfig />;
      case 'days':
        return <DayDefinitionsPanel />;
      case 'day':
        return <DayEditorPanel dayId={activePanel.id!} />;
      case 'entities':
        return <ResourcesPanel locale={activePanel.locale} />;
      case 'entity':
        return <EntityEditorPanel entityId={activePanel.id!} locale={activePanel.locale!} />;
      default:
        return <MetadataPanel />;
    }
  };

  // Calendar picker modal
  const renderCalendarPicker = () => {
    if (!showCalendarPicker) return null;

    return (
      <div className={styles.modalOverlay}>
        <div className={styles.modal}>
          <div className={styles.modalHeader}>
            <h2>Select a Calendar to Edit</h2>
            <button className={styles.modalClose} onClick={() => setShowCalendarPicker(false)} title="Close">
              &times;
            </button>
          </div>

          <div className={styles.modalBody}>
            <div style={{ marginBottom: '1rem' }}>
              <input
                type="text"
                placeholder="Search calendars..."
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                style={{
                  width: '100%',
                  padding: '0.75rem',
                  border: '1px solid var(--ifm-toc-border-color)',
                  borderRadius: '6px',
                  fontSize: '1rem',
                  background: 'var(--ifm-background-color)',
                }}
              />
            </div>

            <div style={{ maxHeight: '400px', overflowY: 'auto' }}>
              {Object.entries(groupedRepoCalendars).map(([type, calendars]) => {
                if (calendars.length === 0) return null;
                return (
                  <div key={type} style={{ marginBottom: '1.5rem' }}>
                    <h4
                      style={{
                        fontSize: '0.75rem',
                        fontWeight: 600,
                        color: 'var(--ifm-color-content-secondary)',
                        textTransform: 'uppercase',
                        marginBottom: '0.5rem',
                      }}
                    >
                      {getCalendarTypeLabel(type as keyof typeof groupedRepoCalendars)} ({calendars.length})
                    </h4>
                    <div style={{ display: 'flex', flexDirection: 'column', gap: '0.25rem' }}>
                      {calendars.map((cal) => (
                        <button
                          key={cal.id}
                          onClick={() => handleSelectCalendar(cal)}
                          style={{
                            display: 'flex',
                            alignItems: 'center',
                            justifyContent: 'space-between',
                            padding: '0.625rem 0.75rem',
                            background: 'var(--ifm-background-color)',
                            border: '1px solid var(--ifm-toc-border-color)',
                            borderRadius: '6px',
                            cursor: 'pointer',
                            textAlign: 'left',
                            transition: 'background 0.15s',
                          }}
                          onMouseOver={(e) => {
                            e.currentTarget.style.background = 'var(--ifm-hover-overlay)';
                          }}
                          onMouseOut={(e) => {
                            e.currentTarget.style.background = 'var(--ifm-background-color)';
                          }}
                        >
                          <span style={{ fontWeight: 500 }}>{cal.name}</span>
                          <span
                            style={{
                              fontSize: '0.75rem',
                              fontFamily: 'monospace',
                              color: 'var(--ifm-color-content-secondary)',
                            }}
                          >
                            {cal.id}
                          </span>
                        </button>
                      ))}
                    </div>
                  </div>
                );
              })}
            </div>

            {filteredRepoCalendars.length === 0 && (
              <div style={{ textAlign: 'center', padding: '2rem', color: 'var(--ifm-color-content-secondary)' }}>
                No calendars found matching "{searchQuery}"
              </div>
            )}
          </div>

          <div className={styles.modalFooter}>
            <button className={clsx(styles.button, styles.buttonPrimary)} onClick={handleCreateNewInRepo}>
              <span role="img" aria-label="new">
                ✨
              </span>
              Create New Calendar
            </button>
            <button
              className={clsx(styles.button, styles.buttonSecondary)}
              onClick={() => setShowCalendarPicker(false)}
            >
              Cancel
            </button>
          </div>
        </div>
      </div>
    );
  };

  // Welcome screen when no calendar is loaded
  if (!calendar.id && !isDirty) {
    return (
      <div className={styles.editorContainer}>
        <div className={styles.header}>
          <div className={styles.headerLeft}>
            <h1 className={styles.headerTitle}>
              <span role="img" aria-label="calendar">
                📅
              </span>
              Romcal Editor
            </h1>
          </div>
          <div className={styles.headerRight}>
            {isFileSystemSupported && (
              <button className={clsx(styles.button, styles.buttonSecondary)} onClick={handleConnectRepo}>
                <span role="img" aria-label="folder">
                  📁
                </span>
                Connect Repository
              </button>
            )}
          </div>
        </div>
        <div className={styles.welcomeScreen}>
          <div className={styles.welcomeIcon}>🗓️</div>
          <h2 className={styles.welcomeTitle}>Welcome to Romcal Editor</h2>
          <p className={styles.welcomeDescription}>
            Create and edit liturgical calendar definitions for the Romcal project. Define days, entities, and
            translations for your local calendar.
          </p>

          {scanError && (
            <div
              style={{
                padding: '0.75rem 1rem',
                background: 'rgba(239, 68, 68, 0.1)',
                color: 'rgb(220, 38, 38)',
                borderRadius: '6px',
                marginBottom: '1rem',
                maxWidth: '500px',
              }}
            >
              {scanError}
            </div>
          )}

          <div className={styles.welcomeActions}>
            <button className={clsx(styles.button, styles.buttonPrimary)} onClick={createNewCalendar}>
              <span role="img" aria-label="new">
                ✨
              </span>
              New Calendar
            </button>
            {isFileSystemSupported && (
              <>
                <div className={styles.welcomeDivider}>or</div>
                <button className={clsx(styles.button, styles.buttonSecondary)} onClick={handleConnectRepo}>
                  <span role="img" aria-label="folder">
                    📁
                  </span>
                  Load from Repository
                </button>
              </>
            )}
          </div>
        </div>

        {renderCalendarPicker()}

        {/* Loading Overlay */}
        {isLoading && (
          <div className={styles.loadingOverlay}>
            <div className={styles.loadingSpinner} />
          </div>
        )}
      </div>
    );
  }

  return (
    <div className={styles.editorContainer}>
      {/* Header */}
      <header className={styles.header}>
        <div className={styles.headerLeft}>
          <h1 className={styles.headerTitle}>
            <span role="img" aria-label="calendar">
              📅
            </span>
            Romcal Editor
            {isDirty && <span className={styles.dirtyIndicator} title="Unsaved changes" />}
          </h1>
          {calendar.id && (
            <span style={{ color: 'var(--ifm-color-content-secondary)', fontSize: '0.875rem' }}>— {calendar.id}</span>
          )}
        </div>
        <div className={styles.headerRight}>
          {isFileSystemSupported &&
            (isConnected ? (
              <div className={clsx(styles.connectionStatus, styles.connectionStatusConnected)}>
                <span role="img" aria-label="connected">
                  ✓
                </span>
                {repoPath || 'Connected'}
                <button
                  className={clsx(styles.button, styles.buttonSmall, styles.buttonSecondary)}
                  onClick={() => setShowCalendarPicker(true)}
                  style={{ marginLeft: '0.5rem' }}
                >
                  Change
                </button>
                <button
                  className={clsx(styles.button, styles.buttonSmall, styles.buttonSecondary)}
                  onClick={handleDisconnectRepo}
                >
                  Disconnect
                </button>
              </div>
            ) : (
              <button className={clsx(styles.button, styles.buttonSecondary)} onClick={handleConnectRepo}>
                <span role="img" aria-label="folder">
                  📁
                </span>
                Connect Repository
              </button>
            ))}
          <ExportPanel />
        </div>
      </header>

      {/* Main Content */}
      <div className={styles.mainContent}>
        {/* Sidebar Navigation */}
        <aside className={styles.sidebar}>
          <nav className={styles.sidebarNav}>
            {/* Calendar Section */}
            <div className={styles.navSection}>
              <div className={styles.navSectionTitle}>
                <span role="img" aria-label="calendar">
                  📋
                </span>
                Calendar
              </div>
              <button
                className={clsx(styles.navItem, activePanel.type === 'metadata' && styles.navItemActive)}
                onClick={() => setActivePanel({ type: 'metadata' })}
              >
                Metadata
              </button>
              <button
                className={clsx(styles.navItem, activePanel.type === 'parents' && styles.navItemActive)}
                onClick={() => setActivePanel({ type: 'parents' })}
              >
                Parent Calendars
              </button>
              <button
                className={clsx(styles.navItem, activePanel.type === 'config' && styles.navItemActive)}
                onClick={() => setActivePanel({ type: 'config' })}
              >
                Configuration
              </button>
              <button
                className={clsx(styles.navItem, activePanel.type === 'days' && styles.navItemActive)}
                onClick={() => setActivePanel({ type: 'days' })}
              >
                Day Definitions
                <span className={styles.navItemCount}>{dayCount}</span>
              </button>
            </div>

            {/* Entities Section */}
            <div className={styles.navSection}>
              <div className={styles.navSectionTitle}>
                <span role="img" aria-label="person">
                  👤
                </span>
                Entities
              </div>
              <button
                className={clsx(
                  styles.navItem,
                  activePanel.type === 'entities' && !activePanel.locale && styles.navItemActive
                )}
                onClick={() => setActivePanel({ type: 'entities' })}
              >
                All Entities
              </button>
            </div>
          </nav>

          {/* Sidebar Actions */}
          <div className={styles.sidebarActions}>
            <button
              className={clsx(styles.button, styles.buttonSecondary, styles.buttonSmall)}
              onClick={() => setActivePanel({ type: 'days' })}
            >
              <span role="img" aria-label="add">
                +
              </span>
              New Day Definition
            </button>
            <button
              className={clsx(styles.button, styles.buttonSecondary, styles.buttonSmall)}
              onClick={() => setActivePanel({ type: 'entities' })}
            >
              <span role="img" aria-label="add">
                +
              </span>
              New Entity
            </button>
          </div>
        </aside>

        {/* Editor Panel */}
        <div className={styles.editorPanel}>
          <div className={styles.panelContent}>{renderPanel()}</div>

          {/* Preview Panel */}
          <div className={clsx(styles.previewPanel, isPreviewCollapsed && styles.previewCollapsed)}>
            <div className={styles.previewHeader} onClick={togglePreviewCollapsed}>
              <h3 className={styles.previewTitle}>
                <span role="img" aria-label="preview">
                  👁️
                </span>
                Calendar Preview
              </h3>
              <div className={styles.previewControls}>
                <span>{isPreviewCollapsed ? '▲ Expand' : '▼ Collapse'}</span>
              </div>
            </div>
            {!isPreviewCollapsed && (
              <div className={styles.previewContent}>
                <CalendarPreview />
              </div>
            )}
          </div>
        </div>
      </div>

      {/* Calendar Picker Modal */}
      {renderCalendarPicker()}

      {/* Loading Overlay */}
      {isLoading && (
        <div className={styles.loadingOverlay}>
          <div className={styles.loadingSpinner} />
        </div>
      )}
    </div>
  );
}
