<script lang="ts">
	const year = new Date().getFullYear();
</script>

<footer class="border-t border-hy-gold/20 bg-hy-night/60">
	<div class="mx-auto max-w-6xl px-6 py-8 text-sm text-hy-parchment-light/70">
		<div class="flex flex-col items-center gap-3 sm:flex-row sm:justify-between">
			<div class="flex items-center gap-4">
				<a href="/news" class="hover:text-hy-gold-light">News</a>
				<a href="/minigames" class="hover:text-hy-gold-light">Minigames</a>
				<a href="/community" class="hover:text-hy-gold-light">Community</a>
			</div>

			<p class="font-display tracking-wide text-hy-parchment">Hylandia Network</p>

			<div class="flex items-center gap-4">
				<a href="https://discord.gg/rBxGm3y5P9" class="hover:text-hy-gold-light" rel="noreferrer">Discord</a>
				<a href="https://x.com/hylandianetwork" class="hover:text-hy-gold-light" rel="noreferrer">X</a>
				<a href="https://www.youtube.com/@HylandiaOfficial" class="hover:text-hy-gold-light" rel="noreferrer">
					YouTube
				</a>
			</div>
		</div>

		<div
			class="mt-6 flex flex-col items-center gap-2 border-t border-hy-gold/10 pt-6 text-xs text-hy-parchment-light/50 sm:flex-row sm:justify-between"
		>
			<p>Copyright {year} Hylandia LLC, an Oklahoma limited liability company. All rights reserved.</p>
			<div class="flex items-center gap-4">
				<a href="/privacy" class="hover:text-hy-gold-light">Privacy</a>
				<a href="/terms" class="hover:text-hy-gold-light">Terms</a>
				<a href="/cookies" class="hover:text-hy-gold-light">Cookies</a>
				<a href="/refunds" class="hover:text-hy-gold-light">Refunds</a>
			</div>
		</div>
	</div>
</footer>
