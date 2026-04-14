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
| `src/maneuver/mod.rs` | `Maneuver` aggregate (id, type, name, tags, description, difficulty, default_variation, other_variations) |
| `src/maneuver/difficulty.rs` | `Difficulty` enum (Level1–Level7) |
| `src/maneuver/tag.rs` | `Tag` value object (id, name) |
| `src/maneuver/variation.rs` | `Variation` entity (id, maneuver_id, name, description: MarkdownText, video_asset_name: AssetName) |
| `src/user/mod.rs` | `User` aggregate (id, username, email, password_hash, photo_asset_name: Option<AssetName>) |
| `src/user/query.rs` | `UserTransaction` trait extending `Transaction<User>` with `get_by_id()`, `get_by_username()`, `get_by_email()` |
| `src/model/mod.rs` | `Model` aggregate (id, owner_id: UserId, name: Name, type: Type, photo_asset_name: Option<AssetName>) |
| `src/model/id.rs` | `ModelId(Uuid)` newtype (Copy, wraps Uuid via `::new()`, `::as_uuid()`, `From<ModelId> for Uuid`) |
| `src/model/model_resolver.rs` | `ModelResolver` trait — `get_by_id(&self, &ModelId) -> Option<Model>` |
| `src/model/name.rs` | `Name` validated newtype (non-empty, ≤255 chars) + `NameError` |
| `src/model/transaction.rs` | `ModelTransaction` trait extending `Transaction<Model>` with `get_by_id()`, `list_by_owner()`, `delete_by_id()` |
| `src/session/mod.rs` | `Session` aggregate (id, user_id, date, model_id, note, performed_variations) |
| `src/session/date.rs` | `Date` value object (`YYYY-MM-DD`) + `DateError` |
| `src/session/id.rs` | `SessionId(Uuid)` newtype |
| `src/session/performed_variation.rs` | `PerformedVariation` entity (id, variation_id, quality, comfort, repeatability, note) — each metric uses `Rating` |
| `src/session/performed_variation_id.rs` | `PerformedVariationId(Uuid)` newtype |
| `src/session/rating.rs` | `Rating` value object (`One`–`Five`) + conversion helpers (`from_i16`, `as_i16`) |
| `src/session/transaction.rs` | `SessionTransaction` trait extending `Transaction<Session>` with `get_by_id()` |
| `src/shared/transaction.rs` | `TransactionError`, `Transaction<T>` trait |
| `src/shared/unit_of_work.rs` | `UnitOfWork<T>` trait |
| `src/shared/email.rs` | `Email` validated newtype (non-empty, trimmed, ≤255 chars, contains @) + `EmailError` |
| `src/shared/pagination.rs` | `Pagination` value object (page, page_size) with `offset()`/`limit()` helpers |
| `src/shared/password_hash.rs` | `PasswordHash` newtype |
| `src/shared/type.rs` | `Type` enum (Helicopter, Plane, Drone) |
| `src/shared/markdown_text.rs` | `MarkdownText` newtype |
| `src/asset/mod.rs` | Re-exports all asset types and traits |
| `src/asset/name.rs` | `AssetName` newtype (non-empty, trimmed, ≤255 chars) + `AssetNameError` |
| `src/asset/path.rs` | `AssetPath` newtype (non-empty, trimmed) + `AssetPathError` |
| `src/asset/size.rs` | `AssetSize` enum (`Small`, `Medium`, `Large`) |
| `src/asset/video.rs` | `Video` aggregate + `resolve_path(&self, AssetSize) -> &AssetPath` (fallback: Large→Medium→Small) |
| `src/asset/photo.rs` | `Photo` aggregate (identical structure to `Video`) |
| `src/asset/video_resolver.rs` | `VideoResolver` trait — `get(&self, &AssetName) -> Option<Video>` |
| `src/asset/photo_resolver.rs` | `PhotoResolver` trait — `get(&self, &AssetName) -> Option<Photo>` |
| `src/asset/photo_service.rs` | `PhotoService` trait — `save(&self, &AssetName, &[u8]) -> Photo` and `delete(&self, &AssetName) -> ()` + `PhotoServiceError` |
| `src/asset/photo_transaction.rs` | `PhotoTransaction` trait extending `Transaction<Photo>` with `get_by_name()`, `delete_by_name()` |
| `src/asset/video_transaction.rs` | `VideoTransaction` trait extending `Transaction<Video>` with `get_by_name()`, `delete_by_name()` |

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
fn begin(&mut self) -> impl Future<Output = Result<Self::Transaction, TransactionError>>;
```

---

### `crates/application` — Application Layer

Orchestrates domain operations. Depends only on `domain`. Owns use cases, application errors, and shared result types.

| Path | Contents |
|---|---|
| `src/error.rs` | `ApplicationError` — wraps use case errors via `#[from]` |
| `src/maneuver/get_by_id/error.rs` | `GetManeuverByIdError` (NotFound, InvalidData, RepositoryError) |
| `src/maneuver/get_by_id/model.rs` | `ManeuverDto`, `TagDto`, `VariationDto` — stable application DTOs; `ManeuverDto` includes `default_variation: VariationDto` and `variations: Vec<VariationDto>` |
| `src/maneuver/get_by_id/use_case.rs` | `GetManeuverByIdUseCase<UoW>` — returns `ManeuverDto` |
| `src/maneuver/list/error.rs` | `ListManeuversError` |
| `src/maneuver/list/model.rs` | `ManeuverDto`, `TagDto`; `ManeuverDto` includes `default_variation_video_asset_name: String` (from the default variation only) |
| `src/maneuver/list/use_case.rs` | `ListManeuversUseCase<UoW>` — returns `PaginatedResult<ManeuverDto>` |
| `src/maneuver/shared/difficulty.rs` | Shared `DifficultyDto` used by maneuver DTOs within the maneuver bounded context |
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
| `src/user/update/error.rs` | `UpdateUserError` (NotFound, ValidationError, UsernameTaken, InvalidData, RepositoryError) |
| `src/user/update/model.rs` | `UpdateUserInput`, `UserDto` |
| `src/user/update/use_case.rs` | `UpdateUserUseCase<UoW>` — checks username availability, updates user, returns `UserDto` |
| `src/user/update_photo/error.rs` | `UpdateUserPhotoError` (NotFound, InvalidPhotoContent, InvalidData, RepositoryError, PhotoServiceError) |
| `src/user/update_photo/model.rs` | `UpdateUserPhotoInput { user_id, data: Vec<u8> }`, `UserDto` |
| `src/user/update_photo/use_case.rs` | `UpdateUserPhotoUseCase<UoW, PS>` — generates `user-photo-{uuid}`, saves photo, updates user, best-effort delete old photo |
| `src/user/remove_photo/error.rs` | `RemoveUserPhotoError` (NotFound, InvalidData, RepositoryError, PhotoServiceError) |
| `src/user/remove_photo/model.rs` | `RemoveUserPhotoInput { user_id }` — no output struct |
| `src/user/remove_photo/use_case.rs` | `RemoveUserPhotoUseCase<UoW, PS>` — sets `photo_asset_name: None`, save, commit, best-effort delete old photo |
| `src/video/resolve/error.rs` | `ResolveVideoError` (NotFound, InvalidName, InvalidData, ResolverError) |
| `src/video/resolve/model.rs` | `ResolveVideoInput`, `VideoPathsDto` — raw stored paths (smallPath always present, mediumPath/largePath: `Option<String>`) |
| `src/video/resolve/use_case.rs` | `ResolveVideoUseCase<R: VideoResolver>` — looks up video by name, returns `VideoPathsDto` |
| `src/photo/resolve/error.rs` | `ResolvePhotoError` (NotFound, InvalidName, InvalidData, ResolverError) |
| `src/photo/resolve/model.rs` | `ResolvePhotoInput`, `PhotoPathsDto` — raw stored paths (smallPath always present, mediumPath/largePath: `Option<String>`) |
| `src/photo/resolve/use_case.rs` | `ResolvePhotoUseCase<R: PhotoResolver>` — looks up photo by name, returns `PhotoPathsDto` |
| `src/model/get_by_id/error.rs` | `GetModelByIdError` (NotFound, Forbidden, InvalidData, RepositoryError) |
| `src/model/get_by_id/model.rs` | `GetModelByIdInput { id, owner_id }`, `ModelDto` |
| `src/model/get_by_id/use_case.rs` | `GetModelByIdUseCase<UoW>` — ownership check: returns Forbidden if owner_id mismatch |
| `src/model/list/error.rs` | `ListModelsError` (InvalidData, RepositoryError) |
| `src/model/list/model.rs` | `ListModelsInput { owner_id, pagination }`, `ModelDto` |
| `src/model/list/use_case.rs` | `ListModelsUseCase<UoW>` — returns `PaginatedResult<ModelDto>` scoped to owner |
| `src/model/shared/type.rs` | Shared `TypeDto` used by model DTOs and API extractors for model type values |
| `src/model/create/model.rs` | `CreateModelInput { owner_id, name, type }`, `ModelDto` |
| `src/model/create/use_case.rs` | `CreateModelUseCase<UoW>` — validates name, creates model with `photo_asset_name: None`, saves |
| `src/model/update/error.rs` | `UpdateModelError` (NotFound, Forbidden, ValidationError, InvalidData, RepositoryError) |
| `src/model/update/model.rs` | `UpdateModelInput { id, owner_id, name, type }`, `ModelDto` |
| `src/model/update/use_case.rs` | `UpdateModelUseCase<UoW>` — get, ownership check, preserve existing photo, save |
| `src/model/delete/error.rs` | `DeleteModelError` (NotFound, Forbidden, RepositoryError) |
| `src/model/delete/model.rs` | `DeleteModelInput { id, owner_id }` — no output struct |
| `src/model/delete/use_case.rs` | `DeleteModelUseCase<UoW, PS>` — get, ownership check, delete_by_id, commit, then best-effort `photo_service.delete` |
| `src/model/update_photo/error.rs` | `UpdateModelPhotoError` (NotFound, Forbidden, InvalidPhotoContent, InvalidData, RepositoryError, PhotoServiceError) |
| `src/model/update_photo/model.rs` | `UpdateModelPhotoInput { model_id, owner_id, data: Vec<u8> }`, `ModelDto` |
| `src/model/update_photo/use_case.rs` | `UpdateModelPhotoUseCase<UoW, PS>` — get, ownership check, store new photo (name = `model-photo-{uuid}`), save, commit, best-effort delete old photo |
| `src/model/remove_photo/error.rs` | `RemoveModelPhotoError` (NotFound, Forbidden, InvalidData, RepositoryError, PhotoServiceError) |
| `src/model/remove_photo/model.rs` | `RemoveModelPhotoInput { model_id, owner_id }` — no output struct |
| `src/model/remove_photo/use_case.rs` | `RemoveModelPhotoUseCase<UoW, PS>` — get, ownership check, set `photo_asset_name: None`, save, commit, best-effort delete old photo |
| `src/session/create/error.rs` | `CreateSessionError` (ValidationError, ModelNotFound, InvalidData, RepositoryError) |
| `src/session/create/model.rs` | `CreateSessionInput { user_id, date, model_id, note }`, `SessionDto` |
| `src/session/create/use_case.rs` | `CreateSessionUseCase<SessionUoW, ModelUoW>` — validates date/note, verifies optional `model_id` exists, creates session with empty performed variations, saves |
| `src/session/delete/error.rs` | `DeleteSessionError` (NotFound, Forbidden, RepositoryError) |
| `src/session/delete/model.rs` | `DeleteSessionInput { id, owner_id }` — no output struct |
| `src/session/delete/use_case.rs` | `DeleteSessionUseCase<UoW>` — get session, ownership check, delete_by_id, commit |
| `src/session/list/error.rs` | `ListSessionsError` (InvalidData, RepositoryError) |
| `src/session/list/model.rs` | `ListSessionsInput { owner_id, pagination, filter, sort }`, `SessionDto` (list-view shape with `model_name`, `model_type`, and per-item `maneuver_name`/`variation_name`; no notes) |
| `src/session/list/use_case.rs` | `ListSessionsUseCase<UoW, MR, ManR, VarR>` — owner-scoped paginated list with filters (`model_ids`, `maneuver_ids`, `search_query`) and sort (`date`); enriches list DTO using resolvers and infers `model_type` from first performed variation when session model is absent |
| `src/session/shared/rating.rs` | Shared `RatingDto` (`one`–`five`) + conversion helpers; used by `quality`/`comfort`/`repeatability` fields in session DTOs |
| `src/session/update/error.rs` | `UpdateSessionError` (NotFound, Forbidden, ModelNotFound, ValidationError, InvalidData, RepositoryError) |
| `src/session/update/model.rs` | `UpdateSessionInput { id, owner_id, date, model_id, note }`, `SessionDto` |
| `src/session/update/use_case.rs` | `UpdateSessionUseCase<UoW, MR, ManR, VarR>` — get session, ownership check, validate model type compatibility with existing performed variations, update date/model/note, save |
| `src/session/add_performed_variation/error.rs` | `AddPerformedVariationError` (NotFound, Forbidden, ValidationError, InvalidData, RepositoryError) |
| `src/session/add_performed_variation/model.rs` | `AddPerformedVariationInput { session_id, owner_id, performed_variation_id, variation_id, quality: RatingDto, comfort: RatingDto, repeatability: RatingDto, note }`, `SessionDto` |
| `src/session/add_performed_variation/use_case.rs` | `AddPerformedVariationUseCase<UoW, MR, ManR, VarR>` — get session, ownership check, validate maneuver/model type compatibility (or existing performed-variation type when no model), append new performed variation by unique `performed_variation_id`, save |
| `src/session/update_performed_variation/error.rs` | `UpdatePerformedVariationError` (NotFound, Forbidden, PerformedVariationNotFound, ValidationError, InvalidData, RepositoryError) |
| `src/session/update_performed_variation/model.rs` | `UpdatePerformedVariationInput { session_id, owner_id, performed_variation_id, quality: RatingDto, comfort: RatingDto, repeatability: RatingDto, note }`, `SessionDto` |
| `src/session/update_performed_variation/use_case.rs` | `UpdatePerformedVariationUseCase<UoW>` — get session, ownership check, update performed variation by `performed_variation_id`, save |
| `src/session/remove_performed_variation/error.rs` | `RemovePerformedVariationError` (NotFound, Forbidden, PerformedVariationNotFound, InvalidData, RepositoryError) |
| `src/session/remove_performed_variation/model.rs` | `RemovePerformedVariationInput { session_id, owner_id, performed_variation_id }`, `SessionDto` |
| `src/session/remove_performed_variation/use_case.rs` | `RemovePerformedVariationUseCase<UoW>` — get session, ownership check, remove performed variation by `performed_variation_id`, save |
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

