import { getCurrentWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";

export const keyboardHandlers = {
	closeWindow: () => {
		getCurrentWindow().close();
	},
	minimizeWindow: () => {
		getCurrentWindow().minimize();
	},
	toggleFullscreen: async () => {
		const window = getCurrentWindow();
		const isFullscreen = await window.isFullscreen();
		window.setFullscreen(!isFullscreen);
	},
	forceFocus: () => {
		invoke("force_focus");
	},
	enableClickThrough: () => {
		invoke("enable_clickthrough");
	},
	disableClickThrough: () => {
		invoke("disable_clickthrough");
	},
} as const;

export const keyboardShortcuts = {
	Escape: keyboardHandlers.closeWindow,
	F11: keyboardHandlers.toggleFullscreen,
} as const;
