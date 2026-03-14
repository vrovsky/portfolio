<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { browser } from '$app/environment';
	import * as THREE from 'three';

	let container: HTMLDivElement;
	let animationId: number;
	let scene: THREE.Scene;
	let camera: THREE.PerspectiveCamera;
	let renderer: THREE.WebGLRenderer;
	let particles: THREE.Points;
	let particlePositions: Float32Array;
	let mouseX = 0;
	let mouseY = 0;

	const PARTICLE_COUNT = 2000;

	onMount(() => {
		if (!browser) return;

		initThree();
		animate();
		window.addEventListener('resize', onResize);
		window.addEventListener('mousemove', onMouseMove);
	});

	onDestroy(() => {
		if (!browser) return;

		if (animationId) cancelAnimationFrame(animationId);
		window.removeEventListener('resize', onResize);
		window.removeEventListener('mousemove', onMouseMove);
		if (renderer) renderer.dispose();
	});

	function initThree() {
		scene = new THREE.Scene();

		const width = window.innerWidth;
		const height = window.innerHeight;

		camera = new THREE.PerspectiveCamera(75, width / height, 0.1, 1000);
		camera.position.z = 50;

		renderer = new THREE.WebGLRenderer({ alpha: true, antialias: true });
		renderer.setSize(width, height);
		renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
		// eslint-disable-next-line svelte/no-dom-manipulating
		container.appendChild(renderer.domElement);

		const geometry = new THREE.BufferGeometry();
		particlePositions = new Float32Array(PARTICLE_COUNT * 3);

		for (let i = 0; i < PARTICLE_COUNT * 3; i += 3) {
			particlePositions[i] = (Math.random() - 0.5) * 100;
			particlePositions[i + 1] = (Math.random() - 0.5) * 100;
			particlePositions[i + 2] = (Math.random() - 0.5) * 50;
		}

		geometry.setAttribute('position', new THREE.BufferAttribute(particlePositions, 3));

		const material = new THREE.PointsMaterial({
			color: 0x0ea5e9,
			size: 0.5,
			transparent: true,
			opacity: 0.8,
			blending: THREE.AdditiveBlending
		});

		particles = new THREE.Points(geometry, material);
		scene.add(particles);
	}

	function onMouseMove(event: MouseEvent) {
		mouseX = (event.clientX / window.innerWidth) * 2 - 1;
		mouseY = -(event.clientY / window.innerHeight) * 2 + 1;
	}

	function animate() {
		animationId = requestAnimationFrame(animate);

		if (particles) {
			particles.rotation.y += 0.0005;
			particles.rotation.x += 0.0002;

			particles.position.x += (mouseX * 0.5 - particles.position.x) * 0.02;
			particles.position.y += (mouseY * 0.5 - particles.position.y) * 0.02;
		}

		renderer.render(scene, camera);
	}

	function onResize() {
		const width = window.innerWidth;
		const height = window.innerHeight;

		camera.aspect = width / height;
		camera.updateProjectionMatrix();
		renderer.setSize(width, height);
	}
</script>

<div bind:this={container} class="background-canvas"></div>

<style>
	.background-canvas {
		position: fixed;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		z-index: -1;
		pointer-events: none;
		overflow: hidden;
	}

	.background-canvas :global(canvas) {
		display: block;
	}
</style>
