# Development Guide

> How to run, build, and develop the Recipe App

## Prerequisites

### Required Software

- **Node.js** v18+ and npm (for frontend)
- **Rust** 1.70+ and Cargo (for backend)
- **Git** (for version control)

### Recommended Tools

- **VS Code** with extensions:
  - Svelte for VS Code
  - rust-analyzer
  - ESLint
  - Prettier
- **Postman** or **Insomnia** (for API testing)
- **jq** (for pretty-printing JSON in terminal)

### Optional

- **Docker** (for containerized deployment - not yet configured)

## Quick Start

### 1. Clone the Repository

```bash
git clone <repository-url>
cd copilot-bootcamp-capstone
```

### 2. Start the Backend

```bash
cd packages/backend
cargo build
cargo run
```

**Expected Output**:
```
✅ Seeded database with 9 recipes
🚀 Server running on http://127.0.0.1:3001
```

The server will:
- Create `recipes.db/` database directory
- Seed initial recipes from embedded `recipes.json`
- Start listening on port 3001

**Verify it works**:
```bash
curl http://127.0.0.1:3001/recipes | jq '.[0:2]'
```

### 3. Start the Frontend

In a **new terminal**:

```bash
cd packages/frontend
npm install
npm run dev
```

**Expected Output**:
```
VITE v5.x.x  ready in xxx ms

➜  Local:   http://localhost:5173/
➜  Network: use --host to expose
```

### 4. Open the App

Navigate to `http://localhost:5173` in your browser.

You should see:
- A header with search bar
- A sidebar with navigation
- Recipe cards in a grid
- Favorites section (if any recipes marked as favorites)

## Project Structure

```
copilot-bootcamp-capstone/
├── .github/
│   ├── agents/                 # Custom Copilot agents
│   │   └── teacher.agent.md
│   └── copilot-instructions.md # Project-wide Copilot context
├── docs/
│   ├── architecture.md         # System architecture (THIS IS NEW!)
│   ├── api.md                  # API documentation (THIS IS NEW!)
│   ├── frontend-guide.md       # Frontend patterns (THIS IS NEW!)
│   ├── development.md          # This file (THIS IS NEW!)
│   ├── teacher-logs.md         # Teaching session history
│   └── coding-guideline.md     # Code standards
├── packages/
│   ├── backend/                # Rust API server
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   └── main.rs
│   │   ├── recipes.db/         # Created at runtime (gitignored)
│   │   └── target/             # Build artifacts (gitignored)
│   └── frontend/               # SvelteKit app
│       ├── package.json
│       ├── svelte.config.js
│       ├── vite.config.ts
│       ├── src/
│       │   ├── routes/
│       │   ├── lib/
│       │   └── resources/
│       └── node_modules/       # Dependencies (gitignored)
├── LICENSE
└── README.md
```

## Development Workflows

### Backend Development

#### Running the Server

```bash
cd packages/backend
cargo run
```

Server runs in foreground. Press `Ctrl+C` to stop.

#### Watch Mode (Auto-Restart)

Install `cargo-watch`:
```bash
cargo install cargo-watch
```

Run with auto-reload:
```bash
cargo watch -x run
```

Now the server restarts whenever you edit `src/main.rs`.

#### Running Tests

```bash
cargo test
```

⚠️ **Note**: No tests implemented yet.

#### Checking for Errors

```bash
cargo check       # Fast compile check without building
cargo clippy      # Lint with Clippy
cargo fmt         # Format code
```

#### Building for Release

```bash
cargo build --release
./target/release/backend
```

Release builds are optimized (slower to compile, faster to run).

#### Resetting the Database

```bash
# Stop the server first (Ctrl+C)
rm -rf recipes.db
cargo run
# Database will re-seed from recipes.json
```

#### Viewing Database Contents

**Option 1: Use the API**
```bash
curl http://127.0.0.1:3001/recipes | jq '.'
```

**Option 2: Create a Debug Binary**

Create `src/bin/view_db.rs`:
```rust
fn main() {
    let db = sled::open("recipes.db").unwrap();
    
    println!("Database contains {} items\n", db.len());
    
    for item in db.iter() {
        if let Ok((key, value)) = item {
            let key_str = String::from_utf8_lossy(&key);
            let val_str = String::from_utf8_lossy(&value);
            println!("Key: {}", key_str);
            println!("Value: {}\n", val_str);
        }
    }
}
```

