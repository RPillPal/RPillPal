import { writable, derived } from 'svelte/store';

export const apiData = writable([]);
export const deviceData = writable([]);

export const familyMembers = derived(apiData, ($apiData) => {
  if ($apiData) {
    return $apiData;
  }
  return [];
});

export const deviceList = derived(deviceData, ($deviceData) => {
  if ($deviceData) {
    return $deviceData;
  }
  return [];
});
