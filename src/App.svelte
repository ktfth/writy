<script lang="ts">
	import CodeMirror from 'svelte-codemirror-editor';
	import { javascript } from '@codemirror/lang-javascript';
	import { oneDark } from '@codemirror/theme-one-dark';

	export let content: string;

	function handleShortcut(event) {
		let key = event.key;
		let keyCode = event.keyCode;
		console.log(key, keyCode);
	}
</script>

<svelte:window on:keypress={handleShortcut} />

<main>
	<CodeMirror bind:content lang={javascript()} theme={oneDark}></CodeMirror>
</main>

<style>
	:global(body) {
		background-color: #2c313a !important;
		margin: 0;
		padding: 0 !important;
	}

	main {
		background-color: #2c313a !important;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>