Run it:
```bash
cargo run --bin view_db
```

### Frontend Development

#### Running the Dev Server

```bash
cd packages/frontend
npm run dev
```

Server runs with hot module replacement. Changes auto-reload in browser.

#### Type Checking

```bash
npm run check
```

Validates TypeScript and Svelte syntax without building.

#### Building for Production

```bash
npm run build
```

Outputs to `.svelte-kit/output/`.

#### Preview Production Build

```bash
npm run preview
```

Serves the production build locally for testing.

#### Linting

```bash
npm run lint          # Check for issues
npm run lint -- --fix # Auto-fix issues
```

#### Running Tests

```bash
npm run test          # Run once
npm run test:watch    # Watch mode
npm run test:ui       # Vitest UI
```

#### Adding Dependencies

```bash
npm install <package>           # Production dependency
npm install -D <package>        # Dev dependency
```

### Full Stack Development

#### Starting Both Servers

**Terminal 1 (Backend)**:
```bash
cd packages/backend
cargo watch -x run
```

**Terminal 2 (Frontend)**:
```bash
cd packages/frontend
npm run dev
```

#### Testing the Integration

1. **Backend health check**:
   ```bash
   curl http://127.0.0.1:3001/recipes
   ```

2. **Frontend**: Open `http://localhost:5173` and verify recipes load

3. **Cross-origin requests**: Check browser console for CORS errors

#### Common Issues

**Frontend can't reach backend**:
- ✅ Backend running on port 3001?
- ✅ Frontend fetching from `http://127.0.0.1:3001/recipes`?
- ✅ CORS enabled in backend?

**Recipes not loading**:
- Check browser console (F12) for errors
- Check backend logs for database errors
- Verify backend returns data: `curl http://127.0.0.1:3001/recipes`

**Port already in use**:
```bash
# Find process using port
lsof -i :3001
lsof -i :5173

# Kill process
kill -9 <PID>
```

## Code Quality Tools

### TypeScript

Frontend uses strict TypeScript:
```json
{
  "compilerOptions": {
    "strict": true,
    "noImplicitAny": true,
    "strictNullChecks": true
  }
}
```

Run type checker:
```bash
cd packages/frontend
npm run check
```

### Rust

Backend follows Rust conventions:

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Fix clippy warnings
cargo clippy --fix
```

### Git Hooks

⚠️ **Not yet configured**, but recommended:

Install **husky** and **lint-staged** for pre-commit hooks:

```bash
npm install -D husky lint-staged
npx husky install
```

Configure pre-commit hook to run linters and type checks.

## Environment Variables

### Backend

⚠️ **Not currently used**, but can be added:

```bash
export PORT=3001
export DATABASE_PATH=./recipes.db
cargo run
```

Update `main.rs` to read from env vars:
```rust
let port = std::env::var("PORT").unwrap_or("3001".to_string());
let db_path = std::env::var("DATABASE_PATH").unwrap_or("recipes.db".to_string());
```

### Frontend

Create `.env` file:
```bash
VITE_API_URL=http://127.0.0.1:3001
```

Use in code:
```typescript
const API_URL = import.meta.env.VITE_API_URL || "http://127.0.0.1:3001";
const response = await fetch(`${API_URL}/recipes`);
```

## Debugging

### Backend Debugging

**Print debugging**:
```rust
println!("DEBUG: Recipe ID = {}", recipe.id);
eprintln!("ERROR: Failed to load recipe");
```

**Using rust-analyzer in VS Code**:
1. Install rust-analyzer extension
2. Open `main.rs`
3. Click "Debug" above `#[tokio::main]`
4. Set breakpoints, inspect variables

**Logging with `tracing`**:

Add to `Cargo.toml`:
```toml
tracing = "0.1"
tracing-subscriber = "0.3"
```

In `main.rs`:
```rust
use tracing::{info, warn, error};

tracing_subscriber::fmt::init();

info!("Server starting");
warn!("Database empty, seeding...");
error!("Failed to connect: {}", err);
```

### Frontend Debugging

