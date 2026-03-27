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
| `src/user/mod.rs` | `User` aggregate (id, username, email, password_hash) |
| `src/user/query.rs` | `UserTransaction` trait extending `Transaction<User>` with `get_by_username()` |
| `src/shared/repository.rs` | `RepositoryError`, `Transaction<T>` trait, `UnitOfWork<T>` trait |
| `src/shared/pagination.rs` | `Pagination` value object (page, page_size) with `offset()`/`limit()` helpers |
| `src/shared/password_hash.rs` | `PasswordHash` newtype |
| `src/shared/vehicle_type.rs` | `VehicleType` enum (Helicopter, Plane, Drone) |
| `src/shared/markdown_text.rs` | `MarkdownText` newtype |
| `src/shared/video_path.rs` | `VideoPath` newtype |

**`Transaction<T>` trait** (the repository contract):
```rust
fn get_by_id(&mut self, id: Uuid) -> impl Future<Output = Result<Option<T>, TransactionError>>;
fn save(&mut self, entity: &T) -> impl Future<Output = Result<(), TransactionError>>;
fn commit(self) -> impl Future<Output = Result<(), TransactionError>>;
fn rollback(self) -> impl Future<Output = Result<(), TransactionError>>;
```
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
| `src/user/create/error.rs` | `CreateUserError` (ValidationError, UsernameTaken, EmailTaken, RepositoryError) |
| `src/user/create/model.rs` | `CreateUserInput`, `UserDto` |
| `src/user/create/use_case.rs` | `CreateUserUseCase<UoW>` — creates user with hashed password |
| `src/user/get_by_id/error.rs` | `GetUserByIdError` (NotFound, InvalidData, RepositoryError) |
| `src/user/get_by_id/model.rs` | `GetUserByIdInput`, `UserDto` |
| `src/user/get_by_id/use_case.rs` | `GetUserByIdUseCase<UoW>` — returns `UserDto` |
| `src/user/get_by_username/error.rs` | `GetUserByUsernameError` (NotFound, InvalidUsername, InvalidData, RepositoryError) |
| `src/user/get_by_username/model.rs` | `GetUserByUsernameInput`, `UserDto` |
| `src/user/get_by_username/use_case.rs` | `GetUserByUsernameUseCase<UoW>` — returns `UserDto` |
| `src/user/sign_in/error.rs` | `SignInError` (InvalidCredentials, InvalidData, RepositoryError) |
| `src/user/sign_in/model.rs` | `SignInInput`, `UserDto` |
| `src/user/sign_in/use_case.rs` | `SignInUseCase<UoW>` — verifies argon2 hash, returns `UserDto` |
| `src/user/sign_up/error.rs` | `SignUpError` (ValidationError, UsernameTaken, EmailTaken, HashingError, RepositoryError) |
| `src/user/sign_up/model.rs` | `SignUpInput`, `UserDto` |
| `src/user/sign_up/use_case.rs` | `SignUpUseCase<UoW>` — hashes password with argon2, saves user, returns `UserDto` |
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
| `src/maneuver/transaction.rs` | `SqlxManeuverTransaction`, `SqlxManeuverUnitOfWork` |
| `src/user/transaction.rs` | `SqlxUserTransaction`, `SqlxUserUnitOfWork` |

**Conventions:**
- Repository function parameters must use domain value objects (e.g. `&Username`, `&Email`) rather than primitive types (e.g. `&str`, `&Uuid`) wherever a value object exists. This ensures validation is enforced at the domain boundary and callers cannot bypass it.

**Key implementation details:**
- `SqlxManeuverUnitOfWork` holds a `PgPool` (Arc-backed, `Clone`-derived).
- `get_by_id`: two queries — fetch maneuver row, then fetch its tags.
- `list`: three queries — `COUNT(*)`, paginated `SELECT … ORDER BY name LIMIT/OFFSET`, then one batched `WHERE maneuver_id = ANY($1)` tag fetch (no N+1 problem).
- `ManeuverRow` / `TagRow` / `TagRowWithManeuver` are private sqlx row structs used only for DB mapping.

