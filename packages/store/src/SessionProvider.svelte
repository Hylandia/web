<script lang="ts">
	import { createQuery } from '@tanstack/svelte-query';
	import type { Snippet } from 'svelte';
	import { ApiUnreachableError } from './api';
	import { provideSession } from './session.svelte';
	import { meQueryOptions } from './queries';

	let { apiBaseUrl, children }: { apiBaseUrl: string; children: Snippet } = $props();

	const session = provideSession(apiBaseUrl);
	const me = createQuery(() => meQueryOptions(apiBaseUrl));

	$effect(() => {
		if (me.isSuccess) {
			session.setUser(me.data);
		} else if (me.isError) {
			if (me.error instanceof ApiUnreachableError) {
				session.setUnreachable();
			} else {
				session.setUser(null);
			}
		}
	});
</script>

{@render children()}
