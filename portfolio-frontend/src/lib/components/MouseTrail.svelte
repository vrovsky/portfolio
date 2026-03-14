<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import gsap from 'gsap';

	let mouse = $state({ x: 0, y: 0 });
	let lastTrailTime = 0;
	const TRAIL_THROTTLE_MS = 50; // Throttle to create trail every 50ms

	onMount(() => {
		if (!browser) return;

		const handleMouseMove = (e: MouseEvent) => {
			mouse.x = e.clientX;
			mouse.y = e.clientY;
			const now = Date.now();
			if (now - lastTrailTime > TRAIL_THROTTLE_MS) {
				lastTrailTime = now;
				createTrail();
			}
		};

		window.addEventListener('mousemove', handleMouseMove);

		return () => {
			window.removeEventListener('mousemove', handleMouseMove);
		};
	});

	function createTrail() {
		const trail = document.createElement('div');
		trail.classList.add(
			'trail',
			'fixed',
			'w-12',
			'h-12',
			'rounded-xs',
			'pointer-events-none',
			'mix-blend-screen',
			'will-change-transform-opacity',
			'z-50'
		);

		const colors = [
			'radial-gradient(circle, #ff6b6b, transparent)',
			'radial-gradient(circle, #4ecdc4, transparent)',
			'radial-gradient(circle, #45b7d1, transparent)',
			'radial-gradient(circle, #f7dc6f, transparent)',
			'radial-gradient(circle, #bb8fce, transparent)'
		];
		trail.style.background = colors[Math.floor(Math.random() * colors.length)];
		trail.style.filter = 'drop-shadow(0 0 10px rgba(255, 255, 255, 0.8))';

		const rotation = Math.random() * 360;
		trail.style.transform = `translate(-50%, -50%) rotate(${rotation}deg)`;
		trail.style.left = '0';
		trail.style.top = '0';

		document.body.appendChild(trail);

		gsap.set(trail, {
			x: mouse.x,
			y: mouse.y,
			opacity: 1,
			scale: 1
		});

		gsap.to(trail, {
			opacity: 0,
			scale: 0.3,
			rotation: rotation + 180,
			x: mouse.x + (Math.random() * 40 - 20),
			y: mouse.y + (Math.random() * 40 - 20),
			duration: 0.8,
			ease: 'power2.out',
			onComplete: () => {
				trail.remove();
			}
		});
	}
</script>

<style>
	:global(.trail) {
		filter: drop-shadow(0 0 10px rgba(255, 255, 255, 0.8));
	}
</style>
