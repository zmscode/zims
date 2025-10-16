import { useEffect, useCallback } from "react";

type KeyPressCallback = (event: KeyboardEvent) => void;

interface UseKeyPressOptions {
	key: string | string[];
	callback: KeyPressCallback;
	enabled?: boolean;
	preventDefault?: boolean;
}

export const useKeyPress = ({
	key,
	callback,
	enabled = true,
	preventDefault = false,
}: UseKeyPressOptions) => {
	const handleKeyDown = useCallback(
		(event: KeyboardEvent) => {
			const keys = Array.isArray(key) ? key : [key];

			if (keys.includes(event.key)) {
				if (preventDefault) {
					event.preventDefault();
				}
				callback(event);
			}
		},
		[key, callback, preventDefault],
	);

	useEffect(() => {
		if (!enabled) return;

		window.addEventListener("keydown", handleKeyDown);

		return () => {
			window.removeEventListener("keydown", handleKeyDown);
		};
	}, [handleKeyDown, enabled]);
};
