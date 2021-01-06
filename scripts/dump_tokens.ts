/**
 * @warn CLI ONLY
 * Generates a yaml or JSON file of tokens.
 */
import * as path from "https://deno.land/std@0.83.0/path/mod.ts";

function rcal(str: string): string {
	return str.replace(/(\/\/.*| ?\/\*[^]*?\*\/)(,?)$/gm, '').replace(/(\s){2,}/gm, '')
}

function keyWords(): string {
	// target files
	const reservedWordsFile: string = "src/compiler/typesharp_ast/keyword.rs";
	const strReserved = new TextDecoder().decode(Deno.readFileSync(path.resolve(Deno.cwd(), reservedWordsFile)));
	const reservedWords: string[] = rcal(strReserved).split('enum KeyWord {')[1].split('}')[0].split(',');
	const reservedValues: string[] = reservedWords.map(w => w.toLowerCase().replace(/keyword/gm, ''));
	let i = 0;
	return `Keywords:\n -` + reservedWords.map(w => " " + w + ": \"" + reservedValues[i++] + "\"").join('\n -');
}

if (import.meta.main) {
	Deno.writeFile(path.resolve(Deno.cwd(), Deno.args[0]), new TextEncoder().encode(keyWords()));
}