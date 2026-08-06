import { getContext, setContext } from 'svelte';
import type { User } from './schemas';

const SESSION_KEY = Symbol('hylandia-session');

export type SessionStatus = 'loading' | 'authenticated' | 'guest' | 'unreachable';

/**
 * Instantiated per-request via `provideSession` (SvelteKit SSR runs one
 * module instance per server process, so a module-level singleton would
 * leak session state across requests) and read via `getSession()`.
 */
export class Session {
	user = $state<User | null>(null);
	status = $state<SessionStatus>('loading');

	constructor(private readonly apiBaseUrl: string) {}

	setUser(user: User | null) {
		this.user = user;
		this.status = user ? 'authenticated' : 'guest';
	}

	/** The API couldn't be reached at all — distinct from a reachable API saying "no session". */
	setUnreachable() {
		this.user = null;
		this.status = 'unreachable';
	}

	loginHref(redirectPath = '/'): string {
		const url = new URL('/auth/hytale/login', this.apiBaseUrl);
		url.searchParams.set('redirect', redirectPath);
		return url.toString();
	}
}

export function provideSession(apiBaseUrl: string): Session {
	const session = new Session(apiBaseUrl);
	setContext(SESSION_KEY, session);
	return session;
}

export function getSession(): Session {
	const session = getContext<Session | undefined>(SESSION_KEY);
	if (!session) {
		throw new Error('No Session in context — wrap the component tree in <SessionProvider>');
	}
	return session;
}
