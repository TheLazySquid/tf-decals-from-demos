<script lang="ts">
    import { parseFiles } from "$lib/parser";

	let fileInput: HTMLInputElement;
	let ids: string[] = $state([]);

	async function parseFile() {
		if(!fileInput.files) return;
		
		parseFiles(fileInput.files, (id) => ids.push(id));
	}
</script>

<input type="file" class="cursor-pointer" accept=".dem" multiple
	bind:this={fileInput} onchange={parseFile} />

{ids.join(", ")}