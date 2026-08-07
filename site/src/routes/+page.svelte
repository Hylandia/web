<script lang="ts">
	import heroBgUrl from '$lib/assets/hero-bg.png?w=1920&format=webp&quality=70&url';
	import heroBannerUrl from '$lib/assets/hero-banner.png?w=700&format=webp&url';
	import buttonUrl from '$lib/assets/button-primary.png?w=600&format=webp&url';

	const SERVER_ADDRESS = 'hylandia.net';

	let copied = $state(false);

	async function copyServerAddress() {
		await navigator.clipboard.writeText(SERVER_ADDRESS);
		copied = true;
		setTimeout(() => (copied = false), 2000);
	}
</script>

<svelte:head>
	<title>Hylandia: Your Adventure Begins</title>
</svelte:head>

<section
	class="relative -mt-20 flex min-h-[calc(100vh-1px)] flex-col items-center justify-center overflow-hidden bg-cover bg-center px-6 pt-20 pb-24 text-center lg:-mt-28 lg:pt-28"
	style="background-image: linear-gradient(to bottom, rgba(18,10,8,0.2), var(--color-hy-ink) 90%), url({heroBgUrl});"
>
	<h1>
		<img
			src={heroBannerUrl}
			alt="Hylandia"
			width="700"
			height="277"
			fetchpriority="high"
			class="mx-auto h-24 w-auto drop-shadow-[0_4px_16px_rgba(0,0,0,0.7)] sm:h-32"
		/>
	</h1>

	<button
		onclick={copyServerAddress}
		class="relative mx-auto mt-10 flex w-64 items-center justify-center bg-contain bg-center bg-no-repeat font-display text-lg tracking-wide text-hy-cream drop-shadow-[0_2px_4px_rgba(0,0,0,0.6)] transition hover:brightness-110 sm:w-72"
		style="aspect-ratio: 2141 / 521; background-image: url({buttonUrl});"
	>
		{copied ? 'Copied hylandia.net!' : 'Play on hylandia.net'}
	</button>
</section>