**Use case pattern for resolver-based use cases** (no transaction/UoW — resolvers are injected directly):
```rust
pub struct FooResolveUseCase<R> { resolver: R }

impl<R: FooResolver> FooResolveUseCase<R> {
    #[instrument(skip(self), fields(name = %input.name))]
    pub async fn execute(&self, input: FooInput) -> Result<FooDto, ApplicationError> {
        let name = AssetName::new(input.name).map_err(|e| FooError::InvalidName(e.to_string()))?;
        let asset = self.resolver.get(&name).await.map_err(FooError::from)?.ok_or(FooError::NotFound)?;
        Ok(FooDto::from(asset))
    }
}
```

> Note: `execute` takes `&self` (not `&mut self`) for resolver-based use cases since no mutable state is needed.

> **Layer boundary rule**: Use cases accept and return only application-layer types (`FooDto`, `PaginatedResult<FooDto>`, primitive types). Domain types (`Maneuver`, `Difficulty`, etc.) are internal to the application layer — never exposed to or imported by `api`. The `From<DomainType> for FooDto` impl in `model.rs` is the only place where domain→DTO conversion happens.

> **API response rule**: API handlers must return a *distinct response struct* (e.g. `GetManeuverByIdResponse`, not raw `ManeuverDto`), but that struct **may embed application DTOs directly** as fields. This avoids re-mapping every field while still giving the API layer its own named contract type. Application DTOs derive `Serialize` for this reason.

