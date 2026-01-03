# Magic37 Rust Microservices

This repository contains the Rust-based microservices for the Magic37 project.

## Project Structure

```
├── skaffold.yaml                 # Core Skaffold configuration (Dev/Prod)
├── skaffold.local.yaml           # Local Development Skaffold configuration (Includes Postgres)
└── services/
    ├── auth-provider/            # Auth Provider Service
    │   ├── code/                 # Rust Source Code
    │   ├── chart/                # Helm Chart
    │   └── skaffold.yaml         # Service Module Config
    └── postgres/                 # Local PostgreSQL Service
        ├── chart/                # Helm Chart
        └── skaffold.yaml         # Service Module Config
```

## Development Workflow

We use **Skaffold** for orchestrating the development workflow.

### 1. Local Development (Recommended)
 To run the entire stack locally, including a dedicated PostgreSQL instance:

```bash
skaffold run -f skaffold.local.yaml -p local
```
*   **Active Modules**: `auth-provider`, `postgres`
*   **Environment**: Uses `values-local.yaml` for Helm charts.
*   **Postgres**: Deploys a local Postgres container and connects `auth-provider` to it.

### 2. Standard Development / Production
To run only the core services (assuming an external database or strictly testing the service logic):

```bash
skaffold run -p dev   # or -p prod
```
*   **Active Modules**: `auth-provider`
*   **Environment**: Uses `values-dev.yaml` or `values-prod.yaml`.
*   **Postgres**: **NOT** deployed. The service expects an external database connection (managed via secrets or external config).

## Database Migrations

Migrations are managed via **Flyway** and executed as a Helm Hook (`pre-install`, `pre-upgrade`).

*   **Location**: `services/auth-provider/chart/migrations/`
*   **Adding Migrations**: Simply add standard SQL files (e.g., `V2__description.sql`) to the directory above.
*   **Execution**:
    1.  Helm packages the SQL files into a ConfigMap.
    2.  A Kubernetes Job runs `flyway/flyway` to apply pending migrations before the application starts.

## Build System
The project uses **Cloud Native Buildpacks** (via Paketo) to build container images, removing the need for local `Dockerfile` maintenance.
