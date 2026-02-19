# Boilerplate Architecture Guide

This project is organized as a layered Axum + Askama app. Use this file as the rulebook when cloning it for new projects.

## 1. High-Level Architecture

Request flow:
`Route -> Controller -> Repository -> MySQL`
`Controller -> View Template (Askama) -> HTML response`
`Controller -> Redis (session/cache) -> session + cached option data`

Core principles:
- Keep HTTP concerns in `controllers/`.
- Keep SQL in `repository/`.
- Keep shared types in `models/entities/`.
- Keep template context structs in `views/templates/`.
- Keep application wiring in `routes/` + `main.rs`.
- Before applying or changing template styling/theme, ask the user first and proceed only after they choose the desired direction.

## 2. Folder Contract

- `src/main.rs`: app bootstrap, tracing setup, env loading, DB/Redis/session initialization, server bind.
- `src/state.rs`: global app state (`db`, `redis`) passed with `with_state`.
- `src/routes/`: route namespace wiring.
  - `public.rs` for public pages/auth.
  - `admin.rs` for admin HTML pages + actions.
  - `api/` for JSON endpoints (`/api/v1`, `/api/v2`).
- `src/controllers/`:
  - `auth_controller.rs`: auth extractors (`AdminUser`, `AuthUser`, optional variants).
  - `page_controller/`: request handlers and shared helpers (`shared.rs` for CSRF/cache utilities).
- `src/models/entities/`: DTOs/forms/query models/domain structs.
- `src/repository/`: DB access functions grouped by entity.
- `src/views/templates/`: Askama template structs only.
- `templates/`: HTML templates.
- `static/`: JS/CSS assets.
- `src/bin/`: operational CLI tools (e.g., DB checks, admin bootstrap).

## 3. Layer Responsibilities

Routes:
- Only map paths and methods to controller functions.
- No business logic in route modules.

Controllers:
- Parse extractors (`State`, `Path`, `Query`, `Form`, `Session`).
- Validate input (`validator::Validate`).
- Enforce auth via `AdminUser`/`AuthUser` extractors.
- Handle CSRF via `ensure_csrf_token` + `validate_csrf`.
- Call repository functions.
- Build template context structs or JSON responses.
- Map failures to user-safe messages and status codes.

Repository:
- SQL only. No HTTP/session/template logic.
- Accept `&MySqlPool` and typed parameters.
- Return typed structs (`sqlx::FromRow`) or simple primitives.
- Sanitize dynamic ordering/search parameters before SQL string construction.

Models/Entities:
- Keep form structs, query structs, DB row structs, and view models typed and separate.
- Form structs should include CSRF token where required for POST forms.

Views/Templates:
- Template structs mirror fields actually rendered by `.html` templates.
- Do not place DB or business logic here.

## 4. Security + Session Rules

- Session backend is Redis via `tower-sessions`.
- Admin routes must require `AdminUser` extractor.
- Use CSRF token on all state-changing forms.
- Before implementing password handling, ask the user how they want it done (hash algorithm, password policy, reset/change flow) and implement only after confirmation.
- On successful login, cycle session ID (`session.cycle_id()`).

## 5. Caching Pattern (Redis)

Current pattern in `page_controller/shared.rs`:
- Cache key examples: `geo:countries`, `geo:states:{country_id}`.
- Cache TTL: 300 seconds.
- Read-through strategy:
  1. Try Redis.
  2. Fallback to repository query.
  3. Serialize + store in Redis.
- Invalidate relevant keys after write operations.

Reuse this pattern for lookup/reference data only.

## 6. Logging + Environment

Environment variables used:
- `APP_ENV` (`development`/`production`)
- `LOG_DIR` (optional)
- `DATABASE_URL`
- `REDIS_URL`
- `SESSION_TIMEOUT` (seconds)
- `APP_HOST`
- `APP_PORT` (fallback: `PORT`)

Logging behavior:
- Development: pretty logs to stdout.
- Production or `LOG_DIR` set: rolling daily file logs.

## 7. Feature Implementation Playbook

When adding a new module (example: `products`):

1. Add entity types in `src/models/entities/products.rs` and export from `src/models/entities/mod.rs`.
2. Add repository functions in `src/repository/product_repository.rs` and export in `src/repository/mod.rs`.
3. Add Askama context structs in `src/views/templates/`.
4. Add HTML templates under `templates/admin/products/` (or public path as needed).
5. Add controller handlers in `src/controllers/page_controller/admin.rs` or `public.rs`.
6. Add route mappings in `src/routes/admin.rs` or `src/routes/public.rs`.
7. If forms mutate data, add CSRF validation and error re-render path.
8. If feature has reusable lookup data, add Redis cache helpers + invalidation.
9. Add/update JS/CSS in `static/` if required by UI behavior.
10. Add migration/schema changes and keep SQL column names aligned with structs.

## 8. Naming Conventions

- Handler names:
  - Page render: `<feature>_page` or `<resource>_list` / `<resource>_detail`
  - Form submit: `<feature>_submit`
  - Delete action: `<resource>_delete`
- Repository names:
  - `find_<resource>_by_id`
  - `get_<resource>s`
  - `create_<resource>`
  - `update_<resource>`
  - `delete_<resource>`
- Template structs:
  - `<Area><Action>Template` (example: `AdminEditUserTemplate`)
- Forms: `*Form`, queries: `*Query` / `*Params`, response DTOs: `*Response`.

## 9. What Not To Do

- Do not put SQL in controllers.
- Do not read/write session directly from repository layer.
- Do not bypass CSRF checks for form submissions.
- Do not expose internal DB errors directly to end users.
- Do not add feature logic directly in `main.rs`; keep it in module layers.