---

### `crates/persistance` — Persistence Layer

Implements domain repository traits against PostgreSQL via **sqlx**. Depends on `domain`. Never referenced by `application`.

| Path | Contents |
|---|---|
| `src/maneuver/transaction.rs` | `SqlxManeuverTransaction`, `SqlxManeuverUnitOfWork` |
| `src/model/transaction.rs` | `SqlxModelTransaction`, `SqlxModelUnitOfWork` |
| `src/model/resolver.rs` | `SqlxModelResolver` — cached model-by-id resolver using `moka` |
| `src/session/transaction.rs` | `SqlxSessionTransaction`, `SqlxSessionUnitOfWork` |
| `src/user/transaction.rs` | `SqlxUserTransaction`, `SqlxUserUnitOfWork` |
| `src/asset/video.rs` | `SqlxVideoResolver` — cached resolver for `Video` assets |
| `src/asset/photo.rs` | `SqlxPhotoResolver` — cached resolver for `Photo` assets |
| `src/asset/photo_service.rs` | `DiskDbPhotoService` — implements `PhotoService`; resizes images adaptively to WebP, writes to disk, upserts `asset.photo` row |

**Conventions:**
- Repository function parameters must use domain value objects (e.g. `&Username`, `&Email`, `ModelId`, `UserId`) rather than primitive types (e.g. `&str`, `&Uuid`) wherever a value object exists. This ensures validation is enforced at the domain boundary and callers cannot bypass it.
- Primitive `Uuid` types cross the application/API boundary (in DTO input/output structs). Domain value objects (`ModelId`, `UserId`, `Name`, etc.) are only used *inside* the application layer and below. Never expose domain value object types to the API layer.

**Key implementation details:**
- `SqlxManeuverUnitOfWork` holds a `PgPool` (Arc-backed, `Clone`-derived).
- `get_by_id`: three queries — fetch maneuver row, fetch its tags, fetch all its variations; splits variations into `default_variation` (required) and `other_variations`.
- `list`: four queries — `COUNT(*)`, paginated `SELECT … ORDER BY LIMIT/OFFSET`, batched tag fetch (`WHERE maneuver_id = ANY($1)`), batched default-variation fetch (`WHERE maneuver_id = ANY($1) AND is_default = TRUE`); no N+1 problems.
- `save`: upserts maneuver row (no `video_path`), deletes/re-inserts tags, deletes/re-inserts all variations (default with `is_default=TRUE`, others with `is_default=FALSE`).
- `ManeuverRow` / `TagRow` / `TagRowWithManeuver` / `VariationRow` are private sqlx row structs used only for DB mapping.

