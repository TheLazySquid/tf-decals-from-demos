import ParseWorker from "./parseWorker?worker";

const maxWorkers = navigator.hardwareConcurrency || 4;

export interface ParsedDemo {
	name: string;
	ids?: string[];
}

export function parseFiles(fileList: File[], onParsed: (parsed: ParsedDemo) => void) {
	return new Promise<void>((res) => {
		// Spawn one worker per file, up to maxWorkers
		const files = Array.from(fileList);
		const numWorkers = Math.min(files.length, maxWorkers);
		let workersDone = 0;
	
		for(let i = 0; i < numWorkers; i++) {
			let file = files.pop();		
			const worker = new ParseWorker();
			worker.postMessage(file);
	
			worker.onmessage = (event) => {
				onParsed({
					name: file!.name,
					ids: event.data
				});
	
				file = files.pop();
				if(!file) {
					worker.terminate();

					workersDone++;
					if(workersDone === numWorkers) res();
					return;
				}
	
				worker.postMessage(file);
			}
		}
	})
}