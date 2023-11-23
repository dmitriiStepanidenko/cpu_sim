import { writable, get } from 'svelte/store';
import type { Writable } from 'svelte/store';


const themeStore = (): Writable<string> => {
  const defaultTheme = "grey";
  const themeKey = "theme";
  const localStorageThemeValue = localStorage && localStorage[themeKey] ? localStorage[themeKey] : undefined;
  const startingTheme = localStorageThemeValue ? localStorageThemeValue : defaultTheme;
  const store = writable(startingTheme);
  const { subscribe, set } = store;

  return {
    subscribe,
    set: (n: string) => {
      if (localStorage != undefined) {
        localStorage[themeKey] = n;
      }
      store.set(n);
    },
    update: (cb) => {
      const updatedStore = cb(get(store));

      set(updatedStore);
    }
  }
}
export const theme = themeStore();
