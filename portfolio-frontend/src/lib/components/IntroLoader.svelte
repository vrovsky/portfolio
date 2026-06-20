<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { fade } from 'svelte/transition';

	type Phase = 'loading' | 'modal' | 'done';
	let phase = $state<Phase>('loading');

	const NAME = 'VIACHASLAU ROUSKI';
	const GLYPHS = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789#%&*<>/\\=+';
	const STATUSES = ['INITIALIZING', 'DECRYPTING PROFILE', 'LOADING MODULES', 'ESTABLISHING UPLINK'];
	const DURATION = 1600;

	let display = $state(NAME);
	let progress = $state(0);
	let status = $state(STATUSES[0]);
	let closing = $state(false);

	let intervals: ReturnType<typeof setInterval>[] = [];
	let timers: ReturnType<typeof setTimeout>[] = [];

	function cleanup() {
		intervals.forEach(clearInterval);
		timers.forEach(clearTimeout);
		intervals = [];
		timers = [];
	}

	function finish() {
		cleanup();
		if (browser) sessionStorage.setItem('introSeen', '1');
		closing = true;
		setTimeout(() => {
			phase = 'done';
		}, 380);
	}

	function onKey(e: KeyboardEvent) {
		if (e.key === 'Escape' && phase !== 'done') finish();
	}

	function autofocus(node: HTMLButtonElement) {
		node.focus();
	}

	$effect(() => {
		if (!browser) return;
		document.body.style.overflow = phase === 'done' ? '' : 'hidden';
		return () => {
			document.body.style.overflow = '';
		};
	});

	onMount(() => {
		if (!browser) return;
		const reduced = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
		if (reduced || sessionStorage.getItem('introSeen')) {
			phase = 'done';
			return;
		}

		let frame = 0;
		intervals.push(
			setInterval(() => {
				frame++;
				const revealCount = Math.floor(frame / 2);
				let out = '';
				for (let i = 0; i < NAME.length; i++) {
					if (NAME[i] === ' ') out += ' ';
					else if (i < revealCount) out += NAME[i];
					else out += GLYPHS[Math.floor(Math.random() * GLYPHS.length)];
				}
				display = out;
				if (revealCount >= NAME.length) display = NAME;
			}, 50)
		);

		let s = 0;
		intervals.push(
			setInterval(() => {
				s = (s + 1) % STATUSES.length;
				status = STATUSES[s];
			}, 400)
		);

		const start = performance.now();
		intervals.push(
			setInterval(() => {
				progress = Math.min(100, Math.round(((performance.now() - start) / DURATION) * 100));
			}, 30)
		);

		timers.push(
			setTimeout(() => {
				cleanup();
				progress = 100;
				phase = 'modal';
			}, DURATION + 200)
		);

		return cleanup;
	});
</script>

<svelte:window onkeydown={onKey} />

