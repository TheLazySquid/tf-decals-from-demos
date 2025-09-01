<script lang="ts">
    import { parseFiles, type ParsedDemo } from "$lib/parser";
	import { Modal } from "flowbite-svelte";

	let dropping = $state(false);
	let files: File[] = $state([]);
	let parsed = $state(0);

	let steamApiInfoOpen = $state(false);
	let steamApiKey = $state("");

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
		if(parsing) return;

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
			In order to convert the ids into useful images the
			<a href="https://partner.steamgames.com/doc/webapi/ISteamRemoteStorage#GetUGCFileDetails"
				class="underline" target="_blank">
				Steam API
			</a>
			needs to be used. Unfortunately, this process cannot be automated in the browser due to CORS restrictions.
			The best that can be done is to link to the api endpoint for each decal, which will include a link to the image.
		</p>
		<p class="mt-4">
			You can get a Steam API key by following
			<a href="https://steamcommunity.com/dev/apikey"
				class="underline" target="_blank">
				this link
			</a>
			and signing in with your Steam account. The domain you enter does not matter.
		</p>
	</div>
</Modal>

{#snippet demoDisplay(demo: ParsedDemo)}
	<div class="border-white border rounded-2xl overflow-hidden">
		<div class="border-b px-2 {demo.ids ? "bg-slate-700" : "bg-red-900"}">
			{ demo.name }
		</div>
		<div class="px-2 py-2 flex flex-col">
			{#if !demo.ids}
				Failed to parse demo.
			{:else if demo.ids.length === 0}
				No decals found.
			{:else}
				{#each demo.ids as id}
					{#if steamApiKey}
						<a href="https://api.steampowered.com/ISteamRemoteStorage/GetUGCFileDetails/v1/?key={steamApiKey}&appid=440&ugcid={id}"
							class="underline" target="_blank">
							{id}
						</a>
					{:else}
						<div>{id}</div>
					{/if}
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
				Steam API Key (optional)
			</label>
			<button onclick={() => steamApiInfoOpen = true}
				class="bg-slate-700 w-6 rounded-lg">
				?
			</button>
			<input id="apikey" type="password" bind:value={steamApiKey}
				class="flex-grow border-b border-gray-400 outline-none" />
		</div>
		<div>
			<button disabled={files.length === 0}
				class="bg-slate-700 rounded-lg px-2 py-1"
				onclick={() => startParse()}>
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
				<div class="grid gap-2" style="grid-template-columns: repeat(auto-fit, minmax(240px, 1fr))">
					{#each output as demo}
						{@render demoDisplay(demo)}
					{/each}
				</div>
			</div>
		{/if}
	</div>
</div>