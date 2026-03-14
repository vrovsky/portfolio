# Portfolio Backend

Rust + Axum REST API serving portfolio project data.

## Quick Start

```bash
cargo run
```

The API will be available at: `http://localhost:3001`

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/projects` | Returns all projects as JSON |

### Example Response

```json
[
  {
    "id": 1,
    "title": "Project Name",
    "description": "Project description",
    "technologies": ["React", "Node.js"],
    "github_url": "https://github.com/...",
    "live_url": "https://...",
    "image": "/image.jpg"
  }
]
```

## Commands

| Command | Description |
|---------|-------------|
| `cargo run` | Run development server |
| `cargo build --release` | Build for production |
| `cargo check` | Check for errors without building |

## Project Structure

```
src/
├── main.rs           # Entry point with CORS configuration
├── models.rs         # Project data structures
├── routes.rs         # API route handlers
└── data/
    └── projects.json # Project data source
```

## Tech Stack

- [Rust](https://www.rust-lang.org/)
- [Axum](https://github.com/tokio-rs/axum)
- [Tokio](https://tokio.rs/)
- [Serde](https://serde.rs/)
- [Tower HTTP](https://github.com/tower-rs/tower-http)

## Configuration

The backend runs on port `3001` by default. This is configured in [`main.rs`](src/main.rs):

```rust
let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
```