{#if phase !== 'done'}
	<div class="intro-overlay" class:closing>
		{#if phase === 'loading'}
			<div class="intro-loader">
				<div class="intro-name">{display}</div>
				<div class="intro-bar"><span style="width:{progress}%"></span></div>
				<div class="intro-status">&gt; {status}_ {progress}%</div>
			</div>
			<button class="intro-skip" onclick={finish}>skip &gt;&gt;</button>
		{:else}
			<button class="intro-backdrop" aria-label="Dismiss" onclick={finish}></button>
			<div
				class="intro-modal"
				role="dialog"
				aria-modal="true"
				aria-labelledby="intro-title"
				transition:fade={{ duration: 250 }}
			>
				<button class="intro-close" aria-label="Close" onclick={finish}>×</button>
				<div class="intro-tag">// incoming transmission</div>
				<h2 id="intro-title" class="glitch" data-text="Hey there — what's up?">
					Hey there — what's up?
				</h2>
				<p>Welcome to my portfolio. Grab a coffee and have a look around.</p>
				<button class="intro-cta" onclick={finish} use:autofocus>Sure, let's dive in</button>
			</div>
		{/if}
	</div>
{/if}

<style>
	.intro-overlay {
		position: fixed;
		inset: 0;
		z-index: 200;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		background-color: hsl(221 42% 6% / 0.97);
		background-image: repeating-linear-gradient(
			to bottom,
			transparent 0,
			transparent 2px,
			hsl(199 89% 60% / 0.04) 3px,
			transparent 4px
		);
		backdrop-filter: blur(2px);
		font-family: 'Courier New', ui-monospace, monospace;
		transition: opacity 0.35s ease;
	}

	.intro-overlay.closing {
		opacity: 0;
		pointer-events: none;
	}

	/* ---- Loader ---- */
	.intro-loader {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1.25rem;
		padding: 0 1.5rem;
	}

	.intro-name {
		font-size: clamp(1.25rem, 6vw, 2.5rem);
		font-weight: 700;
		letter-spacing: 0.18em;
		color: #67e8f9;
		text-shadow:
			0 0 8px hsl(199 89% 55% / 0.7),
			0 0 22px hsl(199 89% 50% / 0.4);
		white-space: nowrap;
	}

	.intro-bar {
		width: min(320px, 70vw);
		height: 4px;
		border: 1px solid hsl(199 89% 60% / 0.4);
		background-color: hsl(221 40% 12% / 0.8);
		overflow: hidden;
	}

	.intro-bar span {
		display: block;
		height: 100%;
		background: linear-gradient(90deg, #0ea5e9, #a855f7);
		box-shadow: 0 0 12px hsl(199 89% 55% / 0.8);
		transition: width 0.1s linear;
	}

	.intro-status {
		font-size: 0.8rem;
		letter-spacing: 0.15em;
		color: #5eead4;
		text-shadow: 0 0 8px hsl(168 84% 50% / 0.5);
	}

	.intro-skip {
		position: absolute;
		right: 1.25rem;
		bottom: 1.25rem;
		font-family: inherit;
		font-size: 0.78rem;
		letter-spacing: 0.1em;
		color: hsl(0 0% 100% / 0.55);
		background: none;
		border: none;
		cursor: pointer;
		transition: color 0.2s ease;
	}
	.intro-skip:hover {
		color: #67e8f9;
	}

	/* ---- Modal ---- */
	.intro-backdrop {
		position: absolute;
		inset: 0;
		border: none;
		background: transparent;
		cursor: pointer;
	}

	.intro-modal {
		position: relative;
		max-width: min(440px, 90vw);
		margin: 0 1.5rem;
		padding: 2rem 1.75rem 1.75rem;
		text-align: center;
		color: #e2e8f0;
		background: linear-gradient(135deg, hsl(221 45% 12% / 0.96), hsl(266 45% 13% / 0.96));
		border: 1px solid hsl(199 89% 60% / 0.45);
		box-shadow:
			0 0 0 1px hsl(266 70% 60% / 0.15),
			0 0 40px hsl(199 89% 50% / 0.25),
			inset 0 0 30px hsl(199 89% 50% / 0.06);
		clip-path: polygon(
			0 0,
			calc(100% - 18px) 0,
			100% 18px,
			100% 100%,
			18px 100%,
			0 calc(100% - 18px)
		);
	}

	.intro-tag {
		font-family: 'Courier New', ui-monospace, monospace;
		font-size: 0.72rem;
		letter-spacing: 0.18em;
		text-transform: uppercase;
		color: #5eead4;
		margin-bottom: 0.75rem;
		opacity: 0.8;
	}

	.intro-modal h2 {
		font-family: 'Exo 2', sans-serif;
		font-size: clamp(1.4rem, 5vw, 2rem);
		font-weight: 700;
		color: #fff;
		margin-bottom: 0.6rem;
	}

	.intro-modal p {
		font-family: 'Exo 2', sans-serif;
		font-size: 0.95rem;
		color: hsl(0 0% 100% / 0.7);
		margin-bottom: 1.5rem;
	}

	.intro-cta {
		font-family: 'Exo 2', sans-serif;
		font-size: 0.95rem;
		font-weight: 600;
		color: #67e8f9;
		padding: 0.7rem 1.4rem;
		background-color: hsl(199 89% 48% / 0.15);
		border: 1px solid hsl(199 89% 60% / 0.55);
		border-radius: 9999px;
		cursor: pointer;
		transition:
			transform 0.18s ease,
			background-color 0.2s ease,
			box-shadow 0.2s ease;
	}
	.intro-cta:hover {
		transform: translateY(-2px);
		background-color: hsl(199 89% 48% / 0.28);
		box-shadow: 0 0 16px hsl(199 89% 52% / 0.5);
	}

	.intro-close {
		position: absolute;
		top: 0.5rem;
		right: 0.75rem;
		font-family: 'Exo 2', sans-serif;
		font-size: 1.5rem;
		line-height: 1;
		color: hsl(0 0% 100% / 0.5);
		background: none;
		border: none;
		cursor: pointer;
		transition: color 0.2s ease;
	}
	.intro-close:hover {
		color: #fff;
	}

	/* ---- Glitch heading ---- */
	.glitch {
		position: relative;
	}
	.glitch::before,
	.glitch::after {
		content: attr(data-text);
		position: absolute;
		inset: 0;
		pointer-events: none;
	}
	.glitch::before {
		color: #0ea5e9;
		transform: translateX(-2px);
		animation: glitch-x 2.2s infinite steps(2, end);
	}
	.glitch::after {
		color: #f0f;
		transform: translateX(2px);
		animation: glitch-x 1.7s infinite reverse steps(2, end);
	}

	@keyframes glitch-x {
		0%,
		92%,
		100% {
			opacity: 0;
			clip-path: inset(0 0 100% 0);
		}
		93% {
			opacity: 0.55;
			clip-path: inset(10% 0 60% 0);
		}
		96% {
			opacity: 0.55;
			clip-path: inset(55% 0 20% 0);
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.glitch::before,
		.glitch::after {
			animation: none;
			opacity: 0;
		}
		.intro-cta:hover {
			transform: none;
		}
	}
</style>