**User repository** (`src/user/transaction.rs`):
- `SqlxUserUnitOfWork` holds a `PgPool` and implements `UnitOfWork<User>`.
- `SqlxUserTransaction` implements both `Transaction<User>` and `UserTransaction` trait.
- `get_by_id(uuid)`: single query to fetch user by ID.
- `get_by_username(username)`: single query to fetch user by username (required by `UserTransaction` extended trait).
- `save(user)`: upsert user record (insert or update on conflict).
- `UserRow` is private sqlx row struct for DB mapping.

**Asset resolvers** (`src/asset/video.rs`, `src/asset/photo.rs`):
- `SqlxVideoResolver` / `SqlxPhotoResolver` each hold a `PgPool` and a `moka::future::Cache<String, Arc<Video/Photo>>` keyed on asset name.
- Both expose a concrete `resolve(&self, &AssetName, AssetSize) -> impl Future<…>` inherent method and implement the `VideoResolver` / `PhotoResolver` domain trait (`get(&self, &AssetName) -> Option<Video/Photo>`).
- `::new(pool, settings: CacheSettings)` — cache capacity and TTL are configured in API env (`RC_LOG_*_CACHE_SIZE`, `RC_LOG_*_CACHE_TTL_SECONDS`).
- **Cache strategy**: per-asset (one cache entry per name; all sizes derived from that entry). On a cache miss the full row is fetched from DB, inserted into the cache, then `resolve_path(size)` is called on the cached value.
- **Size fallback**: `Large` → `medium_path` → `small_path`; `Medium` → `small_path`; `Small` always present (DB `NOT NULL`).
- Both resolvers are `Clone` (moka cache shares the underlying store via `Arc`).

**Photo service** (`src/asset/photo_service.rs`):
- `DiskDbPhotoService { pool: PgPool, asset_path: PathBuf }` — `Clone`-derived; implements the `PhotoService` domain trait.
- `::new(pool, asset_path: PathBuf)` — wired in `AppState::new`.
- `save(&self, name, data)`: CPU work in `tokio::task::spawn_blocking` — decode with `image` crate → adaptive Lanczos3 resize → WebP encode → write to `{asset_path}/photos/` → upsert `asset.photo` row. Returns `Photo` domain value.
- `delete(&self, name)`: fetch paths from DB → delete DB row → remove files (ignore `NotFound`). Always returns `Ok(())`; file errors are logged as warnings.
- **Adaptive sizing** (longest side = max(width, height)): ≤400px → `small` only; ≤800px → `small` + `medium`; >800px → `small` + `medium` + `large`. Target pixel sizes: small=400, medium=800, large=1600. Images smaller than a tier's target are never upscaled.
- **Stored paths** are relative to `asset_path`, e.g. `photos/{name}_small.webp`.
- **Asset name convention** for model photos: `model-photo-{uuid}` (fresh UUID per upload).

**Database schema** (`migrations/`):
- `maneuver.maneuver` — core entity table (no `video_path` column)
- `maneuver.tag` — tag lookup table
- `maneuver.maneuver_tag` — many-to-many join table
- `maneuver.variation` — `id UUID PK`, `maneuver_id UUID FK`, `name TEXT NOT NULL`, `description TEXT NOT NULL`, `video_asset_name TEXT NOT NULL`, `is_default BOOLEAN NOT NULL`; unique partial index `(maneuver_id) WHERE is_default = TRUE` enforces exactly one default per maneuver
- `user.user` — user entity table with unique constraints on username and email
- `asset.video` — `id UUID PK`, `name VARCHAR(255) UNIQUE NOT NULL`, `small_path TEXT NOT NULL`, `medium_path TEXT`, `large_path TEXT`
- `asset.photo` — identical structure to `asset.video`
- `model.model` — `id UUID PK`, `owner_id UUID FK → user.user`, `name VARCHAR(255) NOT NULL`, `type VARCHAR(50) NOT NULL`, `photo_asset_name VARCHAR(255)`; no FK to asset (loosely coupled)
- `session.session` — `id UUID PK`, `user_id UUID FK → user.user`, `date DATE NOT NULL`, `model_id UUID FK → model.model`, `note TEXT`
- `session.performed_variation` — `id UUID PK`, `session_id UUID FK`, `variation_id UUID FK`, `quality SMALLINT NOT NULL`, `comfort SMALLINT NOT NULL`, `repeatability SMALLINT NOT NULL` + optional note; duplicate `variation_id` entries are allowed per session

---

### `crates/api` — API Layer (Composition Root)

Axum HTTP server. Wires concrete infrastructure into use cases. Depends on all other crates.

