import type { ReactNode } from 'react';
import Layout from '@theme/Layout';
import CalendarEditor from '@site/src/components/CalendarEditor';

export default function EditorPage(): ReactNode {
  return (
    <Layout
      title="Calendar Editor"
      description="Create and edit liturgical calendar definitions for Romcal"
    >
      <CalendarEditor />
    </Layout>
  );
}