**User repository** (`src/user/transaction.rs`):
- `SqlxUserUnitOfWork` holds a `PgPool` and implements `UnitOfWork<User>`.
- `SqlxUserTransaction` implements both `Transaction<User>` and `UserTransaction` trait.
- `get_by_id(uuid)`: single query to fetch user by ID.
- `get_by_username(username)`: single query to fetch user by username (required by `UserTransaction` extended trait).
- `save(user)`: upsert user record (insert or update on conflict).
- `UserRow` is private sqlx row struct for DB mapping.

**Database schema** (`migrations/`):
- `maneuver.maneuver` — core entity table
- `maneuver.tag` — tag lookup table
- `maneuver.maneuver_tag` — many-to-many join table
- `user.user` — user entity table with unique constraints on username and email

---

### `crates/api` — API Layer (Composition Root)

Axum HTTP server. Wires concrete infrastructure into use cases. Depends on all other crates.

| Path | Contents |
|---|---|
| `src/main.rs` | Bootstrap: load `.env` → init tracing → build `PgPool` → `AppState` → serve |
| `src/config.rs` | `AppConfig::load()` reads `APP_ENV`, `DATABASE_URL`, `APP_HOST`, `APP_PORT`, `APP_ASSET_PATH`, `JWT_SECRET` from env |
| `src/state.rs` | `AppState { maneuver_uow, user_uow, jwt_secret }` — passed via axum `State` |
| `src/error.rs` | `ApiError: IntoResponse` — maps `ApplicationError` to HTTP status codes; includes `Unauthorized` variant (401) |
| `src/jwt.rs` | `JwtClaims`, `create_token()`, `verify_token()`, `new_claims()` — JWT HS256 utilities (24 h expiry) |
| `src/extractors/auth.rs` | `AuthenticatedUser` — axum `FromRequestParts` extractor that validates Bearer JWT; rejects with 401 |
| `src/maneuver/get_by_id/` | `extractor.rs`, `handler.rs`, `response.rs`, `mod.rs` |
| `src/maneuver/list/` | `extractor.rs`, `handler.rs`, `response.rs`, `mod.rs` |
| `src/maneuver/router.rs` | Mounts maneuver routes |
| `src/auth/sign_in/` | `extractor.rs`, `handler.rs`, `response.rs`, `mod.rs` |
| `src/auth/sign_up/` | `extractor.rs`, `handler.rs`, `response.rs`, `mod.rs` |
| `src/auth/router.rs` | Mounts auth routes |
| `src/user/get_by_id/` | `extractor.rs`, `handler.rs`, `response.rs`, `mod.rs` — guarded by `AuthenticatedUser` extractor |
| `src/user/router.rs` | Mounts user routes |

**Error mapping (`ApiError`):**
| Error | HTTP |
|---|---|
| `NotFound` (maneuver/user) | 404 |
| `InvalidData` | 500 (bad DB data is a server problem) |
| `RepositoryError` | 500 (internal details not leaked) |
| `UsernameTaken` / `EmailTaken` | 409 |
| `ValidationError` | 400 |
| `InvalidCredentials` (sign-in) | 401 |
| `Unauthorized` (missing/invalid JWT) | 401 |

**HTTP Endpoints:**
```
GET  /api/maneuvers?page=1&page_size=20   → ListManeuversResponse
GET  /api/maneuvers/{id}                  → GetManeuverByIdResponse

POST /api/auth/sign-up                    → SignUpResponse  { token, user }
POST /api/auth/sign-in                    → SignInResponse  { token, user }

GET  /api/users/{id}          [JWT required]  → GetByIdResponse
```

`PaginationQuery` validates `page >= 1` and `1 <= page_size <= 100`; defaults are page=1, page_size=20. Returns 400 JSON on invalid params.

