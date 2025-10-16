import { useEffect } from "react";
import { setOutsideContent } from "@/utils/clickThrough";

const isElementVisible = (element: Element): boolean => {
	const tagName = element.tagName.toLowerCase();

	if (tagName === "html" || tagName === "body") {
		return false;
	}

	const computed = window.getComputedStyle(element);

	const bgColor = computed.backgroundColor;
	const hasVisibleBackground =
		bgColor && bgColor !== "rgba(0, 0, 0, 0)" && bgColor !== "transparent";

	const hasBorder =
		(computed.borderTopWidth && computed.borderTopWidth !== "0px") ||
		(computed.borderRightWidth && computed.borderRightWidth !== "0px") ||
		(computed.borderBottomWidth && computed.borderBottomWidth !== "0px") ||
		(computed.borderLeftWidth && computed.borderLeftWidth !== "0px");

	const hasBackgroundImage =
		computed.backgroundImage && computed.backgroundImage !== "none";

	const hasBoxShadow = computed.boxShadow && computed.boxShadow !== "none";

	const isInteractive = ["button", "input", "select", "textarea", "a"].includes(
		tagName,
	);

	const hasInteractiveRole =
		element.hasAttribute("role") || element.hasAttribute("aria-label");

	return (
		hasVisibleBackground ||
		hasBorder ||
		hasBackgroundImage ||
		hasBoxShadow ||
		isInteractive ||
		hasInteractiveRole
	);
};

export const useClickThrough = () => {
	useEffect(() => {
		let lastOverContent = false;

		const handleMouseMove = (e: MouseEvent) => {
			const elementsAtPoint = document.elementsFromPoint(e.clientX, e.clientY);

			const isOverContent = elementsAtPoint.some((element) =>
				isElementVisible(element),
			);

			if (isOverContent !== lastOverContent) {
				lastOverContent = isOverContent;
				setOutsideContent(!isOverContent);
			}
		};

		document.addEventListener("mousemove", handleMouseMove);

		setOutsideContent(true);

		return () => {
			document.removeEventListener("mousemove", handleMouseMove);
		};
	}, []);
};
