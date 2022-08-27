import App from './App.svelte';

interface TabFile {
    path: string;
    content: string;
    index: number;
}

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
		}],
		tabIndex: 0,
	}
};

const app = new App(appConfig);

export default app;