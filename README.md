# Axum Askama Admin Template

Production-oriented starter app with:
- `axum` routing
- `askama` server-rendered templates
- MySQL via `sqlx`
- Redis-backed sessions via `tower-sessions`
- CSRF protection for form posts

## Contributor Notes
- Architecture guide: [`ARCHITECTURE_GUIDE.md`](ARCHITECTURE_GUIDE.md)
- Boilerplate checklist: [`PROJECT_TEMPLATE_CHECKLIST.md`](PROJECT_TEMPLATE_CHECKLIST.md)

## Quick Start
1. Create database:
```sql
CREATE DATABASE axum_app;
```
2. Apply schema:
```bash
mysql -u root -p axum_app < schema.sql
```
3. Configure env:
```bash
cp .env.example .env
```
4. Run app:
```bash
cargo run
```

Default URL: `http://127.0.0.1:3000`

## Environment Variables
- `DATABASE_URL` (required)
- `REDIS_URL` (required)
- `APP_HOST` (default `127.0.0.1`)
- `APP_PORT` (default `3000`)
- `SESSION_TIMEOUT` in seconds (default `604800`)
- `APP_ENV` (`development` or `production`)
- `LOG_DIR` (optional)

## Routes
- `/` public landing page
- `/admin/login` admin login
- `/admin/dashboard` admin dashboard
- `/admin/*` admin CRUD routes (countries, states, users)
- `/api/*` API routes

## Admin Bootstrap
Create an admin account:
```bash
cargo run --bin create_admin -- --username admin --email admin@example.com --password admin123
```

## Template Bootstrap for New Project
Use the included helper:
```powershell
.\scripts\bootstrap-template.ps1 -ProjectName my-admin-app -DatabaseName my_admin_app
```

Then review and complete [`PROJECT_TEMPLATE_CHECKLIST.md`](PROJECT_TEMPLATE_CHECKLIST.md).
