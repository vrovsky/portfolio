<script lang="ts">
	import type { Project } from '$lib/types';

	let { project }: { project: Project } = $props();
</script>

<div class="opacity-80 hover:opacity-100" role="region">
	<h4 class="mb-2 text-2xl font-bold">{project.name}</h4>
	<div class="mb-4 flex flex-wrap gap-2">
		{#each project.tech as tech (tech)}
			<span class="bg-primary/40 hover:bg-primary/80 rounded-sm px-3 py-1 text-sm transition-colors"
				>{tech}</span
			>
		{/each}
	</div>
	<div class="min-h-20 flex-1">
		<p>{project.description}</p>
		<div class="mt-4 flex space-x-4">
			{#if project.github}
				<!-- eslint-disable-next-line svelte/no-navigation-without-resolve -->
				<a target="_blank" href={project.github} class="text-pink-600 hover:underline">GitHub</a>
				{#if project.live}
					<!-- eslint-disable-next-line svelte/no-navigation-without-resolve -->
					<a target="_blank" href={project.live} class="text-pink-600 hover:underline">Live</a>
				{/if}
			{/if}
		</div>
	</div>
</div>
