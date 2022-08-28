<script lang="ts">
	import CodeMirror from 'svelte-codemirror-editor';
	import { javascript } from '@codemirror/lang-javascript';
	import { oneDark } from '@codemirror/theme-one-dark';

	import { open, save } from '@tauri-apps/api/dialog';
	import { readTextFile, writeTextFile, BaseDirectory } from '@tauri-apps/api/fs';

	import Terminal from 'svelte-terminal';

	import type { TabFile } from './tab-file';

	export let value: string;
	export let latestFilePath: string;

	export let tabs: Array<TabFile>;
	export let tabIndex: number;

	let showTerminal = false;

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
					tabs[tabIndex].saved = true;
					tabs[tabIndex].initialContent = value;
					tabs = tabs;
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
					tabs[tabIndex].saved = true;
					tabs[tabIndex].initialContent = value;
					tabs = tabs;
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
					tabs[tabIndex].initialContent = value;
					tabs[tabIndex].saved = true;
					tabs = tabs;
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
					saved: false,
					initialContent: '',
				});
				value = '';
				latestFilePath = '';
				tabs = tabs;
			}
			// close file
			if (keyCode === 87) {
				event.preventDefault();
				if (tabIndex >= 0) {
					tabs.splice(tabIndex, 1);
					tabIndex -= 1;
					if (tabIndex > -1) {
						value = tabs[tabs.length - 1]?.content;
						latestFilePath = tabs[tabs.length - 1]?.path;
					} 
					tabs = tabs;
				}
			}
			// open terminal
			if (keyCode === 192) {
				event.preventDefault();
				showTerminal = !showTerminal;
			}
		}
	}

	document.addEventListener('keydown', handleShortcut);

	function handleValueChange() {
		tabs[tabIndex].saved = tabs[tabIndex].initialContent === value;
		tabs[tabIndex].content = value;
		tabs = tabs;
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
			<a 
				href="#{tab.index}" 
				class="tab {tab.index === tabIndex ? 'selected' : ''}" 
				on:click={(event) => handleSelectTab(event, tab.index)}>
				{tab.path.split('\\')[tab.path.split('\\').length - 1] ? tab.path.split('\\')[tab.path.split('\\').length - 1] + (!tab.saved ? ' *' : ''): 'Untitled *'}
			</a>
		{/each}
	</div>

	{#if tabs.length}
		<CodeMirror bind:value={value} lang={javascript()} theme={oneDark} on:change={handleValueChange}></CodeMirror>
	{/if}

	{#if showTerminal}
		<div class="wrapper-terminal">
			<Terminal></Terminal>
		</div>
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
		overflow: hidden;
		height: 18px;
	}

	.main-tabs a {
		color: lightgray;
		font-size: 12px;
		border-right: 1px solid lightgray;
		padding: 0 0.5em;
	}

	.selected {
		text-decoration: underline;
	}

	.main-tabs a:last-child {
		border-right: none;
	}

	.wrapper-terminal {
		margin-top: 1em;
		margin-left: 1em;
		margin-right: 1em;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>