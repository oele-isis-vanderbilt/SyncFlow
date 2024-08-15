'use client';

import { DarkThemeToggle, Tooltip } from 'flowbite-react';

export function DarkModeSwitcher() {
  return (
    <Tooltip animation={false} content="Toggle Dark Mode">
      <DarkThemeToggle />
    </Tooltip>
  );
}
