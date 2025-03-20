import type { PageLoad } from './$types';
import { decode } from '@msgpack/msgpack';

export const load: PageLoad = async ({ params }) => {
	// define command used to create the subprocess
	const command = new Deno.Command('../sysadmin-rs/target/debug/sysadmin', {
		args: ['-o=msgpack', 'packages', 'list', 'gitui', 'sqlite'],
		stdout: 'piped',
		stderr: 'piped'
	});

	// create subprocess and collect output
	const { code, stdout, stderr } = await command.output();

	// console.assert(code === 0);
	// console.log(new TextDecoder().decode(stdout));
	// console.log(new TextDecoder().decode(stderr));

	const object = decode(stdout);
	console.log(object);
	// const exit = new Exit(object);
	// console.log(exit);
	return {
		post: object
	};
};
