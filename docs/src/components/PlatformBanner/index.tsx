import type { ReactNode } from 'react';
import styles from './styles.module.css';

export default function PlatformBanner(): ReactNode {
  return (
    <section className={styles.platformBanner}>
      <div className="container">
        <p className={styles.availableFor}>Available for</p>
        <div className={styles.platforms}>
          <span>TypeScript</span>
          <span className={styles.separator}>•</span>
          <span>Python</span>
          <span className={styles.separator}>•</span>
          <span>Rust</span>
          <span className={styles.separator}>•</span>
          <span>CLI</span>
        </div>
      </div>
    </section>
  );
}
