import init, { parse_demo } from "tf-decals-from-demo";
import wasmUrl from "tf-decals-from-demo/tf_decals_from_demo_bg.wasm?url";

const initPromise = init({ module_or_path: wasmUrl });

async function parseFile(file: File) {
	const buffer = await file.arrayBuffer();
	await initPromise;

	let ids = parse_demo(new Uint8Array(buffer));
	self.postMessage(ids);
}

self.onmessage = (event) => {
	parseFile(event.data);
}