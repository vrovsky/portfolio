Frontend: portfolio-frontendThis is scaffolded via npx sv create portfolio-frontend with TypeScript and Tailwind add-ons. It includes:Home page with intro, skills table, and animated "play game" (upgraded color picker with spring physics).
Projects page with dynamic fetching and animated cards.
Global layout for header/footer.

Directory Tree

portfolio-frontend/
├── package.json
├── package-lock.json
├── svelte.config.js
├── tsconfig.json
├── vite.config.ts
├── postcss.config.js
├── tailwind.config.js
├── src/
│   ├── app.html
│   ├── app.d.ts
│   ├── routes/
│   │   ├── +layout.svelte
│   │   ├── +page.svelte
│   │   └── projects/
│   │       ├── +page.ts
│   │       └── +page.svelte
│   ├── lib/
│   │   ├── types.ts
│   │   ├── components/
│   │   │   └── ProjectCard.svelte
│   │   └── data/
│   │       └── projects.json  (fallback static data)
│   └── hooks.server.ts  (optional, empty for now)
├── static/
│   ├── favicon.ico
│   ├── free-scribe.png  (example; add your images)
│   ├── droplify.png
│   ├── calendoor.png
│   ├── swolefit.png
│   └── pumpgram.png
└── .gitignore

File Contentspackage.jsonjson

{
  "name": "portfolio-frontend",
  "version": "0.0.1",
  "private": true,
  "scripts": {
    "dev": "vite dev",
    "build": "vite build",
    "preview": "vite preview",
    "check": "svelte-kit sync && svelte-check --tsconfig ./tsconfig.json",
    "check:watch": "svelte-kit sync && svelte-check --tsconfig ./tsconfig.json --watch"
  },
  "devDependencies": {
    "@sveltejs/adapter-auto": "^3.0.0",
    "@sveltejs/kit": "^2.0.0",
    "@sveltejs/vite-plugin-svelte": "^3.0.0",
    "autoprefixer": "^10.4.19",
    "postcss": "^8.4.38",
    "svelte": "^5.0.0-next.1",
    "svelte-check": "^3.6.0",
    "tailwindcss": "^3.4.1",
    "tslib": "^2.4.1",
    "typescript": "^5.0.0",
    "vite": "^5.0.0"
  },
  "type": "module"
}

svelte.config.jsjavascript

import adapter from '@sveltejs/adapter-auto';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),
  kit: {
    adapter: adapter()
  }
};

export default config;

tsconfig.jsonjson

{
  "extends": "./.svelte-kit/tsconfig.json",
  "include": ["src/**/*.d.ts", "src/**/*.js", "src/**/*.ts", "src/**/*.svelte"]
}

vite.config.tstypescript

import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
  plugins: [sveltekit()]
});

postcss.config.jsjavascript

export default {
  plugins: {
    tailwindcss: {},
    autoprefixer: {}
  }
};

tailwind.config.jsjavascript

/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {}
  },
  plugins: []
};

src/app.htmlhtml

<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    %sveltekit.head%
    <title>Viachaslau Rouski Portfolio</title>
    <link rel="icon" href="/favicon.ico" type="image/x-icon" />
  </head>
  <body data-sveltekit-preload-data="hover">
    <div style="display: contents">%sveltekit.body%</div>
  </body>
</html>

src/app.d.tstypescript

// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
  namespace App {
    // interface Error {}
    // interface Locals {}
    // interface PageData {}
    // interface PageState {}
    // interface Platform {}
  }
}

export {};

src/routes/+layout.sveltesvelte

<script lang="ts">
  import '../app.css'; // Global styles if needed
</script>

<header class="bg-gray-800 text-white p-4 flex justify-between items-center">
  <a href="/" class="text-xl font-bold">Viachaslau Rouski Portfolio</a>
  <nav class="space-x-4">
    <a target="_blank" href="https://github.com/vrovsky"><i class="fab fa-github"></i></a>
    <a target="_blank" href="https://www.linkedin.com/in/viachaslau-rouski-58840521a/"><i class="fab fa-linkedin"></i></a>
  </nav>
