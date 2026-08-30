//! Hylandia web API — Sign in with Hytale + our own JWT sessions.

## Architecture

Hytale OAuth (Authorization Code + PKCE) is used only to establish identity
at sign-in — `sub` (stable per-app anonymous id) and, via `hytale:profile`,
`username`. Hytale issues no refresh token and always re-prompts for
consent, so there's no way to keep a Hytale session alive silently. Instead,
once a user signs in we mint our **own** session: a short-lived ES256 JWT
access token plus a long-lived, rotating opaque refresh token (hashed in
`sessions`). That refresh token — not anything from Hytale — is what keeps
the user logged in.

Our access tokens are ES256-signed; `GET /.well-known/jwks.json` publishes
the public key so other Hylandia services can verify them independently.

## Run locally

Requires Postgres.

```bash
cp .env.example .env   # fill in HYTALE_*, JWT_*, FRONTEND_URL
cargo run --release --bin api
```

Migrations run automatically at startup (`diesel_migrations`, embedded in
the binary). To generate the ES256 keypair for `JWT_PRIVATE_KEY_PEM` /
`JWT_PUBLIC_KEY_PEM`:

```bash
openssl ecparam -genkey -name prime256v1 -noout -out ec-private.pem
openssl ec -in ec-private.pem -pubout -out ec-public.pem
```

Paste each PEM file's contents as a double-quoted, multi-line value in `.env`.

## Environment variables

| Variable | Required | Notes |
|---|---|---|
| `DATABASE_URL` | yes | |
| `HYTALE_CLIENT_ID` / `HYTALE_CLIENT_SECRET` | yes | confidential client from the Hytale dev portal |
| `HYTALE_REDIRECT_URI` | yes | must exactly match a URI registered for the client |
| `HYTALE_ISSUER` | no | default `https://connect.accounts.hytale.com` |
| `HYTALE_SCOPES` | no | default `openid hytale:profile` |
| `JWT_PRIVATE_KEY_PEM` / `JWT_PUBLIC_KEY_PEM` | yes | ES256 keypair, PEM |
| `JWT_ISSUER` / `JWT_AUDIENCE` | no | default `hylandia-web-api` / `hylandia` |
| `ACCESS_TOKEN_TTL_SECS` | no | default 900 (15m) |
| `REFRESH_TOKEN_TTL_SECS` | no | default 2592000 (30d) |
| `FRONTEND_URL` | yes | origin the browser lands back on after login |
| `COOKIE_DOMAIN` | no | host-only cookies if unset |
| `COOKIE_SECURE` | no | default true; set `false` for plain-http local dev |
| `SENTRY_DSN` | no | |

## Deploy

The site and API images are published to the Hylandia Nexus registry by
`.github/workflows/publish-images.yaml`. Kubernetes deployment, immutable image
digests, runtime configuration, and secrets are managed in `Hylandia/platform`.

## Known limitation

Refresh token rotation happens in place on the same `sessions` row — there's
no token-family/reuse-detection chain yet. A stolen-and-later-reused old
refresh token isn't currently distinguishable from a legitimate one; add a
family id column if that threat model matters before this handles anything
sensitive.
