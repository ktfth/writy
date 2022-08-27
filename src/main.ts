import App from './App.svelte';

import type { TabFile } from './tab-file';

interface IProps {
	value: string;
	latestFilePath: string;
	tabs: Array<TabFile>;
	tabIndex: number;
}

interface IApp {
	target: HTMLElement;
	props: IProps;
}

const appConfig: IApp = {
	target: document.body,
	props: {
		value: '',
		latestFilePath: '',
		tabs: [{
			path: '',
			content: '',
			index: 0,
			saved: false,
			initialContent: '',
		}],
		tabIndex: 0,
	}
};

const app = new App(appConfig);

export default app;