| Path | Contents |
|---|---|
| `src/main.rs` | Bootstrap: load `.env` → init tracing → build `PgPool` → `AppState` → serve |
| `src/bin/typegen_models.rs` | Specta type generator: exports application DTO contracts to `frontend/src/models/__generated/` |
| `src/config.rs` | `AppConfig::load()` reads `RC_LOG_ENV`, `RC_LOG_DATABASE_URL`, `RC_LOG_HOST`, `RC_LOG_PORT`, `RC_LOG_ASSET_PATH`, `RC_LOG_JWT_SECRET`, `RC_LOG_MODEL_CACHE_TTL_SECONDS`, `RC_LOG_MODEL_CACHE_SIZE`, `RC_LOG_MANEUVER_CACHE_TTL_SECONDS`, `RC_LOG_MANEUVER_CACHE_SIZE`, `RC_LOG_VARIATION_CACHE_TTL_SECONDS`, `RC_LOG_VARIATION_CACHE_SIZE`, `RC_LOG_VIDEO_CACHE_TTL_SECONDS`, `RC_LOG_VIDEO_CACHE_SIZE`, `RC_LOG_PHOTO_CACHE_TTL_SECONDS`, `RC_LOG_PHOTO_CACHE_SIZE` from env |
| `src/state.rs` | `AppState { maneuver_uow, model_uow, session_uow, user_uow, model_resolver, maneuver_resolver, variation_resolver, video_resolver, photo_resolver, photo_service, jwt_secret }` — passed via axum `State`; `::new(pool, jwt_secret, cache_config, asset_path)` |
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
| `src/user/update/` | `extractor.rs`, `handler.rs`, `response.rs`, `mod.rs` — guarded by `AuthenticatedUser` |
| `src/user/update_photo/` | `extractor.rs`, `handler.rs`, `response.rs`, `mod.rs` — guarded by `AuthenticatedUser`; multipart `photo` field |
| `src/user/remove_photo/` | `handler.rs`, `mod.rs` — guarded by `AuthenticatedUser`; returns 204 No Content |
| `src/user/router.rs` | Mounts user routes |
| `src/model/get_by_id/` | `extractor.rs`, `handler.rs`, `response.rs`, `mod.rs` — guarded by `AuthenticatedUser` |
| `src/model/list/` | `extractor.rs`, `handler.rs`, `response.rs`, `mod.rs` — guarded by `AuthenticatedUser` |
| `src/model/create/` | `extractor.rs`, `handler.rs`, `response.rs`, `mod.rs` — guarded by `AuthenticatedUser` |
| `src/model/update/` | `extractor.rs`, `handler.rs`, `response.rs`, `mod.rs` — guarded by `AuthenticatedUser` |
| `src/model/delete/` | `handler.rs`, `mod.rs` — guarded by `AuthenticatedUser`; returns 204 No Content |
| `src/model/update_photo/` | `extractor.rs`, `handler.rs`, `response.rs`, `mod.rs` — guarded by `AuthenticatedUser`; multipart `photo` field (image/jpeg, image/png, image/webp) |
| `src/model/remove_photo/` | `extractor.rs`, `handler.rs`, `mod.rs` — guarded by `AuthenticatedUser`; returns 204 No Content (no `response.rs` needed) |
| `src/model/router.rs` | Mounts model routes |
| `src/session/create/` | `extractor.rs`, `handler.rs`, `response.rs`, `mod.rs` — guarded by `AuthenticatedUser` |
| `src/session/delete/` | `handler.rs`, `mod.rs` — guarded by `AuthenticatedUser`; returns 204 No Content |
| `src/session/list/` | `extractor.rs`, `handler.rs`, `response.rs`, `mod.rs` — guarded by `AuthenticatedUser` |
| `src/session/update/` | `extractor.rs`, `handler.rs`, `response.rs`, `mod.rs` — guarded by `AuthenticatedUser` |
| `src/session/add_performed_variation/` | `extractor.rs`, `handler.rs`, `response.rs`, `mod.rs` — guarded by `AuthenticatedUser` |
| `src/session/update_performed_variation/` | `extractor.rs`, `handler.rs`, `response.rs`, `mod.rs` — guarded by `AuthenticatedUser` |
| `src/session/remove_performed_variation/` | `extractor.rs`, `handler.rs`, `response.rs`, `mod.rs` — guarded by `AuthenticatedUser` |
| `src/session/router.rs` | Mounts session routes |
| `src/asset_paths/video/` | `extractor.rs`, `handler.rs`, `response.rs`, `mod.rs` |
| `src/asset_paths/photo/` | `extractor.rs`, `handler.rs`, `response.rs`, `mod.rs` |
| `src/asset_paths/router.rs` | Mounts asset-path routes |

**Error mapping (`ApiError`):**
| Error | HTTP |
|---|---|
| `NotFound` (maneuver/user/model/asset) | 404 |
| `Forbidden` (ownership mismatch on model) | 403 |
| `InvalidData` | 500 (bad DB data is a server problem) |
| `RepositoryError` / `ResolverError` | 500 (internal details not leaked) |
| `UsernameTaken` / `EmailTaken` | 409 |
| `InvalidPhotoContent` (bad image data) | 400 |
| `PhotoServiceError` | 500 |
| `InvalidCredentials` (sign-in) | 401 |
| `Unauthorized` (missing/invalid JWT) | 401 |

**HTTP Endpoints:**
```
GET  /api/maneuvers?page=1&page_size=20   → ListManeuversResponse
GET  /api/maneuvers/{id}                  → GetManeuverByIdResponse

POST /api/auth/sign-up                    → SignUpResponse  { token, user }
POST /api/auth/sign-in                    → SignInResponse  { token, user }

GET  /api/users/{id}          [JWT required]  → GetByIdResponse
PUT  /api/users/me             [JWT required]  → UpdateResponse
PUT  /api/users/me/photo       [JWT required]  → UpdatePhotoResponse  (multipart form-data, field: photo)
DELETE /api/users/me/photo     [JWT required]  → 204 No Content

GET  /api/asset-paths/video/{name}        → ResolveVideoResponse  { name, smallPath, mediumPath?, largePath? }
GET  /api/asset-paths/photo/{name}        → ResolvePhotoResponse  { name, smallPath, mediumPath?, largePath? }

GET    /api/models              [JWT required]  → ListResponse  { items, total, page, pageSize, totalPages }
POST   /api/models              [JWT required]  → CreateResponse  (201 Created)
GET    /api/models/{id}         [JWT required]  → GetByIdResponse
PUT    /api/models/{id}         [JWT required]  → UpdateResponse
DELETE /api/models/{id}         [JWT required]  → 204 No Content
PUT    /api/models/{id}/photo   [JWT required]  → UpdatePhotoResponse  (multipart form-data, field: photo)
DELETE /api/models/{id}/photo   [JWT required]  → 204 No Content

POST /api/sessions              [JWT required]  → CreateSessionResponse  (201 Created)
GET  /api/sessions              [JWT required]  → ListResponse  { items, total, page, pageSize, totalPages }
PUT  /api/sessions/{id}         [JWT required]  → UpdateSessionResponse
DELETE /api/sessions/{id}       [JWT required]  → 204 No Content
POST /api/sessions/{id}/performed-variations                                           [JWT required]  → AddPerformedVariationResponse
PUT  /api/sessions/{id}/performed-variations/{performed_variation_id}                  [JWT required]  → UpdatePerformedVariationResponse
DELETE /api/sessions/{id}/performed-variations/{performed_variation_id}                [JWT required]  → RemovePerformedVariationResponse
```

Also adds `RemoveModelPhoto` error to `ApiError` mapping: `NotFound` → 404, `Forbidden` → 403, all other variants → 500.

`PaginationQuery` validates `page >= 1` and `1 <= page_size <= 100`; defaults are page=1, page_size=20. Returns 400 JSON on invalid params.

**JWT / authentication conventions:**
- Tokens are HS256-signed JWTs with a 24 h expiry. The secret is read from `RC_LOG_JWT_SECRET` env var.
- `JwtClaims` carries `sub` (user UUID) and `username`.
- Routes that require authentication add `AuthenticatedUser` as a handler parameter — axum resolves it via `FromRequestParts` which validates the `Authorization: Bearer <token>` header. No JWT middleware layer is used; protection is per-handler.
- Password hashing is done in the `sign_up` use case via **argon2** (`Argon2::default()`, random salt). Verification is in the `sign_in` use case.
- The `sign_in` and `sign_up` handlers both return `{ token, user }` so the client can bootstrap immediately after registration.

