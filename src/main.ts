import App from './App.svelte';

const app = new App({
	target: document.body,
	props: {
		content: ''
	}
});

export default app;