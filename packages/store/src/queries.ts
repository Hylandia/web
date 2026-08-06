import { z } from 'zod';
import { apiFetch } from './api';
import { UserSchema, type User } from './schemas';

const emptySchema = z.object({});

export function meQueryOptions(baseUrl: string) {
	return {
		queryKey: ['auth', 'me'] as const,
		queryFn: () => apiFetch<User>(baseUrl, '/auth/me', UserSchema),
		retry: false
	};
}

export function refreshMutationOptions(baseUrl: string) {
	return {
		mutationKey: ['auth', 'refresh'] as const,
		mutationFn: () => apiFetch(baseUrl, '/auth/refresh', emptySchema, { method: 'POST' })
	};
}

export function logoutMutationOptions(baseUrl: string) {
	return {
		mutationKey: ['auth', 'logout'] as const,
		mutationFn: () => apiFetch(baseUrl, '/auth/logout', emptySchema, { method: 'POST' })
	};
}
