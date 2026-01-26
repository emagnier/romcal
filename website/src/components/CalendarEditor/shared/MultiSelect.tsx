import type { ReactNode } from 'react';
import { useState, useRef, useEffect } from 'react';
import styles from './shared.module.css';
import clsx from 'clsx';

interface MultiSelectProps<T extends string> {
  label: string;
  values: T[];
  options: { value: T; label: string }[];
  onChange: (values: T[]) => void;
  required?: boolean;
  hint?: string;
  placeholder?: string;
}

export function MultiSelect<T extends string>({
  label,
  values,
  options,
  onChange,
  required,
  hint,
  placeholder = 'Type to search...',
}: MultiSelectProps<T>): ReactNode {
  const [inputValue, setInputValue] = useState('');
  const [isOpen, setIsOpen] = useState(false);
  const [highlightedIndex, setHighlightedIndex] = useState(0);
  const wrapperRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  const filteredOptions = options.filter(
    (option) => !values.includes(option.value) && option.label.toLowerCase().includes(inputValue.toLowerCase())
  );

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (wrapperRef.current && !wrapperRef.current.contains(event.target as Node)) {
        setIsOpen(false);
      }
    };
    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, []);

  const handleRemove = (value: T) => {
    onChange(values.filter((v) => v !== value));
  };

  const handleSelect = (value: T) => {
    onChange([...values, value]);
    setInputValue('');
    setIsOpen(false);
    setHighlightedIndex(0);
    inputRef.current?.focus();
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Backspace' && !inputValue && values.length > 0) {
      handleRemove(values[values.length - 1]);
    } else if (e.key === 'ArrowDown') {
      e.preventDefault();
      setIsOpen(true);
      setHighlightedIndex((i) => Math.min(i + 1, filteredOptions.length - 1));
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      setHighlightedIndex((i) => Math.max(i - 1, 0));
    } else if (e.key === 'Enter' && isOpen && filteredOptions[highlightedIndex]) {
      e.preventDefault();
      handleSelect(filteredOptions[highlightedIndex].value);
    } else if (e.key === 'Escape') {
      setIsOpen(false);
    }
  };

  const getOptionLabel = (value: T) => {
    return options.find((o) => o.value === value)?.label || value;
  };

  return (
    <div className={styles.formField}>
      <label className={styles.label}>
        {label}
        {required && <span className={styles.required}>*</span>}
      </label>
      <div className={styles.autocompleteWrapper} ref={wrapperRef}>
        <div className={styles.multiSelect} onClick={() => inputRef.current?.focus()}>
          {values.map((value) => (
            <span key={value} className={styles.multiSelectTag}>
              {getOptionLabel(value)}
              <button
                type="button"
                className={styles.multiSelectTagRemove}
                onClick={(e) => {
                  e.stopPropagation();
                  handleRemove(value);
                }}
              >
                ×
              </button>
            </span>
          ))}
          <input
            ref={inputRef}
            type="text"
            value={inputValue}
            onChange={(e) => {
              setInputValue(e.target.value);
              setIsOpen(true);
              setHighlightedIndex(0);
            }}
            onFocus={() => setIsOpen(true)}
            onKeyDown={handleKeyDown}
            placeholder={values.length === 0 ? placeholder : ''}
            className={styles.multiSelectInput}
          />
        </div>
        {isOpen && (
          <div className={styles.autocompleteDropdown}>
            {filteredOptions.length === 0 ? (
              <div className={styles.autocompleteEmpty}>
                {inputValue ? 'No matching options' : 'All options selected'}
              </div>
            ) : (
              filteredOptions.map((option, index) => (
                <div
                  key={option.value}
                  className={clsx(
                    styles.autocompleteItem,
                    index === highlightedIndex && styles.autocompleteItemHighlighted
                  )}
                  onClick={() => handleSelect(option.value)}
                  onMouseEnter={() => setHighlightedIndex(index)}
                >
                  {option.label}
                </div>
              ))
            )}
          </div>
        )}
      </div>
      {hint && <p className={styles.hint}>{hint}</p>}
    </div>
  );
}

interface EntityAutocompleteProps {
  label: string;
  value: string;
  entities: { id: string; fullname?: string; name?: string }[];
  onChange: (value: string) => void;
  required?: boolean;
  hint?: string;
  placeholder?: string;
}

export function EntityAutocomplete({
  label,
  value,
  entities,
  onChange,
  required,
  hint,
  placeholder = 'Search entities...',
}: EntityAutocompleteProps): ReactNode {
  const [inputValue, setInputValue] = useState(value);
  const [isOpen, setIsOpen] = useState(false);
  const [highlightedIndex, setHighlightedIndex] = useState(0);
  const wrapperRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    setInputValue(value);
  }, [value]);

  const filteredEntities = entities
    .filter(
      (entity) =>
        entity.id.toLowerCase().includes(inputValue.toLowerCase()) ||
        entity.fullname?.toLowerCase().includes(inputValue.toLowerCase()) ||
        entity.name?.toLowerCase().includes(inputValue.toLowerCase())
    )
    .slice(0, 20);

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (wrapperRef.current && !wrapperRef.current.contains(event.target as Node)) {
        setIsOpen(false);
      }
    };
    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, []);

  const handleSelect = (id: string) => {
    onChange(id);
    setInputValue(id);
    setIsOpen(false);
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'ArrowDown') {
      e.preventDefault();
      setIsOpen(true);
      setHighlightedIndex((i) => Math.min(i + 1, filteredEntities.length - 1));
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      setHighlightedIndex((i) => Math.max(i - 1, 0));
    } else if (e.key === 'Enter' && isOpen && filteredEntities[highlightedIndex]) {
      e.preventDefault();
      handleSelect(filteredEntities[highlightedIndex].id);
    } else if (e.key === 'Escape') {
      setIsOpen(false);
    }
  };

  return (
    <div className={styles.formField}>
      <label className={styles.label}>
        {label}
        {required && <span className={styles.required}>*</span>}
      </label>
      <div className={styles.autocompleteWrapper} ref={wrapperRef}>
        <input
          type="text"
          value={inputValue}
          onChange={(e) => {
            setInputValue(e.target.value);
            onChange(e.target.value);
            setIsOpen(true);
            setHighlightedIndex(0);
          }}
          onFocus={() => setIsOpen(true)}
          onKeyDown={handleKeyDown}
          placeholder={placeholder}
          className={styles.input}
        />
        {isOpen && inputValue && (
          <div className={styles.autocompleteDropdown}>
            {filteredEntities.length === 0 ? (
              <div className={styles.autocompleteEmpty}>No matching entities</div>
            ) : (
              filteredEntities.map((entity, index) => (
                <div
                  key={entity.id}
                  className={clsx(
                    styles.autocompleteItem,
                    index === highlightedIndex && styles.autocompleteItemHighlighted
                  )}
                  onClick={() => handleSelect(entity.id)}
                  onMouseEnter={() => setHighlightedIndex(index)}
                >
                  <strong>{entity.id}</strong>
                  {entity.fullname && <span style={{ marginLeft: '0.5rem', opacity: 0.7 }}>{entity.fullname}</span>}
                </div>
              ))
            )}
          </div>
        )}
      </div>
      {hint && <p className={styles.hint}>{hint}</p>}
    </div>
  );
}
