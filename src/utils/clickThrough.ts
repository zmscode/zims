import { invoke } from "@tauri-apps/api/core";

let isClickThroughEnabled = false;
let toggleTimeout: number | null = null;

export async function enableClickThrough() {
	try {
		if (!isClickThroughEnabled) {
			await invoke("enable_clickthrough");
			isClickThroughEnabled = true;
		}
	} catch (error) {
		console.error("Failed to enable click-through:", error);
	}
}

export async function disableClickThrough() {
	try {
		if (isClickThroughEnabled) {
			await invoke("disable_clickthrough");
			isClickThroughEnabled = false;
		}
	} catch (error) {
		console.error("Failed to disable click-through:", error);
	}
}

export function setOutsideContent(value: boolean) {
	if (toggleTimeout !== null) {
		clearTimeout(toggleTimeout);
	}

	toggleTimeout = window.setTimeout(async () => {
		if (value) {
			await enableClickThrough();
		} else {
			await disableClickThrough();
		}
		toggleTimeout = null;
	}, 10);
}

export async function handleMouseEnter() {
	await disableClickThrough();
}

export async function restoreFocus() {
	try {
		await invoke("force_focus");
	} catch (error) {
		console.error("Failed to restore focus:", error);
	}
}
