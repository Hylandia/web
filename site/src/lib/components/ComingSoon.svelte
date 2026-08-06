<script lang="ts">
    let { title }: { title: string } = $props();
</script>

<svelte:head>
    <title>{title} — Hylandia</title>
</svelte:head>

<section
    class="mx-auto flex min-h-[60vh] max-w-2xl flex-col items-center justify-center px-6 text-center"
>
    <img src="/brand/logo.png" alt="" class="h-16 w-auto opacity-80" />
    <h1 class="font-display mt-6 text-3xl tracking-wide text-hy-gold-light">
        {title}
    </h1>
    <p class="mt-3 text-hy-parchment-light/70">
        Coming soon. We're still building this part of Hylandia.
    </p>
    <a
        href="/"
        class="mt-8 text-sm text-hy-gold-light underline underline-offset-4 hover:text-hy-cream"
    >
        Back to home
    </a>
</section>
