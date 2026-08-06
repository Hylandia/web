<script lang="ts">
	import { createMutation, useQueryClient } from '@tanstack/svelte-query';
	import { getSession, logoutMutationOptions } from '@hylandia/store';
	import { PUBLIC_API_BASE_URL } from '$env/static/public';

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
</script>

<header class="absolute inset-x-0 top-0 z-20">
	<div class="mx-auto flex max-w-6xl items-center justify-between gap-4 px-6 py-4">
		<a href="/" class="flex items-center gap-3">
			<img src="/brand/logo.png" alt="Hylandia" class="h-10 w-auto drop-shadow-[0_2px_6px_rgba(0,0,0,0.6)]" />
			<span class="font-display text-xl tracking-wide text-hy-cream">HYLANDIA</span>
		</a>

		{#if session.status === 'authenticated' && session.user}
			<div class="flex items-center gap-3 text-sm text-hy-parchment-light">
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
				class="rounded border border-hy-gold/50 px-4 py-1.5 font-display text-sm tracking-wide text-hy-gold-light transition hover:bg-hy-gold/10"
			>
				Sign in
			</a>
		{/if}
	</div>

	<nav
		class="mx-auto flex max-w-4xl items-center justify-center gap-8 bg-contain bg-center bg-no-repeat px-10 py-3 text-sm font-semibold tracking-wide text-hy-crimson uppercase"
		style="background-image: url(/brand/nav-bar.png);"
	>
		{#each links as link (link.href)}
			<a href={link.href} class="transition hover:text-hy-ember">{link.label}</a>
		{/each}
	</nav>
</header>
