<script lang="ts">
	import LegalPage from '$lib/components/LegalPage.svelte';
</script>

<LegalPage title="Cookie Policy" effectiveDate="August 6, 2026">
	<p>This page explains the cookies hylandia.net uses.</p>

	<h2>Strictly necessary cookies</h2>
	<p>These cookies are required for the site to function and cannot be turned off without breaking sign-in.</p>
	<ul>
		<li><strong>access_token</strong>: a short-lived cookie that keeps you signed in</li>
		<li><strong>refresh_token</strong>: a longer-lived cookie used to renew your session without signing in again</li>
		<li><strong>hytale_oauth</strong>: a short-lived cookie used only during the "Sign in with Hytale" redirect, to confirm the sign-in request came from you</li>
	</ul>
	<p>All of these cookies are read only by our servers and cannot be accessed by JavaScript running on the page.</p>

	<h2>Other cookies</h2>
	<p>We do not currently use analytics, advertising, or tracking cookies. If that changes, we will update this page.</p>

	<h2>Managing cookies</h2>
	<p>You can control or delete cookies through your browser settings. Since our cookies are strictly necessary, blocking them will prevent you from signing in.</p>

	<h2>Contact</h2>
	<p>Questions about this policy can be sent to <a href="mailto:support@hylandia.net">support@hylandia.net</a>.</p>
</LegalPage>
