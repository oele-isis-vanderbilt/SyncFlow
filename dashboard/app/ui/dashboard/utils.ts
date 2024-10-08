export function getDateFromTimeStamp(timestamp: number) {
  return new Date(timestamp * 1000).toISOString();
}

export function getDateFromTimeStampNs(timestamp: number) {
  return new Date(timestamp / 1000000).toISOString();
}

export function getTimeDifferenceInMinutes(timestamp: number) {
  return Math.round((Date.now() - timestamp * 1000) / 60000);
}

export function partitionArray(arr: any[], size: number) {
  return arr.reduce((acc, _, i) => {
    if (i % size === 0) {
      acc.push(arr.slice(i, i + size));
    }
    return acc;
  }, []);
}
