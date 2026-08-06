<svelte:head>
	<title>Hylandia: Your Adventure Begins</title>
</svelte:head>

<section
	class="relative -mt-28 flex min-h-[calc(100vh-1px)] flex-col items-center justify-center overflow-hidden bg-cover bg-center px-6 pt-28 pb-24 text-center"
	style="background-image: linear-gradient(to bottom, rgba(18,10,8,0.2), var(--color-hy-ink) 90%), url(/brand/hero-bg.png);"
>
	<h1>
		<img
			src="/brand/hero-banner.png"
			alt="Hylandia"
			class="mx-auto h-24 w-auto drop-shadow-[0_4px_16px_rgba(0,0,0,0.7)] sm:h-32"
		/>
	</h1>

	<a
		href="#play"
		class="relative mx-auto mt-10 flex w-72 items-center justify-center bg-contain bg-center bg-no-repeat font-display text-lg tracking-wide text-hy-cream drop-shadow-[0_2px_4px_rgba(0,0,0,0.6)] transition hover:brightness-110"
		style="aspect-ratio: 2141 / 521; background-image: url(/brand/button-primary.png);"
	>
		Play on hylandia.net
	</a>
</section>
