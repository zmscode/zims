import { useKeyPress } from "@/hooks/useKeyPress";
import { useClickThrough } from "@/hooks/useClickThrough";
import { keyboardShortcuts } from "@/handlers/keyboard";
import { CommandPalette } from "@/components/CommandPalette";

const App = () => {
	useClickThrough();

	useKeyPress({
		key: "Escape",
		callback: keyboardShortcuts.Escape,
	});

	useKeyPress({
		key: "F11",
		callback: keyboardShortcuts.F11,
		preventDefault: true,
	});

	return (
		<>
			<CommandPalette />
		</>
	);
};

export default App;
