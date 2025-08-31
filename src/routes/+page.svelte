<script lang="ts">
    import { parseFiles, type ParsedDemo } from "$lib/parser";
	import { Modal } from "flowbite-svelte";

	let dropping = $state(false);
	let files: File[] = $state([]);
	let steamApiKey = $state("");
	let steamApiInfoOpen = $state(false);
	let parsed = $state(0);

	function setFiles(list?: FileList | null) {
		if (!list) return;
		files = Array.from(list)
			.filter((f) => f.name.endsWith(".dem"));
	}

	function onDragover(e: DragEvent) {
		e.preventDefault();
	}

	function onDragenter(e: DragEvent) {
		e.preventDefault();
		dropping = true;
	}

	function onDragleave() {
		dropping = false;
	}

	function onDrop(e: DragEvent) {
		e.preventDefault();
		dropping = false;

		setFiles(e.dataTransfer?.files);
	}

	function openPicker() {
		let input = document.createElement("input");
		input.type = "file";
		input.accept = ".dem";
		input.multiple = true;
		input.addEventListener("change", () => {
			setFiles(input.files);
		});
		input.click();
	}

	let parsing = $state(false);
	let output: ParsedDemo[] = $state([]);

	function startParse() {
		parsing = true;
		parsed = 0;
		output = [];
		
		parseFiles($state.snapshot(files), (demo) => {
			output.push(demo);
			parsed++;
		}).then(() => parsing = false);
	}
</script>

<Modal bind:open={steamApiInfoOpen}>
	<div class="text-gray-200 pr-4">
		<p>
			Demos only store the ids of decals, rather than the actual image on the decal.
			In order to convert the ids into useful images, the
			<a href="https://partner.steamgames.com/doc/webapi/ISteamRemoteStorage#GetUGCFileDetails"
				class="underline" target="_blank">
				Steam API
			</a>
			needs to be used. Your Steam API key will not be stored or sent anywhere aside from the Steam API.
		</p>
		<p class="mt-4">
			You can get a Steam API key by following
			<a href="https://steamcommunity.com/dev/apikey"
				class="underline" target="_blank">
				this link
			</a>
			and signing in with your Steam account.
			Without the API key this tool can still extract only the decal ids if you wish.
		</p>
	</div>
</Modal>

{#snippet demoDisplay(demo: ParsedDemo)}
	<div class="border-white border rounded-2xl overflow-hidden">
		<div class="border-b px-2 {demo.ids ? "bg-slate-700" : "bg-red-900"}">
			{ demo.name }
		</div>
		<div class="px-2 py-2">
			{#if !demo.ids}
				Failed to parse demo.
			{:else if demo.ids.length === 0}
				No decals found.
			{:else}
				{#each demo.ids as id}
					<p>{ id }</p>
				{/each}
			{/if}
		</div>
	</div>
{/snippet}

<div class="flex justify-center">
	<div class="flex flex-col px-4 gap-2" style="width: min(100%, 800px)">
		<h1 class="text-2xl font-bold">Extract Decals from TF2 Demos</h1>
		<button class="bg-slate-700 h-[200px] rounded-2xl text-center
			content-center border-white border-2 {dropping ? "border-solid" : "border-dashed"}"
			ondragover={onDragover} ondragenter={onDragenter} ondragleave={onDragleave}
			ondrop={onDrop} onclick={openPicker}>
			{#if files.length === 0}
				Upload or drag and drop demos.
				<div class="text-xs">
					Nothing will be uploaded, demos are processed locally in your browser.
				</div>
			{:else if files.length === 1}
				{files[0].name}
			{:else}
				{files.length} files selected
			{/if}
		</button>
		<div class="flex gap-2">
			<label for="apikey">
				Steam API Key
			</label>
			<button onclick={() => steamApiInfoOpen = true}
				class="bg-slate-700 w-6 rounded-lg">
				?
			</button>
			<input id="apikey" type="password" bind:value={steamApiKey}
				class="flex-grow border-b border-gray-400 outline-none" />
		</div>
		<div>
			<button disabled={files.length === 0 || !steamApiKey}
				class="bg-slate-700 rounded-lg px-2 py-1">
				Extract Decals
			</button>
			<button disabled={files.length === 0}
				class="bg-slate-700 rounded-lg px-2 py-1"
				onclick={startParse}>
				Extract Decal Ids
			</button>
		</div>
		{#if parsing || output.length > 0}
			<div class="bg-slate-800 rounded-2xl border-white border p-2 mt-2 flex flex-col gap-2">
				{#if parsing}
					<div class="flex items-center gap-4">
						<div class="text-xl font-bold">Parsing Demos</div>
						<progress max={files.length} value={parsed}></progress>
					</div>
				{/if}
				<div class="flex flex-wrap gap-2">
					{#each output as demo}
						{@render demoDisplay(demo)}
					{/each}
				</div>
			</div>
		{/if}
	</div>
</div>