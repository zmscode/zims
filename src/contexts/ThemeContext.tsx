import { createContext, useEffect, useState } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { setTheme as setTauriTheme } from "@tauri-apps/api/app";

export type ThemeMode = "light" | "dark" | "system";
type Theme = "light" | "dark";

interface ThemeContextType {
	mode: ThemeMode;
	theme: Theme;
	setMode: (mode: ThemeMode) => void;
}

export const ThemeContext = createContext<ThemeContextType | undefined>(
	undefined,
);

const THEME_STORAGE_KEY = "theme-mode";

export const ThemeProvider = ({ children }: { children: React.ReactNode }) => {
	const [mode, setModeState] = useState<ThemeMode>("dark");

	const [systemTheme, setSystemTheme] = useState<Theme>("dark");
	const [theme, setThemeState] = useState<Theme>("dark");

	useEffect(() => {
		let unlisten: (() => void) | undefined;

		async function initializeTheme() {
			const window = getCurrentWindow();

			const initialTheme = await window.theme();
			setSystemTheme(initialTheme || "dark");

			unlisten = await window.onThemeChanged((event) => {
				setSystemTheme(event.payload);
			});
		}

		initializeTheme();

		return () => {
			if (unlisten) unlisten();
		};
	}, []);

	useEffect(() => {
		let effectiveTheme: Theme;

		if (mode === "system") {
			effectiveTheme = systemTheme;
			setTauriTheme(null);
		} else {
			effectiveTheme = mode;
			setTauriTheme(mode);
		}

		setThemeState(effectiveTheme);

		const root = document.documentElement;
		root.classList.remove("light", "dark");
		root.classList.add(effectiveTheme);
	}, [mode, systemTheme]);

	const setMode = (newMode: ThemeMode) => {
		setModeState(newMode);
		localStorage.setItem(THEME_STORAGE_KEY, newMode);
	};

	return (
		<ThemeContext.Provider value={{ mode, theme, setMode }}>
			{children}
		</ThemeContext.Provider>
	);
};
