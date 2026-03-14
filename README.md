# Viachaslau Rouski - Portfolio

A full-stack portfolio website featuring a SvelteKit frontend and Rust backend API.

## Architecture

```
portfolio/
├── portfolio-frontend/     # SvelteKit + Tailwind CSS + Three.js
└── portfolio-backend/      # Rust + Axum API
```

| Component | Technology | Port | Description |
|-----------|------------|------|-------------|
| **Frontend** | SvelteKit 5, Tailwind CSS 4, Three.js | 5173 | Interactive portfolio with 3D animations |
| **Backend** | Rust, Axum | 3001 | REST API serving project data |

---

## Quick Start

### Prerequisites

- **Frontend**: Node.js 20+, npm
- **Backend**: Rust (latest stable), Cargo

### Backend Setup

```bash
cd portfolio-backend

# Run the backend
cargo run
```

The backend API will be available at: `http://localhost:3001`

**API Endpoint:**
- `GET /projects` - Returns all projects as JSON

### Frontend Setup

```bash
cd portfolio-frontend

# Install dependencies
npm install

# Create environment file
cp .env.example .env

# Start development server
npm run dev
```

The frontend will be available at: `http://localhost:5173`

---

## Available Commands

### Backend

```bash
cd portfolio-backend

cargo run            # Run development server
cargo build --release # Build for production
cargo check          # Check for errors
```

### Frontend

```bash
cd portfolio-frontend

npm run dev          # Start development server
npm run build        # Build for production
npm run preview      # Preview production build
npm run check        # Type-check
npm run lint         # Lint code
npm run format       # Format code
```

---

## Tech Stack

### Frontend
- [SvelteKit](https://kit.svelte.dev/) - Web framework
- [Tailwind CSS](https://tailwindcss.com/) - Styling
- [Three.js](https://threejs.org/) - 3D graphics
- [GSAP](https://greensock.com/gsap/) - Animations

### Backend
- [Rust](https://www.rust-lang.org/) - Programming language
- [Axum](https://github.com/tokio-rs/axum) - Web framework
- [Tokio](https://tokio.rs/) - Async runtime

---

## License

MIT License
