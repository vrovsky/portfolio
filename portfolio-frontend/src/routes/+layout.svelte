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
	<title>Viachaslau Rouski — Senior Full-Stack Engineer & Team Lead</title>
	<meta
		name="description"
		content="Viachaslau Rouski — Senior Full-Stack Engineer and Team Lead specializing in Rust, TypeScript, React and Web3. Building high-performance trading, DeFi and AI platforms."
	/>
	<meta property="og:type" content="website" />
	<meta property="og:title" content="Viachaslau Rouski — Senior Full-Stack Engineer & Team Lead" />
	<meta
		property="og:description"
		content="Senior Full-Stack Engineer and Team Lead specializing in Rust, TypeScript, React and Web3."
	/>
	<meta name="twitter:card" content="summary" />
	<meta name="twitter:title" content="Viachaslau Rouski — Senior Full-Stack Engineer & Team Lead" />
	<meta
		name="twitter:description"
		content="Senior Full-Stack Engineer and Team Lead specializing in Rust, TypeScript, React and Web3."
	/>
</svelte:head>

<header
	class="sticky top-0 z-10 flex items-center justify-between gap-3 bg-gray-800/50 p-4 text-white backdrop-blur-md"
>
	<nav class="flex items-center gap-2 sm:gap-3">
		<a
			target="_blank"
			rel="noopener"
			href="https://github.com/vrovsky"
			aria-label="GitHub"
			class="icon-btn"><i class="fab fa-github"></i></a
		>
		<a
			target="_blank"
			rel="noopener"
			href="https://www.linkedin.com/in/viachaslau-rouski-58840521a/"
			aria-label="LinkedIn"
			class="icon-btn"><i class="fab fa-linkedin-in"></i></a
		>
		<a
			target="_blank"
			rel="noopener"
			href="https://t.me/rovskyv"
			aria-label="Telegram"
			class="icon-btn"><i class="fab fa-telegram-plane"></i></a
		>
		<a href="mailto:vrovsky@gmail.com" aria-label="Email" class="icon-btn"
			><i class="fas fa-envelope"></i></a
		>
	</nav>
	<!-- eslint-disable-next-line svelte/no-navigation-without-resolve -->
	<a href="/viachaslau_rouski_fs_cv.pdf" download aria-label="Download CV (PDF)" class="cv-btn">
		<i class="fas fa-file-pdf"></i>
		<span>Download CV</span>
	</a>
</header>

<main>
	{@render children()}
</main>

<footer class="bg-gray-800/50 p-4 text-start text-white backdrop-blur-md">
	<p>
		<a href="https://www.linkedin.com/in/viachaslau-rouski-58840521a/" class="hover:text-primary"
			>Vi Rovsky</a
		>, 2026
	</p>
</footer>