**JWT / authentication conventions:**
- Tokens are HS256-signed JWTs with a 24 h expiry. The secret is read from `JWT_SECRET` env var.
- `JwtClaims` carries `sub` (user UUID) and `username`.
- Routes that require authentication add `AuthenticatedUser` as a handler parameter — axum resolves it via `FromRequestParts` which validates the `Authorization: Bearer <token>` header. No JWT middleware layer is used; protection is per-handler.
- Password hashing is done in the `sign_up` use case via **argon2** (`Argon2::default()`, random salt). Verification is in the `sign_in` use case.
- The `sign_in` and `sign_up` handlers both return `{ token, user }` so the client can bootstrap immediately after registration.

---

## Configuration (`.env`)

```env
APP_ENV=development        # or: production, prod, dev
DATABASE_URL=              # PostgreSQL connection string
APP_HOST=127.0.0.1
APP_PORT=3000
JWT_SECRET=                # Required — secret key for HS256 JWT signing
RUST_LOG=rc_log_api=debug,rc_log_application=debug,sqlx=warn,info
```

`APP_ENV` affects log level defaults. All variables are **required** — no silent defaults outside `.env`.

---

## Tracing

Initialized in `main.rs` from `RUST_LOG` (read from `.env` before subscriber init). Every use case method and handler is annotated with `#[instrument]`. Key fields in spans:
- `maneuver_id` on get-by-id
- `page`, `page_size` on list
- `username` on sign-in/sign-up and user use cases
- `user_id` on get-user-by-id

---

## Frontend — Domain-Driven TypeScript Architecture

React/TypeScript SPA scaffolded with Vite using **shadcn/ui** components and Tailwind CSS. Source lives in `frontend/src/`.

### Architecture Overview

```
frontend/src/
├── context/             # React context providers
│   └── AuthContext.tsx  # AuthProvider, useAuth hook — JWT + user state
├── domain/              # Domain layer — business types and formatting logic
│   ├── maneuver/        # Maneuver aggregate
│   │   ├── maneuver.ts     # Maneuver interface (matches backend DTO)
│   │   ├── difficulty.ts   # DifficultyLevel type (level1-level7) + formatting functions
│   │   ├── vehicle.tsx     # VehicleType type + icon component
│   │   ├── tag.ts          # Tag interface
│   │   ├── filters.ts      # Filter/sort/pagination types
│   │   └── index.ts        # Barrel export
│   └── user/            # User aggregate
│       ├── user.ts         # User interface (id, username, email)
│       └── index.ts        # Barrel export
├── lib/api/             # API layer — HTTP client and request/response types
│   ├── apiClient.ts     # Axios instance with JWT request interceptor and 401 response interceptor
│   ├── auth.ts          # authApi (signIn, signUp) — returns { token, user }
│   └── maneuvers.ts     # maneuversApi (list, getById)
├── hooks/               # Custom React hooks
│   ├── useManeuverFilters.ts  # URL-synced filter state
│   └── useDebounce.ts
├── components/          # React components
│   ├── auth/            # Auth-related components
│   │   └── ProtectedRoute.tsx  # Redirects to /sign-in when not authenticated
│   ├── maneuvers/       # Maneuver-specific components
│   │   ├── ManeuverCard.tsx
│   │   ├── ManeuverFilters.tsx
│   │   └── ActiveFilterBadge.tsx
│   ├── layout/          # Layout components
│   └── ui/              # shadcn/ui components
└── pages/               # Page components
    ├── HomePage.tsx
    ├── ManeuverDetailsPage.tsx
    ├── ManeuversPage.tsx
    ├── SignInPage.tsx    # Username + password form — calls authApi.signIn
    └── SignUpPage.tsx    # Username + email + password form — calls authApi.signUp
```

### Domain Layer (`domain/`)

**Principles:**
- Types directly reflect backend DTOs (camelCase field names)
- Use **types** (interfaces/type aliases), not interfaces for data
- Formatting logic lives in domain functions, not components
- No mappers needed — domain types match API response exactly

**Example — difficulty formatting:**
```typescript
// domain/maneuver/difficulty.ts
export type DifficultyLevel = "level1" | "level2" | ... | "level7";

export function getDifficultyColor(difficulty: DifficultyLevel): string { ... }

export function getDifficultyLevelName(vehicleType: VehicleType, difficulty: DifficultyLevel): string {
  // Returns "Beginner", "Basic 3D", etc.
}
```

