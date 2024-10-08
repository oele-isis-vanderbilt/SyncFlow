import type { Project } from '@/types/project';

export function lastUpdatedProjectComparator(a: Project, b: Project) {
  return b.lastUpdated - a.lastUpdated;
}

export function dateFromTimestamp(timestamp: number) {
  return new Date(timestamp * 1000).toLocaleString();
}

export function durationInMinutes(timestamp: number) {
  return Math.round((Date.now() - timestamp) / 60000);
}
