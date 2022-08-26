<script lang="ts">
	import CodeMirror from 'svelte-codemirror-editor';
	import { javascript } from '@codemirror/lang-javascript';
	import { oneDark } from '@codemirror/theme-one-dark';

	import { appWindow } from '@tauri-apps/api/window';
	import { open, save } from '@tauri-apps/api/dialog';
	import { readTextFile, writeTextFile, BaseDirectory } from '@tauri-apps/api/fs';

	export let value: string;
	export let latestFilePath: string;

	async function handleShortcut(event) {
		let keyCode = event.keyCode;
		if (event.ctrlKey) {
			if (!event.shiftKey && keyCode === 83) {
				event.preventDefault();
				let filePath = latestFilePath;
				if (!latestFilePath) {
					filePath = await save({
						filters: [{
							name: 'JavaScript',
							extensions: ['js', 'jsx', 'ts', 'tsx'],
						}],
					});
				}
				if (!Array.isArray(filePath)) {
					latestFilePath = filePath;
					await writeTextFile(filePath, value);
				}
			}
			if (event.shiftKey && keyCode === 83) {
				event.preventDefault();
				const filePath = await save({
					filters: [{
						name: 'JavaScript',
						extensions: ['js', 'jsx', 'ts', 'tsx'],
					}],
				});
				if (!Array.isArray(filePath)) {
					latestFilePath = filePath;
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
					latestFilePath = selected;
					value = await readTextFile(selected, { dir: BaseDirectory.App });
				}
			}
		}
	}

	document.addEventListener('keydown', handleShortcut);

	function handleValueChange() {
		appWindow.emit('value-change', value);
	}

	appWindow.listen('open-file-path', async (filePath) => {
		value = await readTextFile(filePath.payload as string, { dir: BaseDirectory.App });
	});

	appWindow.listen('save-file-path', async (filePath) => {
		latestFilePath = filePath.payload as string;
		await writeTextFile(filePath.payload as string, value);
	});

	appWindow.listen('save-as-file-path', async (filePath) => {
		await writeTextFile(filePath.payload as string, value);
	});
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