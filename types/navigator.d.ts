export type {};

declare global {
	interface Navigator {
		readonly userAgent: string;
	}

	var navigator: Navigator;
}
