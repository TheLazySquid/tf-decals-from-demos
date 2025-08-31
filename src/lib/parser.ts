import ParseWorker from "./parseWorker?worker";

const maxWorkers = navigator.hardwareConcurrency || 4;

export function parseFiles(fileList: FileList, onId: (id: string) => void) {
	// Spawn one worker per file, up to maxWorkers
	const files = Array.from(fileList);
	const numWorkers = Math.min(files.length, maxWorkers);

	for(let i = 0; i < numWorkers; i++) {
		const worker = new ParseWorker();
		worker.postMessage(files.pop());

		worker.onmessage = (event) => {
			for(const id of event.data) {
				onId(id);
			}

			const file = files.pop();
			if(!file) {
				worker.terminate();
				return;
			}

			worker.postMessage(file);
		}
	}
}