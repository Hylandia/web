<script lang="ts">
	import { createMutation, useQueryClient } from '@tanstack/svelte-query';
	import { getSession, logoutMutationOptions } from '@hylandia/store';
	import { PUBLIC_API_BASE_URL } from '$env/static/public';
	import logoUrl from '$lib/assets/logo.png?w=100&format=webp&url';
	import navBarUrl from '$lib/assets/nav-bar.png?w=1200&format=webp&url';

	const session = getSession();
	const queryClient = useQueryClient();

	const logout = createMutation(() => ({
		...logoutMutationOptions(PUBLIC_API_BASE_URL),
		onSuccess: () => {
			session.setUser(null);
			queryClient.invalidateQueries({ queryKey: ['auth', 'me'] });
		}
	}));

	const links = [
		{ href: '/news', label: 'News' },
		{ href: '/minigames', label: 'Minigames' },
		{ href: '/leaderboards', label: 'Leaderboards' },
		{ href: '/community', label: 'Community' },
		{ href: '/store', label: 'Store' }
	];

	let mobileOpen = $state(false);

	function closeMobile() {
		mobileOpen = false;
	}
</script>

<svelte:window
	onkeydown={(e) => {
		if (e.key === 'Escape') closeMobile();
	}}
/>

<header class="absolute inset-x-0 top-0 z-20">
	<div class="mx-auto flex max-w-6xl items-center justify-between gap-3 px-4 py-3 sm:px-6 sm:py-4">
		<a href="/" class="flex min-w-0 items-center gap-2 sm:gap-3" onclick={closeMobile}>
			<img
				src={logoUrl}
				alt="Hylandia"
				width="40"
				height="40"
				class="h-9 w-auto drop-shadow-[0_2px_6px_rgba(0,0,0,0.6)] sm:h-10"
			/>
			<span class="font-display text-lg tracking-wide text-hy-cream sm:text-xl">HYLANDIA</span>
		</a>

		<div class="flex shrink-0 items-center gap-3">
			{#if session.status === 'authenticated' && session.user}
				<div class="hidden items-center gap-3 text-sm text-hy-parchment-light lg:flex">
					<span>{session.user.username}</span>
					<button
						onclick={() => logout.mutate()}
						class="rounded border border-hy-gold/40 px-3 py-1.5 transition hover:bg-hy-gold/10"
					>
						Log out
					</button>
				</div>
			{:else if session.status === 'guest'}
				<a
					href={session.loginHref('/')}
					class="hidden rounded border border-hy-gold/50 px-4 py-1.5 font-display text-sm tracking-wide text-hy-gold-light transition hover:bg-hy-gold/10 lg:inline-block"
				>
					Sign in
				</a>
			{/if}

			<button
				onclick={() => (mobileOpen = !mobileOpen)}
				aria-label={mobileOpen ? 'Close menu' : 'Open menu'}
				aria-expanded={mobileOpen}
				aria-controls="mobile-nav"
				class="rounded border border-hy-gold/40 p-2 text-hy-cream transition hover:bg-hy-gold/10 lg:hidden"
			>
				{#if mobileOpen}
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" class="h-5 w-5" fill="none" stroke="currentColor" stroke-width="2">
						<path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
					</svg>
				{:else}
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" class="h-5 w-5" fill="none" stroke="currentColor" stroke-width="2">
						<path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5M3.75 17.25h16.5" />
					</svg>
				{/if}
			</button>
		</div>
	</div>

	<nav
		class="mx-auto hidden max-w-4xl items-center justify-center gap-5 bg-contain bg-center bg-no-repeat px-8 py-3 text-xs font-semibold tracking-wide text-hy-crimson uppercase xl:gap-8 xl:px-10 xl:text-sm lg:flex"
		style="background-image: url({navBarUrl});"
	>
		{#each links as link (link.href)}
			<a href={link.href} class="shrink-0 transition hover:text-hy-ember">{link.label}</a>
		{/each}
	</nav>

	{#if mobileOpen}
		<nav
			id="mobile-nav"
			class="border-t border-hy-gold/20 bg-hy-night/98 px-4 py-5 shadow-[0_12px_40px_rgba(0,0,0,0.55)] backdrop-blur-sm lg:hidden"
		>
			<ul class="mx-auto flex max-w-6xl flex-col gap-1">
				{#each links as link (link.href)}
					<li>
						<a
							href={link.href}
							onclick={closeMobile}
							class="block rounded px-3 py-3 font-display text-sm tracking-[0.14em] text-hy-parchment-light uppercase transition hover:bg-hy-gold/10 hover:text-hy-gold-light"
						>
							{link.label}
						</a>
					</li>
				{/each}
			</ul>

			<div class="mx-auto mt-4 max-w-6xl border-t border-hy-gold/15 px-3 pt-4">
				{#if session.status === 'authenticated' && session.user}
					<p class="mb-3 text-sm text-hy-parchment-light/80">{session.user.username}</p>
					<button
						onclick={() => {
							closeMobile();
							logout.mutate();
						}}
						class="w-full rounded border border-hy-gold/40 px-3 py-2.5 text-left text-hy-parchment-light transition hover:bg-hy-gold/10"
					>
						Log out
					</button>
				{:else if session.status === 'guest'}
					<a
						href={session.loginHref('/')}
						onclick={closeMobile}
						class="block w-full rounded border border-hy-gold/50 px-3 py-2.5 text-center font-display text-sm tracking-wide text-hy-gold-light transition hover:bg-hy-gold/10"
					>
						Sign in
					</a>
				{/if}
			</div>
		</nav>
	{/if}
</header>