</header>

<slot />

<footer class="bg-gray-800 text-white p-4 text-center">
  <p>By <a href="https://www.linkedin.com/in/viachaslau-rouski-58840521a/">Vrovsky</a></p>
</footer>

<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.1/css/all.min.css" />

src/routes/+page.sveltesvelte

<script lang="ts">
  import { spring } from 'svelte/motion';
  import { fade } from 'svelte/transition';

  let showGame = false;
  let selectedColor = '#0084ff';
  let hoverScale = spring(1, { stiffness: 0.2, damping: 0.4 });
  let ruined = false;

  function playGame() {
    showGame = true;
  }

  function changeColor() {
    document.body.style.background = selectedColor;
    ruined = true;
  }

  function resetColor() {
    document.body.style.background = 'white';
    showGame = false;
    ruined = false;
  }
</script>

<section class="p-8">
  <div class="text-center">
    <h1 class="text-4xl font-bold">Greetings! I'm Viachaslau,</h1>
    <p class="text-2xl">Software Developer</p>
  </div>
  <p class="mt-4 text-lg">
    A radiobiology engineer by qualification, and a software developer by trade. Based in Lisbon, Portugal. I love to surf, to train and build out all sorts of fun and useful applications!
  </p>
  <div class="flex space-x-4 mt-4">
    <a href="mailto:vrovsky@gmail.com" class="text-blue-500">Email me</a>
    <a target="_blank" href="https://github.com/vrovsky" class="text-blue-500">GitHub</a>
    <a target="_blank" href="https://www.linkedin.com/in/viachaslau-rouski-58840521a/" class="text-blue-500">LinkedIn</a>
  </div>
  <div id="play-game" class="mt-4">
    <button 
      on:click={playGame}
      on:mouseenter={() => hoverScale.set(1.05)}
      on:mouseleave={() => hoverScale.set(1)}
      style="transform: scale({$hoverScale});"
      class="bg-blue-500 text-white px-4 py-2 rounded"
    >Play game???</button>
  </div>
  {#if showGame}
    <div transition:fade class="mt-4 p-4 border rounded">
      <h4>Pick a color</h4>
      {#if ruined}
        <p transition:fade>Great... now it's ruined!</p>
        <button on:click={resetColor} class="bg-red-500 text-white px-4 py-2 rounded">Reset</button>
      {:else}
        <input type="color" bind:value={selectedColor} on:change={changeColor} />
      {/if}
    </div>
  {/if}
</section>

<section class="p-8">
  <h2 class="text-3xl font-bold">Skills</h2>
  <table class="table-auto w-full mt-4 border-collapse border">
    <thead>
      <tr>
        <th class="border p-2">Area</th>
        <th class="border p-2">Technologies & Skills</th>
      </tr>
    </thead>
    <tbody>
      <tr>
        <td class="border p-2">Tech Stack</td>
        <td class="border p-2">Next.js | Node.js | TypeScript | React | Solidity | PostgreSQL | AWS | Docker | Grafana | Cypress | Selenium | Gherkin | Rust | Axum</td>
      </tr>
      <tr>
        <td class="border p-2">Leadership & Soft Skills</td>
        <td class="border p-2">Team Management | Cross-functional Collaboration | Strong Communication | Leadership</td>
      </tr>
      <tr>
        <td class="border p-2">Development Methodologies</td>
        <td class="border p-2">Agile | Scrum | Kanban</td>
      </tr>
      <tr>
        <td class="border p-2">Project Management</td>
        <td class="border p-2">JIRA | Trello | Miro</td>
      </tr>
      <tr>
        <td class="border p-2">Languages</td>
        <td class="border p-2">English — Fluent | Belarussian — Native | Russian — Native</td>
      </tr>
    </tbody>
  </table>
</section>

src/routes/projects/+page.tstypescript

import type { PageLoad } from './$types';
import type { Project } from '$lib/types';

export const load: PageLoad = async ({ fetch }) => {
  try {
    const res = await fetch('http://localhost:3001/projects'); // Replace with env var for prod
    if (res.ok) {
      const projects: Project[] = await res.json();
      return { projects };
    }
  } catch (e) {
    console.error(e);
  }
  // Fallback to static JSON
  const projects = (await import('$lib/data/projects.json')).default as Project[];
  return { projects };
};

src/routes/projects/+page.sveltesvelte

<script lang="ts">
  import { fade, scale } from 'svelte/transition';
  import { spring } from 'svelte/motion';
  import type { Project } from '$lib/types';
  import ProjectCard from '$lib/components/ProjectCard.svelte';

  export let data: { projects: Project[] };
  let hoverScale = spring(1, { stiffness: 0.2, damping: 0.4 });
</script>

<section class="p-8">
  <h2 class="text-3xl font-bold">Latest Pet Projects</h2>
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mt-4">
    {#each data.projects as project}
      <div 
        in:fade={{ duration: 500 }} 
        in:scale={{ duration: 300, opacity: 0.5 }}
        on:mouseenter={() => hoverScale.set(1.05)}
        on:mouseleave={() => hoverScale.set(1)}
        style="transform: scale({$hoverScale});"
      >
        <ProjectCard {project} />
      </div>
    {/each}
  </div>
</section>

src/lib/types.tstypescript

export interface Project {
  name: string;
  description: string;
  github: string;
  live?: string;
  tech: string[];
  image?: string;
}

src/lib/components/ProjectCard.sveltesvelte

<script lang="ts">
  import type { Project } from '$lib/types';

  export let project: Project;
</script>

<div class="border p-4 rounded shadow bg-white">
  <h4 class="text-xl font-bold">{project.name}</h4>
  <div class="flex flex-wrap gap-2 mt-2">
    {#each project.tech as tech}
      <span class="bg-gray-200 px-2 py-1 rounded text-sm">{tech}</span>
    {/each}
  </div>
  <p class="mt-2">{project.description}</p>
  <div class="mt-2">
    {#if project.live}
      <a target="_blank" href={project.live} class="text-blue-500 mr-2">Live</a>
    {/if}
    <a target="_blank" href={project.github} class="text-blue-500">GitHub</a>
  </div>
  {#if project.image}
    <img src={project.image} alt="{project.name} thumbnail" class="mt-2 w-full h-auto" />
  {/if}
</div>

src/lib/data/projects.jsonjson

[
  {
    "name": "Free-Scribe",
    "description": "Free AI-powered tool to transcribe and translate audio recordings and audio files.",
    "github": "https://github.com/vrovsky/free-scribe",
    "live": "https://freescriber.netlify.app/",
    "tech": ["ReactJS", "TailwindCSS", "Xenova/transformers"],
    "image": "/free-scribe.png"
  },
  {
    "name": "Droplify",
    "description": "Dropbox clone monorep",
    "github": "https://github.com/vrovsky/droplify",
    "tech": ["NextJS", "Clerk", "Neon", "Drizzle ORM", "ImageKit"],
    "image": "/droplify.png"
  },
  {
    "name": "Calendoor",
    "description": "Calendly clone monorep",
    "github": "https://github.com/vrovsky/calendoor",
    "tech": ["NextJS", "Clerk", "Neon", "Drizzle ORM"],
    "image": "/calendoor.png"
  },
  {
    "name": "SwoleFit",
    "description": "Create your own unique training plan for your fitness goals",
    "github": "https://github.com/vrovsky/swoley-fit-reactjs",
    "live": "https://swoley-portfolio.netlify.app",
    "tech": ["ReactJS", "TailwindCSS", "Vite"],
    "image": "/swolefit.png"
  },
  {
    "name": "Pumpgram",
    "description": "Simple weight counter for the gym bros and siss",
    "github": "https://github.com/vrovsky/pumpgram",
    "live": "https://pumpgram.netlify.app/",
    "tech": ["ReactJS", "FantaCSS", "Vite"],
    "image": "/pumpgram.png"
  },
  {
    "name": "r_wp",
    "description": "Rust-based web parser project",
    "github": "https://github.com/vrovsky/r_wp",
    "tech": ["Rust", "Reqwest"]
  },
  {
    "name": "tp_codec",
    "description": "TypeScript codec implemented in Rust",
    "github": "https://github.com/vrovsky/tp_codec",
    "tech": ["Rust", "TypeScript"]
  }
]

src/hooks.server.tstypescript

// Optional: Add server-side hooks here if needed (e.g., for auth)
export const handle = async ({ event, resolve }) => {
  return resolve(event);
};

.gitignore

node_modules
.svelte-kit
build
.env

Backend: portfolio-backendScaffolded via cargo new portfolio-backend --bin. Dependencies added: axum, tokio, serde. Serves /projects with data from JSON.Directory Tree

portfolio-backend/
├── Cargo.toml
├── Cargo.lock
├── src/
│   ├── main.rs
│   ├── routes.rs
│   ├── models.rs
│   └── data/
│       └── projects.json
└── .gitignore

File ContentsCargo.tomltoml

[package]
name = "portfolio-backend"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.7.5"
tokio = { version = "1.37.0", features = ["full"] }
serde = { version = "1.0.197", features = ["derive"] }
serde_json = "1.0.115"

src/main.rsrust

use axum::{Router, Server};
use std::net::SocketAddr;
use tokio::main;

mod models;
mod routes;

#[main]
async fn main() {
    let app = Router::new().route("/projects", axum::routing::get(routes::get_projects));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

src/routes.rsrust

use axum::Json;
use std::fs;
use crate::models::Project;

pub async fn get_projects() -> Json<Vec<Project>> {
    let data = fs::read_to_string("src/data/projects.json").expect("Unable to read file");
    let projects: Vec<Project> = serde_json::from_str(&data).expect("JSON was not well-formed");
    Json(projects)
}

src/models.rsrust

use serde::Serialize;

#[derive(Serialize)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub github: String,
    pub live: Option<String>,
    pub tech: Vec<String>,
    pub image: Option<String>,
}

src/data/projects.jsonjson

[
  {
    "name": "Free-Scribe",
    "description": "Free AI-powered tool to transcribe and translate audio recordings and audio files.",
    "github": "https://github.com/vrovsky/free-scribe",
    "live": "https://freescriber.netlify.app/",
    "tech": ["ReactJS", "TailwindCSS", "Xenova/transformers"],
    "image": "/free-scribe.png"
  },
  {
    "name": "Droplify",
    "description": "Dropbox clone monorep",
    "github": "https://github.com/vrovsky/droplify",
    "tech": ["NextJS", "Clerk", "Neon", "Drizzle ORM", "ImageKit"],
    "image": "/droplify.png"
  },
  {
    "name": "Calendoor",
    "description": "Calendly clone monorep",
    "github": "https://github.com/vrovsky/calendoor",
    "tech": ["NextJS", "Clerk", "Neon", "Drizzle ORM"],
    "image": "/calendoor.png"
  },
  {
    "name": "SwoleFit",
    "description": "Create your own unique training plan for your fitness goals",
    "github": "https://github.com/vrovsky/swoley-fit-reactjs",
    "live": "https://swoley-portfolio.netlify.app",
    "tech": ["ReactJS", "TailwindCSS", "Vite"],
    "image": "/swolefit.png"
  },
  {
    "name": "Pumpgram",
    "description": "Simple weight counter for the gym bros and siss",
    "github": "https://github.com/vrovsky/pumpgram",
    "live": "https://pumpgram.netlify.app/",
    "tech": ["ReactJS", "FantaCSS", "Vite"],
    "image": "/pumpgram.png"
  },
  {
    "name": "r_wp",
    "description": "Rust-based web parser project",
    "github": "https://github.com/vrovsky/r_wp",
    "tech": ["Rust", "Reqwest"]
  },
  {
    "name": "tp_codec",
    "description": "TypeScript codec implemented in Rust",
    "github": "https://github.com/vrovsky/tp_codec",
    "tech": ["Rust", "TypeScript"]
  }
]

.gitignore

target
