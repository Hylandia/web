import { redirect } from '@sveltejs/kit';
import type { Handle } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
	if (event.url.hostname === 'www.hylandia.net') {
		const target = new URL(event.url);
		target.hostname = 'hylandia.net';
		redirect(308, target.toString());
	}

	return resolve(event);
};
