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
	class="flex flex-col justify-between gap-6 px-4 opacity-0 transition-opacity duration-500 lg:flex-row-reverse lg:gap-12 lg:px-80"
	class:opacity-100={isInView}
>
	<section
		class="flex flex-col items-center justify-start from-primary/20 to-accent/20 pt-8 text-center lg:items-end lg:pt-60 lg:text-end"
	>
		<h1 class="mb-4 text-3xl font-bold lg:text-6xl">Viachaslau Rouski</h1>
		<p class="text-xl lg:text-3xl">Full-Stack Developer</p>
		<p class="mb-8 text-xl lg:text-3xl">Rust & TypeScript Enthusiast</p>
		<p class="mb-8 max-w-2xl text-base lg:text-lg">
			Building performant apps with a focus on AI, blockchain, and finantial tech.
		</p>
		<div class="mb-8 flex space-x-4 backdrop-blur-sm transition-all">
			<a
				target="_blank"
				href="https://github.com/vrovsky"
				class="rounded-md bg-accent/80 px-6 py-2 opacity-50 hover:bg-accent hover:opacity-100"
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
				class="rounded-md bg-primary/80 px-6 py-2 opacity-50 hover:bg-primary hover:opacity-100"
				>Email Me</a
			>
			<a
				target="_blank"
				href="https://t.me/rovskyv"
				class="rounded-md bg-blue-500/80 px-6 py-2 opacity-50 hover:bg-blue-500 hover:opacity-100"
				>Telegram</a
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

	<div use:inview class="flex flex-col from-primary/20 to-accent/20 pt-12 lg:pt-20">
		<section in:fade={{ duration: 800 }}>
			<h2 class="mb-8 text-center text-2xl font-bold lg:text-start lg:text-4xl">Skills</h2>
			<div class="flex flex-col gap-4 lg:gap-6">
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
<section class="px-4 pt-12 pb-4 lg:px-80 lg:pt-60">
	<h2 class="mb-8 text-center text-2xl font-bold lg:mb-12 lg:text-end lg:text-4xl">
		Last Professional Work
	</h2>
	<div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3 lg:gap-8">
		{#each professionalProjects as project (project.name)}
			<div class="rounded-sm bg-accent/10 p-4 shadow-md hover:bg-accent/40" role="article">
				<ProjectCard {project} />
			</div>
		{/each}
	</div>
</section>
<section class="px-4 pt-8 pb-12 lg:px-80 lg:pt-20 lg:pb-20">
	<h2 class="mb-8 text-center text-2xl font-bold lg:mb-12 lg:text-end lg:text-4xl">
		Last Pet Projects
	</h2>
	<div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3 lg:gap-8">
		{#each petProjects as project (project.name)}
			<div class="rounded-sm bg-accent/10 p-4 shadow-md hover:bg-accent/40" role="article">
				<ProjectCard {project} />
			</div>
		{/each}
	</div>
</section>
