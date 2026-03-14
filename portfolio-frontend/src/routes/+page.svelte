<script lang="ts">
	import { fade } from 'svelte/transition';
	import { inview } from 'svelte-inview';
	import type { Project } from '$lib/types';
	import ProjectCard from '$lib/components/ProjectCard.svelte';

	let { data }: { data: { projects: Project[] } } = $props();

	let professionalProjects = $derived(data.projects.filter((p) => p.category === 'professional'));
	let petProjects = $derived(data.projects.filter((p) => p.category === 'pet'));

	let isInView = $state(false);
	let showGame = $state(false);
	let selectedColor = $state('#A00ABE');
	let ruined = $state(false);

	function playGame() {
		showGame = true;
	}

	function changeColor(color: string) {
		document.body.style.setProperty('background-color', color, 'important');
		ruined = true;
	}

	function resetColor() {
		document.body.style.removeProperty('background-color');
		showGame = false;
		ruined = false;
	}

	function handleInView(event: CustomEvent<{ inView: boolean }>) {
		isInView = event.detail.inView;
	}
</script>

<div
	use:inview
	oninview_change={handleInView}
	class="flex flex-row-reverse justify-between gap-12 px-80 opacity-0 transition-opacity duration-500"
	class:opacity-100={isInView}
>
	<section class="justify-top from-primary/20 to-accent/20 flex flex-col items-end pt-60">
		<h1 class="mb-4 text-6xl font-bold">Viachaslau Rouski</h1>
		<p class="mb-8 text-3xl">Full-Stack Developer | Rust & TypeScript Enthusiast</p>
		<p class="mb-8 max-w-2xl text-center text-lg">
			Building performant apps with a focus on AI, blockchain, and finantial tech.
		</p>
		<div class="mb-8 flex space-x-4 backdrop-blur-sm transition-all">
			<a
				target="_blank"
				href="https://github.com/vrovsky"
				class="bg-accent/80 hover:bg-accent rounded-md px-6 py-2 opacity-50 hover:opacity-100"
				>GitHub</a
			>
			<a
				target="_blank"
				href="https://www.linkedin.com/in/viachaslau-rouski-58840521a/"
				class="rounded-md bg-orange-700 px-6 py-2 opacity-50 hover:bg-orange-600 hover:opacity-100"
				>LinkedIn</a
			>
			<a
				href="mailto:vrovsky@gmail.com"
				class="bg-primary/80 hover:bg-primary rounded-md px-6 py-2 opacity-50 hover:opacity-100"
				>Email Me</a
			>
		</div>
		<button
			onclick={playGame}
			class="hover:bg-green600 rounded-md bg-green-500 px-6 py-4 text-white opacity-0 backdrop-blur-sm transition-opacity duration-200 hover:opacity-100"
			class:opacity-70={isInView}>Explore the Colors</button
		>
		{#if showGame}
			<div
				in:fade={{ duration: 500 }}
				class="flex flex-col items-start justify-start rounded-sm p-4 shadow-sm backdrop-blur-lg"
			>
				<div class="flex flex-col items-center justify-center gap-4 opacity-80 hover:opacity-100">
					<h4 class="text-xl">Pick a Color</h4>
					{#if ruined}
						<button
							onclick={resetColor}
							class="rounded-sm bg-red-500 px-2 py-2 text-white opacity-80 transition-all hover:bg-red-600 hover:opacity-100"
							>Reset</button
						>
					{:else}
						<input
							type="color"
							bind:value={selectedColor}
							onchange={(e) => changeColor(e.currentTarget.value)}
							class="mt-2 h-12 w-12 cursor-pointer rounded-md border-none opacity-80 hover:opacity-100"
						/>
					{/if}
				</div>
			</div>
		{/if}
	</section>

	<div use:inview class="from-primary/20 to-accent/20 flex flex-col pt-20">
		<section in:fade={{ duration: 800 }}>
			<h2 class="mb-8 text-start text-4xl font-bold">Skills</h2>
			<div class="flex flex-col gap-6">
				<div class="rounded-xs bg-white/10 p-6 shadow-md backdrop-blur-lg hover:bg-pink-500/20">
					<h3 class="mb-4 text-2xl">Tech Stack</h3>
					<p>
						Rust | TypeScript | Solidity | Next.js | Node.js | React | SvelteKit <br /> TailwindCSS
						| Three.js | GSAP | PostgreSQL <br /> AWS | Docker | Grafana | Cypress | Selenium | Gherkin
						| Axum
					</p>
				</div>
				<div
					class="col-span-2 rounded-xs bg-white/10 p-6 shadow-md backdrop-blur-lg hover:bg-pink-500/20"
				>
					<h3 class="mb-4 text-2xl">Leadership & Soft Skills</h3>
					<p>
						Team Management | Cross-functional Collaboration <br /> Leadership | Work Proccesses Adjustment
					</p>
				</div>
				<div class="rounded-xs bg-white/10 p-6 shadow-md backdrop-blur-lg hover:bg-pink-500/20">
					<h3 class="mb-4 text-2xl">Methodologies</h3>
					<p>Agile | Scrum | Kanban</p>
				</div>
				<div class="rounded-xs bg-white/10 p-6 shadow-md backdrop-blur-lg hover:bg-pink-500/20">
					<h3 class="mb-4 text-2xl">Tools</h3>
					<p>JIRA | Trello | Miro | Notion | ClickUp</p>
				</div>
				<div class="rounded-xs bg-white/10 p-6 shadow-md backdrop-blur-lg hover:bg-pink-500/20">
					<h3 class="mb-4 text-2xl">Languages</h3>
					<p>
						English — Fluent | Belarussian, Russian — Native <br /> Portugese, Spanish, French - Under-intermediate
					</p>
				</div>
			</div>
		</section>
	</div>
</div>
<section class="px-80 pt-60 pb-4">
	<h2 class="mb-12 text-end text-4xl font-bold">Last Professional Work</h2>
	<div class="grid grid-cols-3 gap-8">
		{#each professionalProjects as project (project.name)}
			<div class="bg-accent/10 hover:bg-accent/40 rounded-sm p-4 shadow-md" role="article">
				<ProjectCard {project} />
			</div>
		{/each}
	</div>
</section>
<section class="px-80 pt-20 pb-20">
	<h2 class="mb-12 text-end text-4xl font-bold">Last Pet Projects</h2>
	<div class="grid grid-cols-3 gap-8">
		{#each petProjects as project (project.name)}
			<div class="bg-accent/10 hover:bg-accent/40 rounded-sm p-4 shadow-md" role="article">
				<ProjectCard {project} />
			</div>
		{/each}
	</div>
</section>
