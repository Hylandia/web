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
</script>

<header class="absolute inset-x-0 top-0 z-20">
	<div class="mx-auto flex max-w-6xl items-center justify-between gap-4 px-6 py-4">
		<a href="/" class="flex items-center gap-3">
			<img src={logoUrl} alt="Hylandia" width="40" height="40" class="h-10 w-auto drop-shadow-[0_2px_6px_rgba(0,0,0,0.6)]" />
			<span class="font-display text-xl tracking-wide text-hy-cream">HYLANDIA</span>
		</a>

		<div class="flex items-center gap-3">
			{#if session.status === 'authenticated' && session.user}
				<div class="hidden items-center gap-3 text-sm text-hy-parchment-light sm:flex">
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
					class="hidden rounded border border-hy-gold/50 px-4 py-1.5 font-display text-sm tracking-wide text-hy-gold-light transition hover:bg-hy-gold/10 sm:inline-block"
				>
					Sign in
				</a>
			{/if}

			<button
				onclick={() => (mobileOpen = !mobileOpen)}
				aria-label="Toggle menu"
				aria-expanded={mobileOpen}
				class="rounded border border-hy-gold/40 p-2 text-hy-cream sm:hidden"
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
		class="mx-auto hidden max-w-4xl items-center justify-center gap-8 bg-contain bg-center bg-no-repeat px-10 py-3 text-sm font-semibold tracking-wide text-hy-crimson uppercase sm:flex"
		style="background-image: url({navBarUrl});"
	>
		{#each links as link (link.href)}
			<a href={link.href} class="transition hover:text-hy-ember">{link.label}</a>
		{/each}
	</nav>

	{#if mobileOpen}
		<nav class="flex flex-col gap-1 bg-hy-ink/95 px-6 py-4 text-hy-parchment-light sm:hidden">
			{#each links as link (link.href)}
				<a href={link.href} onclick={() => (mobileOpen = false)} class="rounded px-2 py-2 hover:bg-hy-gold/10">
					{link.label}
				</a>
			{/each}

			{#if session.status === 'authenticated' && session.user}
				<button
					onclick={() => {
						mobileOpen = false;
						logout.mutate();
					}}
					class="mt-2 rounded border border-hy-gold/40 px-3 py-2 text-left"
				>
					Log out
				</button>
			{:else if session.status === 'guest'}
				<a
					href={session.loginHref('/')}
					class="mt-2 rounded border border-hy-gold/50 px-3 py-2 text-center font-display text-hy-gold-light"
				>
					Sign in
				</a>
			{/if}
		</nav>
	{/if}
</header>
