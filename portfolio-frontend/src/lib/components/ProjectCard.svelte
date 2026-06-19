<script lang="ts">
	import type { Project } from '$lib/types';

	let { project }: { project: Project } = $props();
</script>

<!-- 4 direct children (title / chips / description / links) map to 4 subgrid rows
     so every band aligns across the cards in a row with no reserved dead space.
     Falls back to a simple flex column below the md grid. -->
<div
	class="info-card flex flex-col gap-4 rounded-sm p-6 opacity-80 shadow-md hover:opacity-100 md:row-span-4 md:grid md:grid-rows-subgrid"
>
	<h4 class="text-xl font-bold md:text-xl lg:text-2xl">{project.name}</h4>
	<div class="flex flex-wrap content-start gap-2">
		{#each project.tech as tech (tech)}
			<span class="chip rounded-sm border px-2 py-1 text-xs md:px-2 md:text-xs lg:px-3 lg:text-sm"
				>{tech}</span
			>
		{/each}
	</div>
	<p class="whitespace-pre-line">
		{project.description}
	</p>
	<div class="flex w-full items-end space-x-4">
		{#if project.github}
			<!-- eslint-disable-next-line svelte/no-navigation-without-resolve -->
			<a target="_blank" href={project.github} class="text-pink-400 hover:text-pink-300">GitHub</a>
		{/if}
		{#if project.live}
			<!-- eslint-disable-next-line svelte/no-navigation-without-resolve -->
			<a target="_blank" href={project.live} class="text-pink-400 hover:text-pink-300">Live</a>
		{/if}
	</div>
</div>
