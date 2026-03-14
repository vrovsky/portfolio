# Portfolio Frontend

SvelteKit 5 portfolio frontend with Tailwind CSS, Three.js animations, and GSAP.

## Quick Start

```bash
# Install dependencies
npm install

# Create environment file
cp .env.example .env

# Start development server
npm run dev
```

## Configuration

### Environment Variables

Create a `.env` file in this directory:

```env
# Backend API URL
# Development: http://localhost:3001
# Production: https://your-backend.up.railway.app
PUBLIC_API_URL=http://localhost:3001
```

The frontend will fallback to static JSON data if the backend is unavailable.

## Scripts

| Command           | Description                                      |
| ----------------- | ------------------------------------------------ |
| `npm run dev`     | Start development server (http://localhost:5173) |
| `npm run build`   | Build for production                             |
| `npm run preview` | Preview production build                         |
| `npm run check`   | Type-check with svelte-check                     |
| `npm run lint`    | Lint code                                        |
| `npm run format`  | Format code with Prettier                        |

## Project Structure

```
src/
├── lib/
│   ├── components/
│   │   ├── BackgroundAnimations.svelte  # Three.js background
│   │   ├── MouseTrail.svelte           # Custom cursor trail
│   │   └── ProjectCard.svelte          # Project display card
│   ├── data/
│   │   └── projects.json               # Fallback project data
│   └── types.ts                        # TypeScript interfaces
├── routes/
│   ├── +layout.svelte                  # Global layout
│   ├── +page.svelte                    # Home page
│   ├── +page.ts                        # Home page data loader
│   ├── projects/
│   │   ├── +page.svelte                # Projects page
│   │   └── +page.ts                    # Projects data loader
│   └── layout.css                      # Global styles
└── app.html                            # HTML template
```

## Features

- **3D Background**: Interactive Three.js particle animations
- **Mouse Trail**: Custom cursor effect with GSAP
- **Responsive Design**: Mobile-first with Tailwind CSS
- **Type Safety**: Full TypeScript support
- **Fallback Data**: Static JSON when backend unavailable

### Netlify Configuration

See [`netlify.toml`](netlify.toml) for build settings.

## Tech Stack

- [SvelteKit](https://kit.svelte.dev/) - Framework
- [Svelte 5](https://svelte.dev/) - UI language
- [Tailwind CSS 4](https://tailwindcss.com/) - Styling
- [Three.js](https://threejs.org/) - 3D graphics
- [GSAP](https://greensock.com/gsap/) - Animations
- [TypeScript](https://www.typescriptlang.org/) - Type safety
