import type { ReactNode, InputHTMLAttributes, SelectHTMLAttributes } from 'react';
import styles from './shared.module.css';
import clsx from 'clsx';

interface FormFieldProps {
  label: string;
  required?: boolean;
  hint?: string;
  error?: string;
  children: ReactNode;
}

export function FormField({ label, required, hint, error, children }: FormFieldProps): ReactNode {
  return (
    <div className={styles.formField}>
      <label className={styles.label}>
        {label}
        {required && <span className={styles.required}>*</span>}
      </label>
      {children}
      {hint && !error && <p className={styles.hint}>{hint}</p>}
      {error && <p className={styles.error}>{error}</p>}
    </div>
  );
}

interface TextInputProps extends InputHTMLAttributes<HTMLInputElement> {
  label: string;
  required?: boolean;
  hint?: string;
  error?: string;
}

export function TextInput({
  label,
  required,
  hint,
  error,
  className,
  ...props
}: TextInputProps): ReactNode {
  return (
    <FormField label={label} required={required} hint={hint} error={error}>
      <input
        type="text"
        className={clsx(styles.input, error && styles.inputError, className)}
        {...props}
      />
    </FormField>
  );
}

interface TextAreaProps extends InputHTMLAttributes<HTMLTextAreaElement> {
  label: string;
  required?: boolean;
  hint?: string;
  error?: string;
  rows?: number;
}

export function TextArea({
  label,
  required,
  hint,
  error,
  className,
  rows = 3,
  ...props
}: TextAreaProps): ReactNode {
  return (
    <FormField label={label} required={required} hint={hint} error={error}>
      <textarea
        className={clsx(styles.textarea, error && styles.inputError, className)}
        rows={rows}
        {...props}
      />
    </FormField>
  );
}

interface SelectProps extends SelectHTMLAttributes<HTMLSelectElement> {
  label: string;
  required?: boolean;
  hint?: string;
  error?: string;
  options: { value: string; label: string }[];
}

export function Select({
  label,
  required,
  hint,
  error,
  options,
  className,
  ...props
}: SelectProps): ReactNode {
  return (
    <FormField label={label} required={required} hint={hint} error={error}>
      <select
        className={clsx(styles.select, error && styles.inputError, className)}
        {...props}
      >
        {options.map((option) => (
          <option key={option.value} value={option.value}>
            {option.label}
          </option>
        ))}
      </select>
    </FormField>
  );
}

interface CheckboxProps extends Omit<InputHTMLAttributes<HTMLInputElement>, 'type'> {
  label: string;
  hint?: string;
}

export function Checkbox({ label, hint, className, ...props }: CheckboxProps): ReactNode {
  return (
    <div className={styles.checkboxField}>
      <label className={styles.checkboxLabel}>
        <input
          type="checkbox"
          className={clsx(styles.checkbox, className)}
          {...props}
        />
        <span>{label}</span>
      </label>
      {hint && <p className={styles.hint}>{hint}</p>}
    </div>
  );
}

interface RadioGroupProps {
  label: string;
  name: string;
  value: string;
  options: { value: string; label: string }[];
  onChange: (value: string) => void;
  required?: boolean;
  hint?: string;
}

export function RadioGroup({
  label,
  name,
  value,
  options,
  onChange,
  required,
  hint,
}: RadioGroupProps): ReactNode {
  return (
    <FormField label={label} required={required} hint={hint}>
      <div className={styles.radioGroup}>
        {options.map((option) => (
          <label key={option.value} className={styles.radioLabel}>
            <input
              type="radio"
              name={name}
              value={option.value}
              checked={value === option.value}
              onChange={(e) => onChange(e.target.value)}
              className={styles.radio}
            />
            <span>{option.label}</span>
          </label>
        ))}
      </div>
    </FormField>
  );
}
