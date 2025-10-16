import type { Command } from "@/types/commands";

/**
 * Hook that returns all available commands for the command palette
 *
 * To add new commands:
 * 1. Add the command object to the commands array below
 * 2. If calling Rust, add the command handler in src-tauri/src/lib.rs
 * 3. If needed, add permissions in src-tauri/capabilities/default.json
 *
 * Example command structure:
 * {
 * 	id: "my-command",
 * 	label: "My Command",
 * 	icon: <MyIcon className="size-4" />,
 * 	shortcut: "⌘K",
 * 	category: CommandCategories.GENERAL,
 * 	action: async () => {
 * 		// Your command logic
 * 		console.log("Running my command");
 * 	},
 * }
 */
export const useCommands = (): Command[] => {
	const commands: Command[] = [
		// Add your custom commands here
	];

	return commands;
};
