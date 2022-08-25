import App from './App.svelte';

interface IProps {
	value: string;
}

interface IApp {
	target: HTMLElement;
	props: IProps;
}

const appConfig: IApp = {
	target: document.body,
	props: {
		value: ''
	}
};

const app = new App(appConfig);

export default app;