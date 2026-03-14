<script lang="ts">
	import type { Project } from '$lib/types';

	let { project }: { project: Project } = $props();
</script>

<div class="flex h-full flex-col opacity-80 hover:opacity-100" role="region">
	<h4 class="mb-4 text-xl font-bold md:text-xl lg:text-2xl">{project.name}</h4>
	<div class="mb-4 flex flex-wrap gap-2">
		{#each project.tech as tech (tech)}
			<span
				class="transition-color rounded-sm border border-cyan-700/40 bg-cyan-900/60 px-2 py-1 text-xs text-cyan-200 hover:bg-cyan-800/70 md:px-2 md:text-xs lg:px-3 lg:text-sm"
				>{tech}</span
			>
		{/each}
	</div>
	<div class="flex min-h-16 flex-1 flex-col justify-between lg:min-h-20">
		<p class="text-sm whitespace-pre-line text-slate-300 md:text-sm lg:text-base mb-4">
			{project.description}
		</p>
		<div class="mt-auto flex w-full space-x-4">
			{#if project.github}
				<!-- eslint-disable-next-line svelte/no-navigation-without-resolve -->
				<a target="_blank" href={project.github} class="text-pink-400 hover:text-pink-300">GitHub</a
				>
			{/if}
			{#if project.live}
				<!-- eslint-disable-next-line svelte/no-navigation-without-resolve -->
				<a target="_blank" href={project.live} class="text-pink-400 hover:text-pink-300">Live</a>
			{/if}
		</div>
	</div>
</div>