---

## Configuration (`.env`)

```env
RC_LOG_ENV=development          # or: production, prod, dev
RC_LOG_DATABASE_URL=            # PostgreSQL connection string
RC_LOG_HOST=127.0.0.1
RC_LOG_PORT=3000
RC_LOG_ASSET_PATH=              # Filesystem path served at /api/assets
RC_LOG_JWT_SECRET=              # Required — secret key for HS256 JWT signing
RC_LOG_MODEL_CACHE_TTL_SECONDS=300    # TTL for model resolver cache (seconds)
RC_LOG_MODEL_CACHE_SIZE=1024          # Max entries for model resolver cache
RC_LOG_MANEUVER_CACHE_TTL_SECONDS=300 # TTL for maneuver resolver cache (seconds)
RC_LOG_MANEUVER_CACHE_SIZE=1024       # Max entries for maneuver resolver cache
RC_LOG_VARIATION_CACHE_TTL_SECONDS=300 # TTL for variation resolver cache (seconds)
RC_LOG_VARIATION_CACHE_SIZE=1024      # Max entries for variation resolver cache
RC_LOG_VIDEO_CACHE_TTL_SECONDS=300    # TTL for video resolver cache (seconds)
RC_LOG_VIDEO_CACHE_SIZE=1024          # Max entries for video resolver cache
RC_LOG_PHOTO_CACHE_TTL_SECONDS=300    # TTL for photo resolver cache (seconds)
RC_LOG_PHOTO_CACHE_SIZE=1024          # Max entries for photo resolver cache
RUST_LOG=rc_log_api=debug,rc_log_application=debug,sqlx=warn,info
```

All variables are **required** — no silent defaults outside `.env`. `RC_LOG_ENV` affects log level defaults.

---

## Tracing

Initialized in `main.rs` from `RUST_LOG` (read from `.env` before subscriber init). Every use case method and handler is annotated with `#[instrument]`. Key fields in spans:
- `maneuver_id` on get-by-id
- `page`, `page_size` on list
- `username` on sign-in/sign-up and user use cases
- `user_id` on get-user-by-id
- `user_id`, `date` on create-session

---

## Frontend — Domain-Driven TypeScript Architecture

React/TypeScript SPA scaffolded with Vite using **shadcn/ui** components and Tailwind CSS. Source lives in `frontend/src/`.

### Architecture Overview

```
frontend/src/
├── context/             # React context providers
│   ├── auth-context.ts  # AuthContext value type + createContext call
│   └── AuthContext.tsx  # AuthProvider component — JWT + user state, localStorage persistence
├── models/              # Domain layer — business types and formatting logic
│   ├── __generated/     # Auto-generated Specta contracts (do not edit manually)
│   ├── maneuver/        # Maneuver aggregate
│   │   ├── list.ts         # Wrapper: generated DTO re-export + frontend filter/sort helpers
│   │   ├── get-by-id.ts    # Wrapper: generated DTO re-export
│   │   └── index.ts        # Barrel export (re-exports as ListManeuverDto, GetByIdManeuverDto, etc.)
│   ├── model/           # Model aggregate
│   │   ├── list.ts         # Wrapper: generated DTO re-export
│   │   ├── get-by-id.ts    # Wrapper: generated DTO re-export
│   │   ├── create.ts       # CreateModelRequest (manual) + CreateModelDto (generated alias)
│   │   ├── update.ts       # UpdateModelRequest (manual) + UpdateModelDto (generated alias)
│   │   ├── update-photo.ts # Wrapper: generated DTO re-export
│   │   └── index.ts        # Barrel export
│   ├── user/            # User aggregate
│   │   ├── get-by-id.ts    # Wrapper: generated DTO re-export
│   │   ├── sign-in.ts      # Wrapper: generated DTO re-export
│   │   ├── sign-up.ts      # Wrapper: generated DTO re-export
│   │   ├── update.ts       # UpdateUserRequest (manual) + UpdateUserDto (generated alias)
│   │   ├── update-photo.ts # Wrapper: generated DTO re-export
│   │   └── index.ts        # Barrel export
│   ├── session/         # Session aggregate
│   │   ├── list.ts         # Generated DTO aliases + frontend filter/sort + rating helper functions
│   │   ├── create.ts       # CreateSessionRequest (manual) + CreateSessionDto (generated alias)
│   │   ├── update.ts       # UpdateSessionRequest (manual) + UpdateSessionDto (generated alias)
│   │   ├── add-performed-variation.ts  # Request (manual) + DTO/rating aliases (generated)
│   │   ├── update-performed-variation.ts # Request (manual) + rating aliases (generated)
│   │   └── index.ts        # Barrel export
│   ├── asset/           # Asset types
│   │   ├── photo.ts        # Generated DTO alias + getPhotoUrl()
│   │   ├── video.ts        # Generated DTO alias + getVideoUrl()
│   │   └── index.ts
│   └── shared/          # Shared domain types
│       ├── type.tsx # Type type + getVehicleIcon(), getVehicleLabel()
│       ├── difficulty.ts   # DifficultyLevel type + formatting functions
│       ├── pagination.ts   # PaginatedResult, PaginationOptions
│       └── index.ts        # Barrel export
├── lib/api/             # API layer — HTTP client and request/response types
│   ├── apiClient.ts     # Axios instance with JWT request interceptor and 401 response interceptor
│   ├── auth.ts          # authApi (signIn, signUp) — returns { token, user }
│   ├── maneuvers.ts     # maneuversApi (list, getById)
│   ├── models.ts        # modelsApi (list, getById, create, update, delete, updatePhoto, removePhoto)
│   ├── sessions.ts      # sessionsApi (list, create, update, delete, addPerformedVariation, updatePerformedVariation, removePerformedVariation)
│   └── assets.ts        # assetsApi (getPhotoPath, getVideoPath)
├── hooks/               # Custom React hooks
│   ├── useManeuverFilters.ts  # URL-synced filter state
│   ├── usePhotoPath.ts  # React Query hook for photo path resolution
│   ├── useVideoPath.ts  # React Query hook for video path resolution
│   └── useDebounce.ts
├── components/          # React components
│   ├── auth/            # Auth-related components
│   │   └── ProtectedRoute.tsx  # Redirects to /sign-in when not authenticated
│   ├── maneuvers/       # Maneuver-specific components
│   ├── models/          # Model-specific components (ModelCard, CreateModelDialog)
│   ├── sessions/        # Session-specific components (SessionCard)
│   ├── layout/          # Layout components
│   └── ui/              # shadcn/ui components
└── pages/               # Page components
    ├── HomePage.tsx
    ├── ManeuverDetailsPage.tsx
    ├── ManeuversPage.tsx
    ├── ModelDetailsPage.tsx
    ├── ProfilePage.tsx
    ├── SignInPage.tsx    # Username + password form — calls authApi.signIn
    └── SignUpPage.tsx    # Username + email + password form — calls authApi.signUp
```

