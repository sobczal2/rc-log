# rc-log — Agent & Architecture Guide

## Project Overview

**rc-log** is a web app for cataloguing RC (radio-controlled) vehicle maneuvers. It consists of a Rust backend API and a TypeScript/React frontend, hosted together.

---

## Repository Layout

```
rc-log/
├── backend/          # Rust workspace (Axum HTTP API + PostgreSQL)
│   ├── Cargo.toml    # Workspace manifest + shared dependency versions
│   ├── .env          # Local environment variables (git-ignored)
│   ├── migrations/   # sqlx migration SQL files
│   └── crates/
│       ├── domain/
│       ├── application/
│       ├── persistance/
│       └── api/
└── frontend/         # Vite + React + TypeScript SPA (early stage)
```

---

## Backend — 4-Layer DDD Architecture

Dependencies flow strictly in one direction: `api → application → domain ← persistance`. The `api` crate is the **composition root** — the only place where concrete infrastructure types (e.g. `SqlxManeuverUnitOfWork`) are wired into use cases.

### `crates/domain` — Domain Layer

Pure domain model. No framework dependencies. Owns:

| Path | Contents |
|---|---|
| `src/maneuver/mod.rs` | `Maneuver` aggregate (id, vehicle_type, name, tags, description, difficulty, video_path) |
| `src/maneuver/difficulty.rs` | `Difficulty` enum (Level1–Level7) |
| `src/maneuver/tag.rs` | `Tag` value object (id, name) |
| `src/shared/repository.rs` | `RepositoryError`, `Transaction<T>` trait, `UnitOfWork<T>` trait |
| `src/shared/pagination.rs` | `Pagination` value object (page, page_size) with `offset()`/`limit()` helpers |
| `src/shared/vehicle_type.rs` | `VehicleType` enum (Helicopter, Plane, Drone) |
| `src/shared/markdown_text.rs` | `MarkdownText` newtype |
| `src/shared/video_path.rs` | `VideoPath` newtype |

**`Transaction<T>` trait** (the repository contract):
```rust
fn get_by_id(&mut self, id: Uuid) -> impl Future<Output = Result<Option<T>, RepositoryError>>;
fn list(&mut self, pagination: Pagination) -> impl Future<Output = Result<(Vec<T>, u64), RepositoryError>>;
fn save(&mut self, entity: &T) -> impl Future<Output = Result<(), RepositoryError>>;
fn commit(self) -> impl Future<Output = Result<(), RepositoryError>>;
fn rollback(self) -> impl Future<Output = Result<(), RepositoryError>>;
```

**`UnitOfWork<T>` trait**:
```rust
fn begin(&mut self) -> impl Future<Output = Result<Self::Transaction, RepositoryError>>;
```

---

### `crates/application` — Application Layer

Orchestrates domain operations. Depends only on `domain`. Owns use cases, application errors, and shared result types.

| Path | Contents |
|---|---|
| `src/error.rs` | `ApplicationError` — wraps use case errors via `#[from]` |
| `src/maneuver/get_by_id/error.rs` | `GetManeuverByIdError` (NotFound, InvalidData, RepositoryError) |
| `src/maneuver/get_by_id/model.rs` | `ManeuverDto`, `TagDto` — stable application DTOs |
| `src/maneuver/get_by_id/use_case.rs` | `GetManeuverByIdUseCase<UoW>` — returns `ManeuverDto` |
| `src/maneuver/list/error.rs` | `ListManeuversError` |
| `src/maneuver/list/model.rs` | `ManeuverDto`, `TagDto` |
| `src/maneuver/list/use_case.rs` | `ListManeuversUseCase<UoW>` — returns `PaginatedResult<ManeuverDto>` |
| `src/shared/paginated_result.rs` | `PaginatedResult<T>` (items, total, page, page_size, total_pages()) |

**Use case pattern** (all use cases follow this template):
```rust
pub struct FooUseCase<UoW> { uow: UoW }

impl<UoW: UnitOfWork<Entity>> FooUseCase<UoW> {
    #[instrument(skip(self), fields(...))]
    pub async fn execute(&mut self, ...) -> Result<FooDto, ApplicationError> {
        let mut tx = self.uow.begin().await.map_err(FooError::from)?;
        // ... business logic using domain Entities ...
        tx.commit().await.map_err(FooError::from)?;
        Ok(FooDto::from(domain_entity)) // map to DTO before returning
    }
}
```

> **Layer boundary rule**: Use cases accept and return only application-layer types (`FooDto`, `PaginatedResult<FooDto>`, primitive types). Domain types (`Maneuver`, `Difficulty`, etc.) are internal to the application layer — never exposed to or imported by `api`. The `From<DomainType> for FooDto` impl in `model.rs` is the only place where domain→DTO conversion happens.

