import { writable, derived } from 'svelte/store';

export const apiData = writable([]);

export const familyMembers = derived(apiData, ($apiData) => {
  if ($apiData) {
    return $apiData;
  }
  return [];
});

/*
export const familyMembers = writable([
  { "name": "Abdul", "dosagesLeft": "6", "doseToday": 1 },
  { "name": "Amaan", "dosagesLeft": "23", "doseToday": 0 },
  { "name": "Dan", "dosagesLeft": "18", "doseToday": 0 },
  { "name": "Om", "dosagesLeft": "5", "doseToday": 1 },
]);*/
