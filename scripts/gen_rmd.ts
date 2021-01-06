/**
 * Generates readme files in /src
 */
import { walk } from "https://deno.land/std@0.83.0/fs/walk.ts";
import { exists } from "https://deno.land/std@0.83.0/fs/exists.ts";
import * as path from "https://deno.land/std@0.83.0/path/mod.ts";

interface DescEnt {
	name: string;
	description: string;
}

async function getDescriptions(path: string): Promise<DescEnt[]> {
	if (!await exists(path)) {
		return [];
	} else {
		let descriptions: DescEnt[] = [];
		// iterate through the files in this path.
		for (let entry of Deno.readDirSync(path)) {
			if (entry.isDirectory) {
				// check if theres a readme
				const rdm: string = path + "/" + entry.name + "/Readme.md";

				if (await exists(rdm)) {
					const contents: string = new TextDecoder().decode(Deno.readFileSync(rdm));
					descriptions.push({
						name: entry.name,
						description: contents.split("\n")[1] || "No description."
					});
				}
			}
		}

		return descriptions;
	}
}
const decode = (file: Uint8Array) => {
	return new TextDecoder().decode(file);
}
const PATH: string = path.resolve(Deno.cwd(), "src");
const DESCRIPT: any = (await exists(path.resolve(Deno.cwd(), 'scripts/descriptions.json')))
	? JSON.parse(decode(Deno.readFileSync(path.resolve(Deno.cwd(), 'scripts/descriptions.json'))))
	: {};
for (let entry of Deno.readDirSync(PATH)) {
	if (entry.isDirectory) {
		console.log("Generating readme for: " + entry.name);
		const descriptions: DescEnt[] = await getDescriptions(path.resolve(PATH, entry.name));
		const entryCapital: string = entry.name.split('')[0].toUpperCase() + entry.name.split('').slice(1).join('');
		const entryDesc: string = DESCRIPT[entry.name] || "No Description.";
		const source: string = `https://github.com/TypeSharp/Typesharp/tree/master/src/${entry.name}/`;

		let fileContents: string = "<!-- This file was auto generated by Typesharp. -->"
			+ "\n# " + entryCapital + " - " + entryDesc;
		for (let desc of descriptions) {
			fileContents += `\n - **[${desc.name}](${source}${desc.name})** - ${desc.description}`;
		}
		Deno.writeFileSync(path.resolve(PATH, entry.name + "/Readme.md"), new TextEncoder().encode(fileContents));
	}
}