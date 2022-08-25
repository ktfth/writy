<script lang="ts">
	import CodeMirror from 'svelte-codemirror-editor';
	import { javascript } from '@codemirror/lang-javascript';
	import { oneDark } from '@codemirror/theme-one-dark';

	import { emit } from '@tauri-apps/api/event';
	import { open, save } from '@tauri-apps/api/dialog';
	import { readTextFile, writeTextFile, BaseDirectory } from '@tauri-apps/api/fs';

	export let value: string;

	async function handleShortcut(event) {
		let keyCode = event.keyCode;
		if (event.ctrlKey) {
			if (keyCode === 83) {
				event.preventDefault();
				const filePath = await save({
					filters: [{
						name: 'JavaScript',
						extensions: ['js', 'jsx', 'ts', 'tsx'],
					}],
				});
				if (!Array.isArray(filePath)) {
					await writeTextFile(filePath, value);
				}
			}
			if (keyCode === 79) {
				event.preventDefault();
				const selected = await open({
					multiple: false,
					filters: [{
						name: 'JavaScript',
						extensions: ['js', 'jsx', 'ts', 'tsx'],
					}]
				});
				if (!Array.isArray(selected)) {
					value = await readTextFile(selected, { dir: BaseDirectory.App });
				}
			}
		}
	}

	document.addEventListener('keydown', handleShortcut);

	function handleValueChange() {
		emit('value-change', value);
	}
</script>

<!-- <svelte:window on:keypress={handleShortcut} /> -->

<main>
	<CodeMirror bind:value lang={javascript()} theme={oneDark} on:change={handleValueChange}></CodeMirror>
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