**Example — vehicle icons:**
```typescript
// domain/maneuver/vehicle.tsx (JSX file for React component return)
export function getVehicleIcon(vehicleType: VehicleType, size = 18): ReactNode {
  switch (vehicleType) {
    case "Plane": return <Plane size={size} />;
    case "Helicopter": return <Helicopter size={size} />;
    case "Drone": return <Drone size={size} />;
  }
}
```

### Authentication (`context/AuthContext.tsx`, `lib/api/auth.ts`)

**`AuthProvider`** wraps the whole app (in `App.tsx`). It persists `token` and `user` to `localStorage` and exposes them via `useAuth()`.

**`useAuth()` returns:**
- `user: User | null` — deserialized from localStorage on init
- `token: string | null` — JWT string
- `isAuthenticated: boolean`
- `signIn(req)` / `signUp(req)` — call the API and store the returned token + user
- `signOut()` — clears localStorage and resets state

**Axios interceptors (`lib/apiClient.ts`):**
- **Request**: attaches `Authorization: Bearer <token>` if a token is present in `localStorage`.
- **Response**: on 401, clears `token` and `user` from `localStorage` and redirects to `/sign-in`.

**`ProtectedRoute`** (`components/auth/ProtectedRoute.tsx`) — wraps any route element that requires auth; redirects to `/sign-in` when `isAuthenticated` is `false`.

**Routes:** `/sign-in` → `SignInPage`, `/sign-up` → `SignUpPage`.

The sidebar footer shows **Sign In / Register** buttons when logged out, and a **Sign Out** button with the username when logged in.

### API Layer (`lib/api/`)

**Principles:**
- Handles HTTP specifics (building query params, axios calls)
- Re-exports domain types for convenience
- Request/response types reference domain types directly (no duplication)

**Example:**
```typescript
// lib/api/maneuvers.ts
import type { Maneuver, ManeuverFilter, ManeuverSort } from "@/domain/maneuver";

export interface ListManeuversRequest extends PaginationOptions {
  filter?: ManeuverFilter;
  sort?: ManeuverSort;
}

export const maneuversApi = {
  list: async (req: ListManeuversRequest): Promise<ListManeuversResponse> => {
    const params = new URLSearchParams();
    // ... build query params
    const { data } = await apiClient.get<ListManeuversResponse>("/maneuvers", { params });
    return data;
  },
};
```

### Backend Serialization

Backend DTOs use `#[serde(rename_all = "camelCase")]` to serialize fields as camelCase:
- `vehicle_type` → `vehicleType`
- `page_size` → `pageSize`
- `video_path` → `videoPath`
- `total_pages` → `totalPages`

Difficulty serializes as lowercase string (`level1`–`level7`), not integer.

### Adding a New Feature — Frontend Checklist

1. **Domain**: Add types in `domain/<entity>/` matching backend DTOs. Add formatting functions for display logic.
2. **API**: Add request/response types in `lib/api/<entity>.ts`. Request types reference domain types.
3. **Components**: Use domain types and domain formatting functions — never duplicate display logic.
4. **Auth-gated routes**: Wrap the route element in `<ProtectedRoute>` in `App.tsx`.
5. **Import rules**:
   - Components import from `@/domain/maneuver` (or other entity) for types and formatting
   - API layer imports from `@/domain/<entity>` for type references
   - Never duplicate domain types in API layer

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
6. **Code Style**:
   - Always prefer explicit `use` declarations (e.g., `use std::env;`) instead of inline fully-qualified paths (`std::env::var()`), unless doing so creates severe ambiguity. Expand this preference across the entire backend workspace.
   - Each API operation lives in its own subdirectory under the entity: `src/<entity>/<operation>/` with `extractor.rs`, `handler.rs`, `response.rs`, `mod.rs`.
   - `DifficultyLevel` serializes as lowercase strings (`level1`–`level7`) from the backend.
