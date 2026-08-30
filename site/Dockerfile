# syntax=docker/dockerfile:1
#
# Build context must be web/ (monorepo root), not site/. Images are published
# by .github/workflows/publish-images.yaml and deployed by Hylandia/platform.

FROM node:22-bookworm-slim AS build
WORKDIR /app
RUN corepack enable

COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
COPY packages/store/package.json packages/store/package.json
COPY site/package.json site/package.json
RUN pnpm install --frozen-lockfile

COPY packages/store packages/store
COPY site site

# SvelteKit inlines $env/static/public at build time, so the API origin
# has to be baked in here rather than read at container runtime.
ARG PUBLIC_API_BASE_URL
ENV PUBLIC_API_BASE_URL=$PUBLIC_API_BASE_URL

RUN pnpm --filter @hylandia/site build

FROM node:22-bookworm-slim
WORKDIR /app

# adapter-node doesn't bundle third-party deps or emit its own package.json
# here, so it needs the real node_modules tree from the build stage,
# including pnpm's symlink layout, hence copying the whole workspace rather
# than just build/.
COPY --from=build /app /app

ENV NODE_ENV=production
ENV PORT=8080
EXPOSE 8080
USER node
CMD ["node", "site/build/index.js"]
