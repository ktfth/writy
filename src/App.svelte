<script lang="ts">
	import CodeMirror from 'svelte-codemirror-editor';
	import { javascript } from '@codemirror/lang-javascript';
	import { oneDark } from '@codemirror/theme-one-dark';

	import { open, save } from '@tauri-apps/api/dialog';
	import { readTextFile, writeTextFile, BaseDirectory } from '@tauri-apps/api/fs';

	import Terminal from './Terminal.svelte';
	// import { consoleCommand } from './terminal';

	import type { TabFile } from './tab-file';

	export let value: string;
	export let latestFilePath: string;

	export let tabs: Array<TabFile>;
	export let tabIndex: number;

	let showTerminal = false;

	function consoleCommand(input, closeWin = ()=>{}){
		let str = input.trim();
		switch (str) {
			case "new":
				createFile()
					.then(() => showTerminal = !showTerminal);
				return "Creating new file...";
			case "open":
				openFile()
					.then(() => showTerminal = !showTerminal);
				return "Opening file...";
			case "save":
				saveFile()
					.then(() => showTerminal = !showTerminal);
				return "Saving file...";
			case "save-as":
				saveAsFile()
					.then(() => showTerminal = !showTerminal);
				return "Saving file as...";
			case "close":
				closeFile()
					.then(() => showTerminal = !showTerminal);
				return "Closing file...";
			case "help":
				return "Try \"open\"!";
			default:
				return "Mybe you want to \"help\". Try it!";
		}
	}

	async function createFile() {
		tabIndex = tabs[tabs.length - 1].index + 1;
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

	async function openFile() {
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

	async function saveFile() {
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

	async function saveAsFile() {
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

	async function closeFile() {
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

	async function handleShortcut(event) {
		let keyCode = event.keyCode;
		if (event.ctrlKey) {
			// save
			if (!event.shiftKey && keyCode === 83) {
				event.preventDefault();
				await saveFile();
			}
			// save as and alternate back
			if (event.shiftKey) {
				event.preventDefault();
				if (keyCode === 83) {
					await saveAsFile();
				} else if (keyCode === 9) {
					event.preventDefault();
					console.log('shift + tab', tabs[tabIndex - 1]);
					if (tabs[tabIndex - 1] !== undefined) {
						tabIndex = tabs[tabIndex - 1].index;
						latestFilePath = tabs[tabIndex].path;
						value = tabs[tabIndex].content;
						tabs = tabs;
					} else if (tabs[tabIndex - 1] === undefined) {
						tabIndex = tabs.length - 1;
					}
				}
			}
			// open file
			if (keyCode === 79) {
				event.preventDefault();
				await openFile();
			}
			// new file
			if (keyCode === 78) {
				event.preventDefault();
				await createFile();
			}
			// close file
			if (keyCode === 87) {
				event.preventDefault();
				await closeFile();
			}
			// open terminal
			if (keyCode === 192) {
				event.preventDefault();
				showTerminal = !showTerminal;
			}
			// start change focus
			if (!event.shiftKey && keyCode === 9) {
				console.log('tab', tabs[tabIndex + 1]);
				event.preventDefault();
				if (tabs[tabIndex + 1] !== undefined) {
					console.log('exists');
					tabIndex = tabs[tabIndex + 1].index;
					latestFilePath = tabs[tabIndex].path;
					value = tabs[tabIndex].content;
					tabs = tabs;
				} else if (tabs[tabIndex + 1] === undefined) {
					tabIndex = 0;
				}
			}
			// end change focus
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
		<Terminal 
			title="{tabs[tabIndex]?.path.split('\\')[tabs[tabIndex]?.path.split('\\').length - 2]}"
			commands={consoleCommand}
			fontfamily="Monaco"
			fontsize="0.85rem"
			exactClose={() => {}}></Terminal>
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

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>