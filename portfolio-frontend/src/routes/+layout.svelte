<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import MouseTrail from '$lib/components/MouseTrail.svelte';
	import BackgroundAnimations from '$lib/components/BackgroundAnimations.svelte';

	let { children } = $props();
</script>

<BackgroundAnimations />
<MouseTrail />

<svelte:head>
	<link rel="icon" href={favicon} />
	<link
		rel="stylesheet"
		href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.1/css/all.min.css"
	/>
	<title>Rouski Portfolio</title>
</svelte:head>

<header
	class="sticky top-0 z-10 flex items-center justify-between bg-gray-800/50 p-4 text-white backdrop-blur-md"
>
	<nav class="flex items-center space-x-4">
		<a
			target="_blank"
			href="https://github.com/vrovsky"
			aria-label="GitHub"
			class="transition-colors hover:text-primary"><i class="fab fa-github"></i></a
		>
		<a
			target="_blank"
			href="https://www.linkedin.com/in/viachaslau-rouski-58840521a/"
			aria-label="LinkedIn"
			class="transition-colors hover:text-primary"><i class="fab fa-linkedin"></i></a
		>
		<a
			target="_blank"
			href="https://t.me/rovskyv"
			aria-label="Telegram"
			class="transition-colors hover:text-primary"><i class="fab fa-telegram"></i></a
		>
		<a
			target="_blank"
			href="mailto:vrovsky@gmail.com"
			aria-label="Email"
			class="rotate-90 transition-colors hover:text-primary"><i class="fab fa-envelope"></i></a
		>
	</nav>
</header>

<main>
	{@render children()}
</main>

<footer class="bg-gray-800/50 p-4 text-start text-white backdrop-blur-md">
	<p>
		<a href="https://www.linkedin.com/in/viachaslau-rouski-58840521a/" class="hover:text-primary"
			>Vi Rouski</a
		>, 2026
	</p>
</footer>
