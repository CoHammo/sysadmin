<script lang="ts">
	import type { PageProps } from './$types';
	import { Exit } from '$lib/classes';

	let { data }: PageProps = $props();
	let command = new Exit(data.post);
	console.log(command);
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read the documentation</p>
<p>{JSON.stringify(command, null, 2)}</p>
