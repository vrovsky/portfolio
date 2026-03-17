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
		<h1 class="mb-4 text-3xl font-bold md:text-2xl lg:text-5xl">Viachaslau Rouski</h1>
		<p class="text-xl md:text-xl lg:text-2xl">Full-Stack Developer</p>
		<p class="mb-8 text-xl md:text-xl lg:text-2xl">Rust & TypeScript Enthusiast</p>
		<p class="md:text-md mb-8 max-w-2xl text-base lg:text-lg">
			Building performant apps with a focus on AI, blockchain, and finantial tech.
		</p>
		<div class="mb-8 hidden space-x-4 backdrop-blur-sm transition-all lg:flex">
			<a
				target="_blank"
				href="https://github.com/vrovsky"
				class="rounded-sm bg-violet-600 px-5 py-2 opacity-80 backdrop-blur-sm hover:bg-violet-500 hover:opacity-90"
				>GitHub</a
			>
			<a
				target="_blank"
				href="https://www.linkedin.com/in/viachaslau-rouski-58840521a/"
				class="rounded-sm bg-cyan-700 px-5 py-2 opacity-80 backdrop-blur-sm hover:bg-cyan-600 hover:opacity-90"
				>LinkedIn</a
			>
			<a
				href="mailto:vrovsky@gmail.com"
				class="rounded-sm bg-sky-700 px-5 py-2 opacity-80 backdrop-blur-sm hover:bg-sky-600 hover:opacity-90"
				>Email</a
			>
			<a
				target="_blank"
				href="https://t.me/rovskyv"
				class="rounded-sm bg-blue-700 px-5 py-2 opacity-80 backdrop-blur-sm hover:bg-blue-600 hover:opacity-90"
				>Telegram</a
			>
		</div>
		<button
			onclick={playGame}
			class="hover:bg-green600 rounded-sm bg-green-600 px-6 py-2 text-white opacity-0 backdrop-blur-sm transition-opacity duration-200 hover:bg-green-500 hover:opacity-100"
			class:opacity-80={isInView}>Explore the Colors</button
		>
		{#if showGame}
			<div in:fade={{ duration: 500 }} class="flex flex-col rounded-sm p-4">
				<div class="flex flex-col items-center justify-center gap-4 opacity-80 hover:opacity-100">
					<h4 class="text-xl">Pick a Color</h4>
					{#if ruined}
						<div class="flex gap-4">
							<input
								type="color"
								bind:value={selectedColor}
								onchange={(e) => changeColor(e.currentTarget.value)}
								class="h-12 w-12 cursor-pointer border border-white rounded-sm hover:scale-105"
							/>
							<button
								onclick={resetColor}
								class="rounded-sm bg-red-500 border border-black p-2 text-white transition-all hover:scale-105"
								>Reset</button
							>
						</div>
					{:else}
						<input
							type="color"
							bind:value={selectedColor}
							onchange={(e) => changeColor(e.currentTarget.value)}
							class="mt-2 h-12 w-12 cursor-pointer rounded-sm border-none opacity-80 hover:opacity-100"
						/>
					{/if}
				</div>
			</div>
		{/if}
	</section>

	<div use:inview class="flex flex-col from-primary/20 to-accent/20 pt-12 lg:pt-20">
		<section in:fade={{ duration: 800 }}>
			<h2 class="mb-8 text-center text-2xl font-bold md:text-xl lg:text-start lg:text-4xl">
				Skills
			</h2>
			<div class="flex flex-col gap-4 lg:gap-6">
				<div
					class="rounded-sm bg-white/10 bg-linear-to-br from-slate-900/40 via-blue-950/40 to-cyan-950/30 p-6 shadow-md hover:bg-violet-900/40"
				>
					<h3 class="mb-4 text-2xl">Tech Stack</h3>
					<p>
						Rust | TypeScript | Solidity | Next.js | Node.js | React | SvelteKit <br /> TailwindCSS
						| Three.js | GSAP | PostgreSQL <br /> AWS | Docker | Grafana | Cypress | Selenium | Gherkin
						| Axum
					</p>
				</div>
				<div
					class="col-span-2 rounded-sm bg-white/10 bg-linear-to-br from-slate-900/40 via-blue-950/40 to-cyan-950/30 p-6 shadow-md hover:bg-violet-900/40"
				>
					<h3 class="mb-4 text-2xl">Leadership & Soft Skills</h3>
					<p>
						Team Management | Cross-functional Collaboration <br /> Leadership | Work Proccesses Adjustment
					</p>
				</div>
				<div
					class="rounded-sm bg-white/10 bg-linear-to-br from-slate-900/40 via-blue-950/40 to-cyan-950/30 p-6 shadow-md hover:bg-violet-900/40"
				>
					<h3 class="mb-4 text-2xl">Methodologies</h3>
					<p>Agile | Scrum | Kanban</p>
				</div>
				<div
					class="rounded-sm bg-white/10 bg-linear-to-br from-slate-900/40 via-blue-950/40 to-cyan-950/30 p-6 shadow-md hover:bg-violet-900/40"
				>
					<h3 class="mb-4 text-2xl">Tools</h3>
					<p>JIRA | Trello | Miro | Notion | ClickUp</p>
				</div>
				<div
					class="rounded-sm bg-white/10 bg-linear-to-br from-slate-900/40 via-blue-950/40 to-cyan-950/30 p-6 shadow-md hover:bg-violet-900/40"
				>
					<h3 class="mb-4 text-2xl">Languages</h3>
					<p>
						English, Belarussian, Russian — Fluent <br /> Portugese, Spanish, French - Pre-intermediate
					</p>
				</div>
			</div>
		</section>
	</div>
</div>
<section class="px-4 pt-12 pb-4 lg:px-80 lg:pt-60">
	<h2 class="mb-8 text-center text-2xl font-bold md:text-2xl lg:mb-12 lg:text-end lg:text-4xl">
		Last Professional Work
	</h2>
	<div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3 lg:gap-8">
		{#each professionalProjects as project (project.name)}
			<div
				class="rounded-sm bg-white/10 bg-linear-to-br from-slate-900/40 via-blue-950/40 to-cyan-950/30 p-6 shadow-md hover:bg-violet-900/40"
				role="article"
			>
				<ProjectCard {project} />
			</div>
		{/each}
	</div>
</section>
<section class="px-4 pt-8 pb-12 lg:px-80 lg:pt-20 lg:pb-20">
	<h2 class="mb-8 text-center text-2xl font-bold md:text-3xl lg:mb-12 lg:text-end lg:text-4xl">
		Last Pet Projects
	</h2>
	<div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3 lg:gap-8">
		{#each petProjects as project (project.name)}
			<div
				class="rounded-sm bg-white/10 bg-linear-to-br from-slate-900/40 via-blue-950/40 to-cyan-950/30 p-6 shadow-md hover:bg-violet-900/40"
				role="article"
			>
				<ProjectCard {project} />
			</div>
		{/each}
	</div>
</section>
