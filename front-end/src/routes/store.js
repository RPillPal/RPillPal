import { writable, derived } from 'svelte/store';

export const apiData = writable([]);
/*
export const familyMembers = derived(apiData, ($apiData) => {
  if ($apiData.users) {
    return $apiData.users.map(user => user.name);
  }
  return [];
});
*/
export const familyMembers = writable(["Alice", "Bob", "Om", "Dan"]);
