import type { ReactNode } from 'react';
import { useState } from 'react';
import { useEditorStore, selectIsFileSystemSupported } from '../context/useEditorStore';
import { validateCalendar } from '../utils/validation';
import {
  downloadCalendar,
  downloadEntities,
  saveCalendarToFileSystem,
  saveEntitiesToFileSystem,
  getCalendarFilePath,
  getModifiedFiles,
} from '../utils/jsonExport';
import styles from '../shared/shared.module.css';
import clsx from 'clsx';

export default function ExportPanel(): ReactNode {
  const {
    calendar,
    entities,
    directoryHandle,
    isConnected,
    isDirty,
    setValidationErrors,
    markAsSaved,
    setIsLoading,
  } = useEditorStore();

  const [showPanel, setShowPanel] = useState(false);
  const [saveStatus, setSaveStatus] = useState<'idle' | 'saving' | 'success' | 'error'>('idle');
  const [errorMessage, setErrorMessage] = useState('');

  const isFileSystemSupported = selectIsFileSystemSupported();
  const validation = validateCalendar(calendar);
  const hasErrors = !validation.valid;
  const modifiedFiles = getModifiedFiles(calendar, entities);

  const handleSave = async () => {
    if (!calendar.id) {
      setErrorMessage('Calendar ID is required');
      setSaveStatus('error');
      return;
    }

    if (!validation.valid) {
      setValidationErrors(
        validation.errors.reduce((acc, err) => {
          if (!acc[err.path]) acc[err.path] = [];
          acc[err.path].push(err.message);
          return acc;
        }, {} as Record<string, string[]>)
      );
      setErrorMessage('Please fix validation errors before saving');
      setSaveStatus('error');
      return;
    }

    setIsLoading(true);
    setSaveStatus('saving');

    try {
      if (isConnected && directoryHandle) {
        // Save to file system
        const calendarPath = getCalendarFilePath(calendar.id, calendar.metadata.type);
        const calendarSaved = await saveCalendarToFileSystem(directoryHandle, calendar, calendarPath);

        if (!calendarSaved) {
          throw new Error('Failed to save calendar file');
        }

        // Save entities
        for (const [locale, localeEntities] of Object.entries(entities)) {
          if (Object.keys(localeEntities).length > 0) {
            const entitiesSaved = await saveEntitiesToFileSystem(directoryHandle, locale, localeEntities);
            if (!entitiesSaved) {
              throw new Error(`Failed to save entities for locale ${locale}`);
            }
          }
        }

        markAsSaved();
        setSaveStatus('success');
        setErrorMessage('');
      } else {
        // Download files
        downloadCalendar(calendar);

        for (const [locale, localeEntities] of Object.entries(entities)) {
          if (Object.keys(localeEntities).length > 0) {
            downloadEntities(locale, localeEntities);
          }
        }

        markAsSaved();
        setSaveStatus('success');
        setErrorMessage('');
      }
    } catch (error) {
      console.error('Save failed:', error);
      setSaveStatus('error');
      setErrorMessage(error instanceof Error ? error.message : 'Unknown error occurred');
    } finally {
      setIsLoading(false);
    }
  };

  const getButtonLabel = () => {
    if (saveStatus === 'saving') return 'Saving...';
    if (isConnected) return 'Save to Repository';
    return 'Download JSON';
  };

  return (
    <div style={{ position: 'relative' }}>
      <button
        type="button"
        onClick={() => setShowPanel(!showPanel)}
        style={{
          display: 'inline-flex',
          alignItems: 'center',
          gap: '0.5rem',
          padding: '0.5rem 1rem',
          fontSize: '0.875rem',
          fontWeight: 500,
          background: hasErrors ? 'var(--ifm-color-warning)' : 'var(--ifm-color-primary)',
          color: 'white',
          border: 'none',
          borderRadius: 'var(--ifm-button-border-radius)',
          cursor: 'pointer',
        }}
      >
        💾 {getButtonLabel()}
        {isDirty && <span style={{ width: '6px', height: '6px', background: 'white', borderRadius: '50%' }} />}
      </button>

      {showPanel && (
        <div
          style={{
            position: 'absolute',
            top: '100%',
            right: 0,
            marginTop: '0.5rem',
            width: '350px',
            background: 'var(--ifm-background-surface-color)',
            border: '1px solid var(--ifm-toc-border-color)',
            borderRadius: 'var(--ifm-card-border-radius)',
            boxShadow: '0 4px 12px rgba(0, 0, 0, 0.15)',
            zIndex: 100,
          }}
        >
          <div style={{ padding: '1rem', borderBottom: '1px solid var(--ifm-toc-border-color)' }}>
            <h3 style={{ margin: 0, fontSize: '1rem', fontWeight: 600 }}>
              {isConnected ? '📁 Save to Repository' : '⬇️ Download Files'}
            </h3>
          </div>

          <div style={{ padding: '1rem' }}>
            {/* Validation Status */}
            <div style={{ marginBottom: '1rem' }}>
              <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem', marginBottom: '0.5rem' }}>
                <span>{validation.valid ? '✅' : '⚠️'}</span>
                <span style={{ fontWeight: 500 }}>
                  {validation.valid ? 'Validation passed' : `${validation.errors.length} issue(s) found`}
                </span>
              </div>
              {!validation.valid && (
                <ul style={{ margin: 0, paddingLeft: '1.5rem', fontSize: '0.8125rem', color: 'var(--ifm-color-warning-darkest)' }}>
                  {validation.errors.slice(0, 3).map((err, i) => (
                    <li key={i}>{err.message}</li>
                  ))}
                  {validation.errors.length > 3 && (
                    <li>...and {validation.errors.length - 3} more</li>
                  )}
                </ul>
              )}
            </div>

            {/* Files to be modified */}
            {modifiedFiles.length > 0 && (
              <div style={{ marginBottom: '1rem' }}>
                <div style={{ fontSize: '0.8125rem', fontWeight: 500, marginBottom: '0.5rem' }}>
                  Files to be {isConnected ? 'saved' : 'downloaded'}:
                </div>
                <ul style={{ margin: 0, paddingLeft: '1.25rem', fontSize: '0.75rem', fontFamily: 'monospace' }}>
                  {modifiedFiles.slice(0, 5).map((file, i) => (
                    <li key={i}>{file.path}</li>
                  ))}
                  {modifiedFiles.length > 5 && (
                    <li>...and {modifiedFiles.length - 5} more</li>
                  )}
                </ul>
              </div>
            )}

            {/* Status messages */}
            {saveStatus === 'success' && (
              <div style={{ padding: '0.5rem 0.75rem', background: 'rgba(34, 197, 94, 0.1)', color: 'rgb(22, 163, 74)', borderRadius: '4px', marginBottom: '1rem', fontSize: '0.875rem' }}>
                ✓ {isConnected ? 'Files saved successfully!' : 'Files downloaded!'}
              </div>
            )}

            {saveStatus === 'error' && errorMessage && (
              <div style={{ padding: '0.5rem 0.75rem', background: 'rgba(239, 68, 68, 0.1)', color: 'rgb(220, 38, 38)', borderRadius: '4px', marginBottom: '1rem', fontSize: '0.875rem' }}>
                ✗ {errorMessage}
              </div>
            )}

            {/* Action buttons */}
            <div style={{ display: 'flex', gap: '0.5rem' }}>
              <button
                type="button"
                onClick={handleSave}
                disabled={saveStatus === 'saving' || !calendar.id}
                style={{
                  flex: 1,
                  padding: '0.625rem 1rem',
                  fontSize: '0.875rem',
                  fontWeight: 500,
                  background: saveStatus === 'saving' ? 'var(--ifm-color-content-secondary)' : 'var(--ifm-color-primary)',
                  color: 'white',
                  border: 'none',
                  borderRadius: 'var(--ifm-button-border-radius)',
                  cursor: saveStatus === 'saving' || !calendar.id ? 'not-allowed' : 'pointer',
                  opacity: saveStatus === 'saving' || !calendar.id ? 0.7 : 1,
                }}
              >
                {saveStatus === 'saving' ? 'Saving...' : isConnected ? 'Save' : 'Download'}
              </button>
              <button
                type="button"
                onClick={() => setShowPanel(false)}
                style={{
                  padding: '0.625rem 1rem',
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

            {/* Help text */}
            {!isConnected && isFileSystemSupported && (
              <p style={{ margin: '1rem 0 0 0', fontSize: '0.75rem', color: 'var(--ifm-color-content-secondary)' }}>
                Tip: Connect a repository to save files directly without downloading.
              </p>
            )}

            {!isFileSystemSupported && (
              <p style={{ margin: '1rem 0 0 0', fontSize: '0.75rem', color: 'var(--ifm-color-content-secondary)' }}>
                Note: Your browser doesn't support direct file saving. Files will be downloaded instead.
              </p>
            )}
          </div>
        </div>
      )}
    </div>
  );
}
