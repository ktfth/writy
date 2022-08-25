import App from './App.svelte';

interface IProps {
	value: string;
	latestFilePath: string;
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
	}
};

const app = new App(appConfig);

export default app;