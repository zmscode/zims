import { useState, useEffect, useCallback } from "react";
import {
	CommandDialog,
	CommandEmpty,
	CommandGroup,
	CommandInput,
	CommandItem,
	CommandList,
	CommandSeparator,
	CommandShortcut,
} from "@/components/shadcn/command";
import { listen } from "@tauri-apps/api/event";
import type { Command } from "@/types/commands";
import { useCommands } from "@/hooks/useCommands";

export const CommandPalette = () => {
	const [open, setOpen] = useState(false);
	const commands = useCommands();

	useEffect(() => {
		const down = (e: KeyboardEvent) => {
			if (e.key === "k" && (e.metaKey || e.ctrlKey)) {
				e.preventDefault();
				setOpen((open) => !open);
			}
		};

		document.addEventListener("keydown", down);
		return () => document.removeEventListener("keydown", down);
	}, []);

	useEffect(() => {
		const unlisten = listen("toggle-command-palette", () => {
			setOpen((open) => !open);
		});

		return () => {
			unlisten.then((fn) => fn());
		};
	}, []);

	const runCommand = useCallback((command: () => void | Promise<void>) => {
		setOpen(false);
		command();
	}, []);

	const groupedCommands = commands.reduce(
		(acc, command) => {
			if (!acc[command.category]) {
				acc[command.category] = [];
			}
			acc[command.category].push(command);
			return acc;
		},
		{} as Record<string, Command[]>,
	);

	return (
		<CommandDialog
			open={open}
			onOpenChange={setOpen}
			title="Command Palette"
			description="Type a command or search..."
			showOverlay={false}
		>
			<CommandInput placeholder="Type a command or search..." />
			<CommandList>
				<CommandEmpty>No results found.</CommandEmpty>
				{Object.entries(groupedCommands).map(
					([category, categoryCommands], index) => (
						<div key={category}>
							{index > 0 && <CommandSeparator />}
							<CommandGroup heading={category}>
								{categoryCommands.map((command) => (
									<CommandItem
										key={command.id}
										value={command.label}
										onSelect={() => runCommand(command.action)}
									>
										{command.icon}
										<span>{command.label}</span>
										{command.shortcut && (
											<CommandShortcut>{command.shortcut}</CommandShortcut>
										)}
									</CommandItem>
								))}
							</CommandGroup>
						</div>
					),
				)}
			</CommandList>
		</CommandDialog>
	);
};