### Domain Layer (`models/`)

**Principles:**
- Generated contracts in `models/__generated/` are source-of-truth for backend DTO shapes
- Wrapper files in `models/<entity>/` preserve stable import paths and hold frontend-only logic
- Use **interfaces** for DTO shapes; `type` aliases for union types (e.g. `Type`, `DifficultyLevel`)
- Formatting logic lives in domain functions, not components
- No mappers needed — domain types match API response exactly
- Files use **kebab-case** naming (e.g. `get-by-id.ts`, `type.tsx`)
- **Per-operation DTOs**: Each API operation gets its own DTO type in its own file, matching the backend pattern. Even if two operations return the same shape, they must have separate types (e.g. `CreateModelDto` and `UpdateModelDto`). This prevents coupling — if one endpoint's response changes, others aren't affected.
- **Cross-cutting `User` type**: The auth context defines its own `User` interface in `context/auth-context.ts` for application state storage. This is separate from per-operation user DTOs in `models/user/`.

### Type Generation (`Specta`)

Frontend DTO contracts are generated from **application-layer DTOs** using Specta.

- Generator entrypoint: `backend/crates/api/src/bin/typegen_models.rs`
- Output directory: `frontend/src/models/__generated/`
- Generation command:

```bash
cd backend
cargo run -p rc-log-api --bin typegen_models
```

Rules:
- Never edit files in `frontend/src/models/__generated/` manually.
- For new/changed backend response DTOs, add `#[derive(specta::Type)]` to application DTO structs/enums, register them in the generator, then re-run generation.
- Keep request payload types and frontend-only helpers manual in wrapper files under `frontend/src/models/<entity>/`.
- Components/pages/hooks should continue importing from `@/models/<entity>` (wrapper/barrel), not from `@/models/__generated/*` directly.
- `context/auth-context.ts` `User` remains a manual cross-cutting state type; do not replace it with generated operation DTOs.

**Example — difficulty formatting:**
```typescript
// models/shared/difficulty.ts
export type DifficultyLevel = "level1" | "level2" | ... | "level7";

export function getDifficultyColor(difficulty: DifficultyLevel): string { ... }

export function getDifficultyLevelName(type: Type, difficulty: DifficultyLevel): string {
  // Returns "Beginner", "Basic 3D", etc.
}
```

**Example — vehicle icons:**
```typescript
// models/shared/type.tsx (JSX file for React component return)
export function getVehicleIcon(type: Type, size = 18): ReactNode {
  switch (type) {
    case "Plane": return <Plane size={size} />;
    case "Helicopter": return <Helicopter size={size} />;
    case "Drone": return <Drone size={size} />;
  }
}
```

### Authentication (`context/AuthContext.tsx`, `lib/api/auth.ts`)

**`AuthProvider`** wraps the whole app (in `App.tsx`). It persists `token` and `user` to `localStorage` and exposes them via `useAuth()`.

**`User` type** (`context/auth-context.ts`): defines a `User` interface for auth state storage (id, username, email, photoAssetName). This is separate from per-operation DTOs in `models/user/` — the auth context is a cross-cutting application concern.

**`useAuth()` returns:**
- `user: User | null` — deserialized from localStorage on init
- `token: string | null` — JWT string
- `isAuthenticated: boolean`
- `signIn(req)` / `signUp(req)` — call the API and store the returned token + user
- `signOut()` — clears localStorage and resets state
- `updateUser(user)` — updates the stored user (called after profile/photo updates)

**Axios interceptors (`lib/apiClient.ts`):**
- **Request**: attaches `Authorization: Bearer <token>` if a token is present in `localStorage`.
- **Response**: on 401, clears `token` and `user` from `localStorage` and redirects to `/sign-in`.

> **`localStorage` rule**: `localStorage` is accessed in two places only — `AuthContext.tsx` (read/write user state on sign-in/sign-out) and `lib/apiClient.ts` (read token in Axios interceptors, clear on 401). The apiClient exception is necessary because Axios interceptors cannot receive React context. No other file should access `localStorage` directly.

**`ProtectedRoute`** (`components/auth/ProtectedRoute.tsx`) — wraps any route element that requires auth; redirects to `/sign-in` when `isAuthenticated` is `false`.

**Routes:** `/sign-in` → `SignInPage`, `/sign-up` → `SignUpPage`.

The sidebar footer shows **Sign In / Register** buttons when logged out, and a **Sign Out** button with the username when logged in.

**Hook naming**: hooks use `camelCase` file names (e.g. `useAuth.ts`, `usePhotoPath.ts`). The `use-mobile.ts` exception is a shadcn/ui scaffold artifact — new hooks must use `camelCase`.

### API Layer (`lib/api/`)

**Principles:**
- Handles HTTP specifics (building query params, axios calls)
- Re-exports domain types for convenience
- Request/response types reference domain types directly (no duplication)

