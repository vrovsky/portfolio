<script lang="ts">
	import { fade } from 'svelte/transition';
	import { inview } from 'svelte-inview';
	import type { Project } from '$lib/types';
	import ProjectCard from '$lib/components/ProjectCard.svelte';
	import GlitchPhoto from '$lib/components/GlitchPhoto.svelte';

	let { data }: { data: { projects: Project[] } } = $props();

	let professionalProjects = $derived(data.projects.filter((p) => p.category === 'professional'));
	let petProjects = $derived(data.projects.filter((p) => p.category === 'pet'));

	let isInView = $state(true);

	function handleInView(event: CustomEvent<{ inView: boolean }>) {
		isInView = event.detail.inView;
	}
</script>

<div
	use:inview
	oninview_change={handleInView}
	class="mx-auto flex w-full max-w-[1600px] flex-col justify-between gap-6 px-4 transition-opacity duration-500 lg:flex-row lg:gap-12 lg:px-8"
	class:opacity-100={isInView}
>
	<section
		class="flex flex-col items-center justify-start from-primary/20 to-accent/20 pt-8 text-center lg:flex-1 lg:items-start lg:pt-20 lg:text-start"
	>
		<div class="flex flex-col items-center lg:w-full lg:items-start">
			<h1 class="mb-4 text-3xl font-bold md:text-2xl lg:text-5xl">Viachaslau Rouski</h1>
			<p class="text-xl md:text-xl lg:text-2xl">Senior Full-Stack Engineer & Team Lead</p>
			<p class="mb-8 text-xl md:text-xl lg:text-2xl">Rust & TypeScript</p>
			<p class="mb-8 max-w-2xl text-base text-pretty md:text-xl lg:text-2xl lg:text-balance">
				Turning complex blockchain, AI, and fintech problems into production-grade products.
			</p>
			<div class="mb-8 hidden space-x-4 backdrop-blur-sm transition-all lg:flex">
				<a
					target="_blank"
					href="https://github.com/vrovsky"
					class="btn-cut bg-violet-600 px-5 py-2 opacity-80 backdrop-blur-sm hover:bg-violet-500 hover:opacity-90"
					>GitHub</a
				>
				<a
					target="_blank"
					href="https://www.linkedin.com/in/viachaslau-rouski-58840521a/"
					class="btn-cut bg-cyan-700 px-5 py-2 opacity-80 backdrop-blur-sm hover:bg-cyan-600 hover:opacity-90"
					>LinkedIn</a
				>
				<a
					href="mailto:vrovsky@gmail.com"
					class="btn-cut bg-sky-700 px-5 py-2 opacity-80 backdrop-blur-sm hover:bg-sky-600 hover:opacity-90"
					>Email</a
				>
				<a
					target="_blank"
					href="https://t.me/rovskyv"
					class="btn-cut bg-blue-700 px-5 py-2 opacity-80 backdrop-blur-sm hover:bg-blue-600 hover:opacity-90"
					>Telegram</a
				>
			</div>
		</div>
		<div class="mt-6 flex w-full justify-center lg:relative lg:mt-10 lg:block lg:min-h-0 lg:flex-1">
			<GlitchPhoto src="/me-and-dog.webp" alt="Viachaslau Rouski with his dog" fill />
		</div>
	</section>

	<div use:inview class="flex flex-col from-primary/20 to-accent/20 pt-12 lg:flex-1 lg:pt-20">
		<section in:fade={{ duration: 800 }}>
			<h2 class="mb-8 text-center text-2xl font-bold sm:text-3xl lg:text-start lg:text-4xl">
				Skills
			</h2>
			<div class="flex flex-col gap-4 lg:gap-6">
				<div class="info-card rounded-sm p-6 shadow-md">
					<h3 class="mb-4 text-2xl">Tech Stack</h3>
					<p>
						Rust | TypeScript | Solidity | Next.js | Node.js | React | SvelteKit | TailwindCSS |
						Three.js | GSAP | PostgreSQL | AWS | Docker | Grafana | Cypress | Selenium | Gherkin |
						Axum
					</p>
				</div>
				<div class="info-card rounded-sm p-6 shadow-md">
					<h3 class="mb-4 text-2xl">AI & LLM</h3>
					<p>
						OpenAI API | Claude / Anthropic API | LLM Integration | RAG | Vector Databases | Hugging
						Face Transformers | AI Agents | Prompt Engineering | LangChain | AI-Powered Apps
					</p>
				</div>
				<div class="info-card rounded-sm p-6 shadow-md">
					<h3 class="mb-4 text-2xl">Leadership & Soft Skills</h3>
					<p>
						Team Leadership | Mentorship | Cross-functional Collaboration | Engineering Process
						Improvement
					</p>
				</div>
				<div class="info-card rounded-sm p-6 shadow-md">
					<h3 class="mb-4 text-2xl">Methodologies</h3>
					<p>Agile | Scrum | Kanban</p>
				</div>
				<div class="info-card rounded-sm p-6 shadow-md">
					<h3 class="mb-4 text-2xl">Tools</h3>
					<p>JIRA | Trello | Miro | Notion | ClickUp</p>
				</div>
				<div class="info-card rounded-sm p-6 shadow-md">
					<h3 class="mb-4 text-2xl">Languages</h3>
					<p>
						English — Fluent | Belarusian, Russian — Native | Spanish, French — Intermediate |
						Portuguese — Basic
					</p>
				</div>
			</div>
		</section>
	</div>
</div>
<section class="mx-auto max-w-[1600px] px-4 pt-16 pb-4 lg:px-8 lg:pt-24">
	<h2 class="mb-8 text-center text-2xl font-bold sm:text-3xl lg:mb-12 lg:text-start lg:text-4xl">
		Last Professional Work
	</h2>
	<div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3 lg:gap-8">
		{#each professionalProjects as project (project.name)}
			<ProjectCard {project} />
		{/each}
	</div>
</section>
<section class="mx-auto max-w-[1600px] px-4 pt-8 pb-12 lg:px-8 lg:pt-16 lg:pb-20">
	<h2 class="mb-8 text-center text-2xl font-bold sm:text-3xl lg:mb-12 lg:text-start lg:text-4xl">
		Last Pet Projects
	</h2>
	<div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3 lg:gap-8">
		{#each petProjects as project (project.name)}
			<ProjectCard {project} />
		{/each}
	</div>
</section>
