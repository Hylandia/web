<script lang="ts">
	import type { Snippet } from 'svelte';

	let { title, effectiveDate, children }: { title: string; effectiveDate: string; children: Snippet } = $props();
</script>

<svelte:head>
	<title>{title} | Hylandia</title>
</svelte:head>

<article class="mx-auto max-w-3xl px-6 py-16">
	<a href="/" class="text-sm text-hy-gold-light underline underline-offset-4 hover:text-hy-cream">
		Back to home
	</a>

	<h1 class="font-display mt-6 text-3xl tracking-wide text-hy-gold-light">{title}</h1>
	<p class="mt-2 text-sm text-hy-parchment-light/60">Effective date: {effectiveDate}</p>

	<div class="prose-legal mt-8 space-y-6 text-hy-parchment-light/80">
		{@render children()}
	</div>
</article>

<style>
	.prose-legal :global(h2) {
		margin-top: 2rem;
		font-family: var(--font-display);
		font-size: 1.25rem;
		letter-spacing: 0.02em;
		color: var(--color-hy-cream);
	}

	.prose-legal :global(p) {
		line-height: 1.7;
	}

	.prose-legal :global(ul) {
		list-style: disc;
		padding-left: 1.5rem;
		line-height: 1.7;
	}

	.prose-legal :global(a) {
		color: var(--color-hy-gold-light);
		text-decoration: underline;
		text-underline-offset: 4px;
	}
</style>
