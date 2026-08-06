import { z } from 'zod';

export const UserSchema = z.object({
	id: z.uuid(),
	username: z.string(),
	displayName: z.string().nullable(),
	avatarUrl: z.string().nullable(),
	email: z.string().nullable(),
	emailVerified: z.boolean()
});

export type User = z.infer<typeof UserSchema>;

export function envelopeSchema<T extends z.ZodTypeAny>(data: T) {
	return z.object({
		success: z.boolean(),
		data: data.nullable(),
		error: z.object({ code: z.string(), message: z.string() }).nullable(),
		meta: z.object({
			requestId: z.string(),
			traceId: z.string(),
			spanId: z.string(),
			timestamp: z.string(),
			durationMs: z.number()
		})
	});
}
