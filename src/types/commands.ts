export interface Command {
	id: string;
	label: string;
	icon?: React.ReactNode;
	shortcut?: string;
	category: string;
	action: () => void | Promise<void>;
}

export const CommandCategories = {
	WINDOW: "Window",
	CLIPBOARD: "Clipboard",
	APPEARANCE: "Appearance",
	GENERAL: "General",
	NAVIGATION: "Navigation",
	EDITOR: "Editor",
	DEBUG: "Debug",
} as const;

export type CommandCategory =
	(typeof CommandCategories)[keyof typeof CommandCategories];
