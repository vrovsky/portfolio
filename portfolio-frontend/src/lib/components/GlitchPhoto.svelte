<script lang="ts">
	let {
		src = '/me-and-dog.jpg',
		alt = 'Viachaslau Rouski with his dog',
		fill = false
	}: { src?: string; alt?: string; fill?: boolean } = $props();
</script>

<figure class="glitch" class:glitch--fill={fill} style={`--src: url('${src}')`}>
	<img class="glitch__base" {src} {alt} loading="lazy" />
	<div class="glitch__channel glitch__channel--cyan" aria-hidden="true"></div>
	<div class="glitch__channel glitch__channel--magenta" aria-hidden="true"></div>
	<div class="glitch__grade" aria-hidden="true"></div>
	<div class="glitch__scanlines" aria-hidden="true"></div>
	<div class="glitch__frame" aria-hidden="true"></div>
</figure>

<style>
	.glitch {
		position: relative;
		isolation: isolate;
		width: clamp(15rem, 72vw, 26rem);
		aspect-ratio: 1009 / 1792;
		margin: 0;
		overflow: hidden;
		background: #05060a;
		/* Angular cut corners, matching the panels/buttons. */
		clip-path: var(--panel-cut);
		border: 1px solid hsl(calc((var(--bg-hue, 221) - 22) * 1deg) 90% 62% / 0.7);
		/* Neon glow via drop-shadow so it follows the angular silhouette
		   (a box-shadow would be clipped by clip-path). */
		filter: drop-shadow(0 0 18px hsl(calc((var(--bg-hue, 221) - 22) * 1deg) 90% 55% / 0.32))
			drop-shadow(0 0 46px hsl(calc((var(--bg-hue, 221) + 60) * 1deg) 90% 55% / 0.18));
	}

	/* Fill mode: on large screens, stop dictating height via the portrait ratio.
	   The figure is pulled out of flow and pinned to fill its (flex-stretched)
	   parent, so its bottom aligns with the skills column; the photo crops. */
	@media (min-width: 1024px) {
		.glitch--fill {
			position: absolute;
			inset-block: 0;
			left: 0;
			height: auto;
			aspect-ratio: auto;
		}
	}

	.glitch__base,
	.glitch__channel {
		position: absolute;
		inset: 0;
		width: 100%;
		height: 100%;
	}

	/* Full-colour base with a punchy cyberpunk grade. */
	.glitch__base {
		object-fit: cover;
		object-position: center 28%;
		filter: contrast(1.18) saturate(1.4) brightness(1.04);
		animation: glitch-jitter 7s steps(1) infinite;
	}

	/* Two tinted copies offset left/right for chromatic aberration.
	   background-blend-mode multiply tints the photo a single hue;
	   mix-blend-mode screen adds those neon fringes over the base. */
	.glitch__channel {
		background-image: var(--src);
		background-size: cover;
		background-position: center 28%;
		mix-blend-mode: screen;
		pointer-events: none;
	}

	.glitch__channel--cyan {
		background-color: #00fff0;
		background-blend-mode: multiply;
		transform: translateX(-2px);
		animation: glitch-cyan 7s steps(1) infinite;
	}

	.glitch__channel--magenta {
		background-color: #ff00d4;
		background-blend-mode: multiply;
		transform: translateX(2px);
		animation: glitch-magenta 7s steps(1) infinite;
	}

	/* Duotone wash + vignette for the cyberpunk colour cast. */
	.glitch__grade {
		position: absolute;
		inset: 0;
		background:
			radial-gradient(120% 90% at 50% 30%, transparent 55%, rgb(0 0 0 / 0.55) 100%),
			linear-gradient(135deg, hsl(190 100% 50% / 0.22), transparent 45%, hsl(315 100% 55% / 0.26));
		mix-blend-mode: soft-light;
		pointer-events: none;
	}

	/* CRT scanlines + a slow travelling scan highlight. */
	.glitch__scanlines {
		position: absolute;
		inset: 0;
		background:
			repeating-linear-gradient(
				to bottom,
				rgb(0 0 0 / 0) 0,
				rgb(0 0 0 / 0) 2px,
				rgb(0 0 0 / 0.28) 3px,
				rgb(0 0 0 / 0.28) 4px
			),
			linear-gradient(to bottom, transparent 0%, hsl(190 100% 70% / 0.08) 50%, transparent 100%);
		background-size:
			100% 4px,
			100% 100%;
		pointer-events: none;
		animation: glitch-scan 6s linear infinite;
	}

	/* Inner neon edge + corner brackets. */
	.glitch__frame {
		position: absolute;
		inset: 0;
		box-shadow: inset 0 0 0 1px hsl(calc((var(--bg-hue, 221) - 22) * 1deg) 100% 70% / 0.5);
		pointer-events: none;
		animation: glitch-flicker 5s steps(1) infinite;
	}
	.glitch__frame::before,
	.glitch__frame::after {
		content: '';
		position: absolute;
		width: 18px;
		height: 18px;
		border: 2px solid hsl(315 100% 65% / 0.8);
	}
	.glitch__frame::before {
		top: 6px;
		left: 6px;
		border-right: 0;
		border-bottom: 0;
	}
	.glitch__frame::after {
		right: 6px;
		bottom: 6px;
		border-left: 0;
		border-top: 0;
	}

	/* Base barely jitters, then snaps for a couple of frames now and then. */
	@keyframes glitch-jitter {
		0%,
		100% {
			transform: translate(0, 0);
		}
		29.6% {
			transform: translate(0, 0);
		}
		30% {
			transform: translate(-2px, 1px);
		}
		30.4% {
			transform: translate(0, 0);
		}
		70% {
			transform: translate(0, 0);
		}
		70.3% {
			transform: translate(2px, -1px);
		}
		70.6% {
			transform: translate(0, 0);
		}
	}

	/* Cyan channel: idle at -2px, jumps + slices during glitch windows. */
	@keyframes glitch-cyan {
		0%,
		100% {
			transform: translateX(-2px);
			clip-path: inset(0 0 0 0);
		}
		9.8% {
			transform: translateX(-2px);
			clip-path: inset(0 0 0 0);
		}
		10% {
			transform: translateX(-9px);
			clip-path: inset(18% 0 62% 0);
		}
		10.6% {
			transform: translateX(5px);
			clip-path: inset(70% 0 8% 0);
		}
		11% {
			transform: translateX(-2px);
			clip-path: inset(0 0 0 0);
		}
		47.8% {
			transform: translateX(-2px);
			clip-path: inset(0 0 0 0);
		}
		48.2% {
			transform: translateX(-11px);
			clip-path: inset(40% 0 38% 0);
		}
		48.7% {
			transform: translateX(-2px);
			clip-path: inset(0 0 0 0);
		}
	}

	/* Magenta channel: mirror of cyan, opposite offsets. */
	@keyframes glitch-magenta {
		0%,
		100% {
			transform: translateX(2px);
			clip-path: inset(0 0 0 0);
		}
		9.8% {
			transform: translateX(2px);
			clip-path: inset(0 0 0 0);
		}
		10% {
			transform: translateX(9px);
			clip-path: inset(58% 0 26% 0);
		}
		10.6% {
			transform: translateX(-5px);
			clip-path: inset(10% 0 74% 0);
		}
		11% {
			transform: translateX(2px);
			clip-path: inset(0 0 0 0);
		}
		47.8% {
			transform: translateX(2px);
			clip-path: inset(0 0 0 0);
		}
		48.2% {
			transform: translateX(11px);
			clip-path: inset(36% 0 42% 0);
		}
		48.7% {
			transform: translateX(2px);
			clip-path: inset(0 0 0 0);
		}
	}

	@keyframes glitch-scan {
		0% {
			background-position:
				0 0,
				0 -100%;
		}
		100% {
			background-position:
				0 0,
				0 100%;
		}
	}

	@keyframes glitch-flicker {
		0%,
		100% {
			opacity: 1;
		}
		48% {
			opacity: 1;
		}
		48.5% {
			opacity: 0.4;
		}
		49% {
			opacity: 1;
		}
		82% {
			opacity: 1;
		}
		82.4% {
			opacity: 0.55;
		}
		82.8% {
			opacity: 1;
		}
	}

	/* Hold still for users who prefer reduced motion — keep the look, drop the motion. */
	@media (prefers-reduced-motion: reduce) {
		.glitch__base,
		.glitch__channel--cyan,
		.glitch__channel--magenta,
		.glitch__scanlines,
		.glitch__frame {
			animation: none;
		}
	}
</style>
