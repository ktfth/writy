<script lang="ts">
	import CodeMirror from 'svelte-codemirror-editor';
	import { javascript } from '@codemirror/lang-javascript';
	import { oneDark } from '@codemirror/theme-one-dark';

	import { open, save } from '@tauri-apps/api/dialog';
	import { readTextFile, writeTextFile, BaseDirectory } from '@tauri-apps/api/fs';

	interface TabFile {
		path: string;
		content: string;
		index: number;
	}

	export let value: string;
	export let latestFilePath: string;

	export let tabs: Array<TabFile>;
	export let tabIndex: number;

	async function handleShortcut(event) {
		let keyCode = event.keyCode;
		if (event.ctrlKey) {
			// save
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
					tabs[tabIndex].path = filePath;
					tabs[tabIndex].content = value;
				}
			}
			// save as
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
					tabs[tabIndex].path = filePath;
					tabs[tabIndex].content = value;
				}
			}
			// open file
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
					tabs[tabIndex].path = latestFilePath;
					tabs[tabIndex].content = value;
				}
			}
			// new file
			if (keyCode === 78) {
				event.preventDefault();
				tabIndex = tabIndex + 1;
				tabs.push({
					path: '',
					content: '',
					index: tabIndex,
				});
				value = '';
				latestFilePath = '';
				tabs = tabs;
			}
			// close file
			if (keyCode === 87) {
				event.preventDefault();
				if (tabIndex >= 0) {
					tabIndex = tabIndex - 1;
					tabs.pop();
					if (tabIndex > -1) {
						value = tabs[tabs.length - 1].content;
						latestFilePath = tabs[tabs.length - 1].path;
					} 
					tabs = tabs;
				}
			}
		}
	}

	document.addEventListener('keydown', handleShortcut);

	function handleValueChange() {
		tabs[tabIndex].content = value;
	}

	function handleSelectTab(event, index) {
		event.preventDefault();
		tabIndex = index;
		value = tabs[index].content;
		latestFilePath = tabs[index].path;
		tabs = tabs;
	}
</script>

<main>
	<div class="main-tabs">
		{#each tabs as tab (tab.index)}
			<a href="#{tab}" class="tab" on:click={(event) => handleSelectTab(event, tab.index)}>{tab.path.split('\\')[tab.path.split('\\').length - 1] || 'Untitled *'}</a>
		{/each}
	</div>

	{#if tabs.length}
		<CodeMirror bind:value lang={javascript()} theme={oneDark} on:change={handleValueChange}></CodeMirror>
	{/if}
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

	.main-tabs {
		padding: 0.5em;
	}

	.main-tabs a {
		color: lightgray;
		font-size: 12px;
		border-right: 1px solid lightgray;
		padding: 0 0.5em;
	}

	.main-tabs a:last-child {
		border-right: none;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>