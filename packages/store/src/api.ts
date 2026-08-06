import type { z } from 'zod';
import { envelopeSchema } from './schemas';

export class ApiError extends Error {
	constructor(
		message: string,
		public readonly code: string,
		public readonly status: number
	) {
		super(message);
		this.name = 'ApiError';
	}
}

/** Thrown when `fetch` itself rejects — the API is down, unreachable, or blocked by CORS. */
export class ApiUnreachableError extends Error {
	constructor(cause: unknown) {
		super('API unreachable');
		this.name = 'ApiUnreachableError';
		this.cause = cause;
	}
}

export async function apiFetch<T>(
	baseUrl: string,
	path: string,
	schema: z.ZodType<T>,
	init?: RequestInit
): Promise<T> {
	let res: Response;
	try {
		res = await fetch(`${baseUrl}${path}`, { ...init, credentials: 'include' });
	} catch (cause) {
		throw new ApiUnreachableError(cause);
	}

	const json = await res.json().catch(() => null);

	const envelope = envelopeSchema(schema).safeParse(json);
	if (!envelope.success) {
		throw new ApiError('unexpected response shape', 'BAD_RESPONSE', res.status);
	}

	if (!envelope.data.success || envelope.data.data === null) {
		const error = envelope.data.error;
		throw new ApiError(error?.message ?? 'request failed', error?.code ?? 'UNKNOWN', res.status);
	}

	return envelope.data.data;
}
