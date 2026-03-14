<script lang="ts">
	import type { Project } from '$lib/types';

	let { project }: { project: Project } = $props();
</script>

<div class="opacity-80 hover:opacity-100" role="region">
	<h4 class="mb-2 text-xl font-bold lg:text-2xl">{project.name}</h4>
	<div class="mb-4 flex flex-wrap gap-2">
		{#each project.tech as tech (tech)}
			<span
				class="rounded-sm bg-primary/40 px-2 py-1 text-xs transition-colors hover:bg-primary/80 lg:px-3 lg:text-sm"
				>{tech}</span
			>
		{/each}
	</div>
	<div class="min-h-16 flex-1 lg:min-h-20">
		<p class="text-sm whitespace-pre-line lg:text-base">{project.description}</p>
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