**Example:**
```typescript
// lib/api/maneuvers.ts
import type { ListManeuverDto, ListManeuverFilter, ListManeuverSort } from "@/models/maneuver";

export interface ListManeuversRequest extends PaginationOptions {
  filter?: ListManeuverFilter;
  sort?: ListManeuverSort;
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
- `type` → `type`
- `page_size` → `pageSize`
- `defaultVariationVideoAssetName` (in list response)
- `defaultVariation` / `variations` (in get-by-id response)
- `total_pages` → `totalPages`

Difficulty serializes as lowercase string (`level1`–`level7`), not integer.

### Adding a New Feature — Frontend Checklist

1. **Backend DTOs first**: Add/update response DTOs in `backend/crates/application/src/**/model.rs` and derive `specta::Type`.
2. **Regenerate contracts**: Run `cargo run -p rc-log-api --bin typegen_models` from `backend/`.
3. **Domain wrappers**: Update `frontend/src/models/<entity>/` wrapper files (re-exports, request payloads, formatting helpers).
4. **API layer**: Add/update request/response types in `lib/api/<entity>.ts`; import from `@/models/<entity>` wrappers.
5. **Components**: Use domain wrapper exports and formatting functions — never duplicate display logic.
6. **Auth-gated routes**: Wrap the route element in `<ProtectedRoute>` in `App.tsx`.
7. **Import rules**:
   - Components import from `@/models/maneuver` (or other entity) for types and formatting
   - API layer imports from `@/models/<entity>` for type references
  - Do not import from `@/models/__generated/*` outside wrapper modules
   - Never duplicate domain types in API layer

---

## Adding a New Feature — Checklist

1. **Domain**: add any new value objects / entity methods needed. For asset-type lookups define a `FooResolver` trait in `src/asset/foo_resolver.rs` following the `VideoResolver` / `PhotoResolver` pattern.
2. **Application**: create a nested directory for the use case (e.g., `src/<entity>/<use_case>/`). Inside, create `error.rs`, `model.rs` (DTOs + `From<DomainType>` impls), `use_case.rs`, and `mod.rs`. Use cases must return DTO types, not domain types. Add error variant to `ApplicationError`.
   - UoW-based use cases: `execute(&mut self, ...)` — begin transaction, call repository, commit.
   - Resolver-based use cases: `execute(&self, ...)` — inject resolver directly via generic `R: FooResolver`, no transaction needed.
3. **Persistence**: implement new `Transaction` methods on the sqlx transaction struct. For resolvers, implement the domain resolver trait (`VideoResolver`, `PhotoResolver`) on the concrete sqlx resolver struct.
4. **API**: add `src/<entity>/response.rs` — define a **distinct response struct** per endpoint. It may embed application DTOs directly as fields (they derive `Serialize`). Do not import from `rc_log_domain` directly.
5. **Import rules**:
   - `application` must never import from `persistance`
   - `api` may import application DTOs but must wrap them in its own named response types — never return a raw DTO from a handler
   - Only `api` (composition root) wires concrete infrastructure into use cases
6. **Code Style**:
   - Always prefer explicit `use` declarations (e.g., `use std::env;`) instead of inline fully-qualified paths (`std::env::var()`), unless doing so creates severe ambiguity. Expand this preference across the entire backend workspace.
   - Each API operation lives in its own subdirectory under the entity: `src/<entity>/<operation>/` with `extractor.rs`, `handler.rs`, `response.rs`, `mod.rs`.
   - **Extractor rule**: every handler parameter beyond `State` and `AuthenticatedUser` must be a named extractor struct that implements `FromRequest` (when a body or multipart is involved) or `FromRequestParts` (path/headers only). No inline `Path<Uuid>`, `Json<Body>`, or `Multipart` in handler signatures. The extractor owns all validation — the handler is a thin orchestrator that calls the use case and wraps the result.
   - `DifficultyLevel` serializes as lowercase strings (`level1`–`level7`) from the backend.
   - `lib.rs` in each crate only declares top-level modules with `pub mod`, never re-exports directly.
   - `mod.rs` in each operation subdirectory re-exports the use case struct: `pub use use_case::FooUseCase`.
   - Asset paths stored in DB are relative to `RC_LOG_ASSET_PATH` (e.g. `videos/foo_small.mp4`). Frontend clients prepend `/api/assets/` to get the full URL served by `ServeDir`.

---

## Value Objects vs Primitive Types — Where Each Belongs

This is a critical architectural rule. The wrong type at the wrong layer creates leaky abstractions or bypasses validation.

| Layer | Type to Use | Reason |
|---|---|---|
| **Domain** | Domain value objects (`ModelId`, `UserId`, `Name`, `AssetName`, `Type`, etc.) | Value objects carry validation guarantees; domain logic is expressed in domain types |
| **Application (use case inputs)** | Primitive types (`Uuid`, `String`, `u32`) | Inputs come from the API — no domain objects cross the application/API boundary |
| **Application (use case internals)** | Domain value objects | Conversion from primitive → value object happens inside the use case (validation point) |
| **Application (DTOs / outputs)** | Primitive types + application enums (`TypeDto`) | Outputs are stable contracts for the API; no domain types escape the use case |
| **Persistence** | Domain value objects as function parameters | Repository trait methods accept domain types (e.g. `ModelId`, `UserId`) — never raw `&Uuid` |
| **API** | Primitive types only | API layer never sees domain crate types; use application DTOs and primitive Uuid/String |

**Key rule**: Primitive `Uuid` is used in application input/output DTOs (crossing the app/API boundary). Domain value objects (`ModelId`, `UserId`, etc.) are only instantiated *inside* the application use case after validation, and passed into persistence.

**Type bridging**: `Type` (domain enum) ↔ `TypeDto` (application enum, serializable). Conversion happens in use case internals (`match input.type { TypeDto::Helicopter => Type::Helicopter, ... }`). API layer uses `rc_log_application::model::shared::TypeDto`. Never use domain `Type` in API.

**Shared DTO rule (application layer)**: share DTOs only inside the same bounded context when they are truly invariant across operations. Current shared modules are `model/shared/type.rs`, `maneuver/shared/difficulty.rs`, and `session/shared/rating.rs`. When uncertain, keep per-operation DTOs duplicated to avoid coupling endpoints that may diverge.

---

## Ownership Check Pattern (User-Owned Resources)

For aggregates owned by a user (e.g. `Model`), use cases that access a single resource by ID must verify that the authenticated user is the owner. The standard pattern:

```rust
// In use_case.rs (get_by_id, update, delete)
let entity = tx.get_by_id(EntityId::new(input.id)).await.map_err(Error::from)?
    .ok_or(Error::NotFound)?;

if Uuid::from(entity.owner_id()) != input.owner_id {
    tx.rollback().await.map_err(Error::from)?;
    return Err(Error::Forbidden.into());
}
```

- **Returns `Forbidden` (403)**, not `NotFound` — the resource exists but the caller doesn't own it
- **Always rollback** before returning `Forbidden` to avoid leaving open transactions
- `owner_id` in the input comes from the JWT claims (`auth.id` in the handler), never from the request path or body
- `delete` use case: get entity → check ownership → `delete_by_id` → commit (no response body; handler returns 204)
- `update` use case: get entity → check ownership → create new entity with same id/owner → `save` (upsert) → commit
