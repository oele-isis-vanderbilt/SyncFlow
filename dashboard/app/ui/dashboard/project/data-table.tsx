'use client';

import DataTable, { createTheme } from 'react-data-table-component';
import type { TableColumn } from 'react-data-table-component';

import { useEffect, useState } from 'react';

const LS_THEME_MODE = 'flowbite-theme-mode';

createTheme(
  'dark',
  {
    text: {
      primary: '#FFFFFF',
      secondary: '#FFFFFF',
    },
    background: {
      default: '#1f2937',
    },
    context: {
      background: '#cb4b16',
      text: '#FFFFFF',
    },
    divider: {
      default: '#CECECE',
    },
    action: {
      button: 'rgba(0,0,0,.54)',
      hover: 'rgba(0,0,0,.08)',
      disabled: 'rgba(0,0,0,.12)',
    },
  },
  'dark',
);

createTheme(
  'light',
  {
    text: {
      primary: '#111111',
      secondary: '#111111',
    },
    background: {
      default: '#FFFFFF',
    },
    context: {
      background: '#cb4b16',
      text: '#FFFFFF',
    },
    divider: {
      default: '#1f2937',
    },
    action: {
      button: 'rgba(0,0,0,.54)',
      hover: 'rgba(0,0,0,.08)',
      disabled: 'rgba(0,0,0,.12)',
    },
  },
  'light',
);

export function CustomDataTable<T>({
  columns,
  data,
  noDataComponent,
}: {
  columns: TableColumn<T>[];
  data: T[];
  noDataComponent?: string;
}) {
  const [theme, setTheme] = useState(
    localStorage.getItem(LS_THEME_MODE) || 'light',
  );

  useEffect(() => {
    document.addEventListener('flowbite-theme-mode-sync', (e) => {
      setTheme(e.detail || 'light');
    });
  }, []);

  return (
    <div className="data-table-container h-full w-full">
      <DataTable
        pagination
        responsive
        columns={columns}
        data={data}
        theme={theme}
        noDataComponent={noDataComponent || 'No data to display'}
      />
    </div>
  );
}
