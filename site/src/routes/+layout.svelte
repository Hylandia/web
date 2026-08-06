<script lang="ts">
	import '../app.css';
	import { QueryClient, QueryClientProvider } from '@tanstack/svelte-query';
	import { SessionProvider } from '@hylandia/store';
	import { PUBLIC_API_BASE_URL } from '$env/static/public';
	import type { Snippet } from 'svelte';
	import Header from '$lib/components/Header.svelte';
	import Footer from '$lib/components/Footer.svelte';

	let { children }: { children: Snippet } = $props();

	const queryClient = new QueryClient();
</script>

<QueryClientProvider client={queryClient}>
	<SessionProvider apiBaseUrl={PUBLIC_API_BASE_URL}>
		<div class="relative flex min-h-screen flex-col">
			<Header />
			<main class="flex-1 pt-28">
				{@render children()}
			</main>
			<Footer />
		</div>
	</SessionProvider>
</QueryClientProvider>