**Browser DevTools**:
1. Open DevTools (F12)
2. Console: See `console.log()` output and errors
3. Network: Inspect API requests/responses
4. Svelte DevTools extension: Inspect component state

**VS Code Debugging**:
1. Install "Svelte for VS Code" extension
2. Add breakpoint in `.svelte` file
3. Run "Debug: Start Debugging" (F5)

**Reactive State Inspection**:
```svelte
<script>
let items = $state<Recipe[]>([]);

// Log whenever items changes
$effect(() => {
    console.log("Items updated:", items);
});
</script>
```

## Performance Profiling

### Frontend

**Chrome DevTools Performance**:
1. Open DevTools → Performance tab
2. Click Record
3. Interact with app (scroll, drag-drop)
4. Stop recording
5. Analyze flame graph for bottlenecks

**Lighthouse**:
1. Open DevTools → Lighthouse tab
2. Run audit
3. Review scores for Performance, Accessibility, Best Practices

### Backend

**Basic profiling**:
```rust
use std::time::Instant;

let start = Instant::now();
// ... code to profile ...
println!("Operation took {:?}", start.elapsed());
```

**Flamegraph** (Linux/macOS):
```bash
cargo install flamegraph
cargo flamegraph
# Generates flamegraph.svg
```

## Building for Production

### Backend

```bash
cd packages/backend
cargo build --release
```

Binary output: `target/release/backend`

**Run production build**:
```bash
./target/release/backend
```

**Deploy considerations**:
- Set `DATABASE_PATH` to persistent storage location
- Configure CORS to allow only production frontend domain
- Use reverse proxy (nginx) for HTTPS
- Set up systemd service for auto-restart

### Frontend

```bash
cd packages/frontend
npm run build
```

Output: `.svelte-kit/output/`

**Preview**:
```bash
npm run preview
```

**Deploy**:
- **Vercel**: `vercel deploy`
- **Netlify**: `netlify deploy`
- **Static hosting**: Upload `.svelte-kit/output/client/`
- **Node.js server**: Run `.svelte-kit/output/server/`

## Deployment Checklist

- [ ] Update CORS to allow only production frontend domain
- [ ] Set up environment variables (API URL, database path)
- [ ] Configure HTTPS/SSL certificates
- [ ] Set up logging and monitoring
- [ ] Configure database backups
- [ ] Test API endpoints in production
- [ ] Run Lighthouse audit on frontend
- [ ] Set up error tracking (Sentry, etc.)
- [ ] Configure CI/CD pipeline
- [ ] Document deployment process

## Troubleshooting

### "Cannot find module" errors

```bash
# Frontend
cd packages/frontend
rm -rf node_modules package-lock.json
npm install

# Backend
cd packages/backend
cargo clean
cargo build
```

### Type errors in Svelte files

```bash
# Restart Svelte language server in VS Code
Ctrl+Shift+P → "Svelte: Restart Language Server"

# Or run type check
npm run check
```

### CORS errors in browser

Ensure backend has CORS configured:
```rust
let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods(Any)
    .allow_headers(Any);
```

### Database locked errors

Stop all running backend instances:
```bash
pkill backend
rm -rf recipes.db
cargo run
```

### Port conflicts

```bash
# Check what's using the port
lsof -i :3001
lsof -i :5173

# Kill the process
kill -9 <PID>
```

## Learning Resources

### Svelte 5
- [Svelte 5 Docs](https://svelte.dev/docs/svelte/overview)
- [Svelte 5 Migration Guide](https://svelte.dev/docs/svelte/v5-migration-guide)
- [SvelteKit Docs](https://kit.svelte.dev/docs)

### Rust
- [Rust Book](https://doc.rust-lang.org/book/)
- [Axum Docs](https://docs.rs/axum/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Sled Docs](https://docs.rs/sled/)

### Project-Specific
- [teacher-logs.md](./teacher-logs.md) - Teaching sessions explaining implementation details
- [architecture.md](./architecture.md) - System design and decisions
- [api.md](./api.md) - Backend API reference
- [frontend-guide.md](./frontend-guide.md) - Frontend patterns and best practices

---

*Last updated: 2026-03-31 | Source: Project setup and teacher session logs*
