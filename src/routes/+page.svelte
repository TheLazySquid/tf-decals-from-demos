<script lang="ts">
	import { parse_demo } from "tf-decals-from-demo";

	let fileInput: HTMLInputElement;
	let output = $state("");
	async function parseFile() {
		const file = fileInput.files?.[0];
		if(!file) return;

		const buffer = await file.arrayBuffer();
		const result = parse_demo(new Uint8Array(buffer));
		if(result) output = "Decal ids: " + result.join(", ");
		else output = "Failed to parse demo";
	}
</script>

<input type="file" class="cursor-pointer" accept=".dem"
	bind:this={fileInput} onchange={parseFile} />
{output}