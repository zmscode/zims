import { MoonIcon, SunIcon, MonitorIcon } from "@phosphor-icons/react";
import { useTheme } from "@/hooks/useTheme";
import { Button } from "@/components/shadcn/button";
import type { ThemeMode } from "@/contexts/ThemeContext";

export const ThemeToggle = () => {
	const { mode, setMode } = useTheme();

	const themes: { mode: ThemeMode; icon: React.ReactNode; label: string }[] = [
		{ mode: "light", icon: <SunIcon />, label: "Light" },
		{ mode: "dark", icon: <MoonIcon />, label: "Dark" },
		{ mode: "system", icon: <MonitorIcon />, label: "System" },
	];

	return (
		<div className="flex items-center gap-1 rounded-lg border bg-card p-1">
			{themes.map(({ mode: themeMode, icon, label }) => (
				<Button
					key={themeMode}
					variant={mode === themeMode ? "default" : "ghost"}
					size="icon-sm"
					onClick={() => setMode(themeMode)}
					title={label}
				>
					{icon}
				</Button>
			))}
		</div>
	);
};