> **API response rule**: API handlers must return a *distinct response struct* (e.g. `GetManeuverByIdResponse`, not raw `ManeuverDto`), but that struct **may embed application DTOs directly** as fields. This avoids re-mapping every field while still giving the API layer its own named contract type. Application DTOs derive `Serialize` for this reason.

---

### `crates/persistance` — Persistence Layer

Implements domain repository traits against PostgreSQL via **sqlx**. Depends on `domain`. Never referenced by `application`.

| Path | Contents |
|---|---|
| `src/maneuver/repository.rs` | `SqlxManeuverTransaction`, `SqlxManeuverUnitOfWork` |

**Key implementation details:**
- `SqlxManeuverUnitOfWork` holds a `PgPool` (Arc-backed, `Clone`-derived).
- `get_by_id`: two queries — fetch maneuver row, then fetch its tags.
- `list`: three queries — `COUNT(*)`, paginated `SELECT … ORDER BY name LIMIT/OFFSET`, then one batched `WHERE maneuver_id = ANY($1)` tag fetch (no N+1 problem).
- `ManeuverRow` / `TagRow` / `TagRowWithManeuver` are private sqlx row structs used only for DB mapping.

**Database schema** (`migrations/`):
- `maneuver.maneuver` — core entity table
- `maneuver.tag` — tag lookup table
- `maneuver.maneuver_tag` — many-to-many join table

---

### `crates/api` — API Layer (Composition Root)

Axum HTTP server. Wires concrete infrastructure into use cases. Depends on all other crates.

| Path | Contents |
|---|---|
| `src/main.rs` | Bootstrap: load `.env` → init tracing → build `PgPool` → `AppState` → serve |
| `src/config.rs` | `AppConfig::load()` reads `APP_ENV`, `DATABASE_URL`, `APP_HOST`, `APP_PORT` from env |
| `src/state.rs` | `AppState { maneuver_uow: SqlxManeuverUnitOfWork }` — passed via axum `State` |
| `src/error.rs` | `ApiError: IntoResponse` — maps `ApplicationError` to HTTP status codes |
| `src/shared/pagination.rs` | `PaginationQuery` — reusable axum `FromRequestParts` extractor |
| `src/maneuver/handler.rs` | `get_maneuver_by_id`, `list_maneuvers` |
| `src/maneuver/response.rs` | `GetManeuverByIdResponse`, `ListManeuversResponse`, `ManeuverListItemResponse` |
| `src/maneuver/router.rs` | Mounts maneuver routes |

**Error mapping (`ApiError`):**
| `ManeuverError` | HTTP |
|---|---|
| `NotFound` | 404 |
| `InvalidData` | 500 (bad DB data is a server problem) |
| `RepositoryError` | 500 (internal details not leaked) |

**HTTP Endpoints:**
```
GET /api/maneuvers?page=1&page_size=20   → ListManeuversResponse
GET /api/maneuvers/{id}                  → GetManeuverByIdResponse
```

`PaginationQuery` validates `page >= 1` and `1 <= page_size <= 100`; defaults are page=1, page_size=20. Returns 400 JSON on invalid params.

---

## Configuration (`.env`)

```env
APP_ENV=development        # or: production, prod, dev
DATABASE_URL=              # PostgreSQL connection string
APP_HOST=127.0.0.1
APP_PORT=3000
RUST_LOG=rc_log_api=debug,rc_log_application=debug,sqlx=warn,info
```

`APP_ENV` affects log level defaults. All variables are **required** — no silent defaults outside `.env`.

---

## Tracing

Initialized in `main.rs` from `RUST_LOG` (read from `.env` before subscriber init). Every use case method and handler is annotated with `#[instrument]`. Key fields in spans:
- `maneuver_id` on get-by-id
- `page`, `page_size` on list

---

## Frontend

Early-stage React/TypeScript SPA scaffolded with Vite using **shadcn/ui** components. Source lives in `frontend/src/`. Not yet connected to the backend.

---

## Adding a New Feature — Checklist

1. **Domain**: add any new value objects / entity methods needed.
2. **Application**: create a nested directory for the use case (e.g., `src/<entity>/<use_case>/`). Inside, create `error.rs`, `model.rs` (DTOs + `From<DomainType>` impls), `use_case.rs`, and `mod.rs`. Use cases must return DTO types, not domain types. Add error variant to `ApplicationError`.
3. **Persistence**: implement new `Transaction` methods on the sqlx transaction struct.
4. **API**: add `src/<entity>/response.rs` — define a **distinct response struct** per endpoint. It may embed application DTOs directly as fields (they derive `Serialize`). Do not import from `rc_log_domain` directly.
5. **Import rules**:
   - `application` must never import from `persistance`
   - `api` may import application DTOs but must wrap them in its own named response types — never return a raw DTO from a handler
   - Only `api` (composition root) wires concrete infrastructure into use cases
