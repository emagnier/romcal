import type { ReactNode } from 'react';
import clsx from 'clsx';
import Link from '@docusaurus/Link';
import Heading from '@theme/Heading';
import styles from './styles.module.css';

import PerpetualCalendarIcon from '@site/static/img/icons/perpetual-calendar.svg';
import GlobalCoverageIcon from '@site/static/img/icons/global-coverage.svg';
import RichMetadataIcon from '@site/static/img/icons/rich-metadata.svg';

type FeatureItem = {
  title: string;
  description: ReactNode;
  Icon: React.ComponentType<React.SVGProps<SVGSVGElement>>;
};

const FeatureList: FeatureItem[] = [
  {
    title: 'Perpetual Calendar',
    Icon: PerpetualCalendarIcon,
    description: (
      <>
        Calculate liturgical dates for any year, following official Church norms (
        <Link to="/docs/reference/girm">GIRM</Link>, <Link to="/docs/reference/gnly">GNLY</Link>,{' '}
        <Link to="/docs/reference/gilh">GILH</Link>). Accurate and canonical data for any application.
      </>
    ),
  },
  {
    title: 'Global Coverage',
    Icon: GlobalCoverageIcon,
    description: (
      <>
        60+ calendars for countries, dioceses, and religious communities. Available in 10+ languages with easy support
        for adding new locales.
      </>
    ),
  },
  {
    title: 'Rich Metadata',
    Icon: RichMetadataIcon,
    description: (
      <>
        Complete martyrology and entity catalog: saints, blesseds, feasts, places, and events with biographical
        information, colors, ranks, and cycles.
      </>
    ),
  },
];

function Feature({ title, Icon, description }: FeatureItem) {
  return (
    <div className={clsx('col col--4')}>
      <div className="text--center">
        <Icon className={styles.featureSvg} />
      </div>
      <div className="text--center padding-horiz--md">
        <Heading as="h3">{title}</Heading>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures(): ReactNode {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
