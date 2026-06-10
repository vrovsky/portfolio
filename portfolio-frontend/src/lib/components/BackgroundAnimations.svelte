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
	let cloud: THREE.Group;
	let lines: THREE.LineSegments;
	let lineColorAttr: THREE.BufferAttribute;
	let linePhases: Float32Array;
	let lineSpeeds: Float32Array;
	let lineCount = 0;
	let mouseX = 0;
	let mouseY = 0;

	const PARTICLE_COUNT = 2000;

	const MAX_CONNECTIONS = 300;
	const CONNECTION_DISTANCE = 8;
	const LINE_LIGHTNESS = 0.7;

	const PARTICLE_HUE_OFFSET = (199 - 221) / 360;
	const PARTICLE_SATURATION = 0.89;
	const PARTICLE_LIGHTNESS = 0.48;
	const HUE_SYNC_INTERVAL_MS = 250;
	let lastHueSync = 0;
	const bgColor = new THREE.Color();

	onMount(() => {
		if (!browser) return;
		// Respect users who prefer reduced motion — skip the continuous WebGL animation.
		if (window.matchMedia('(prefers-reduced-motion: reduce)').matches) return;

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
			size: 0.8,
			map: createParticleTexture(),
			transparent: true,
			opacity: 0.8,
			depthWrite: false,
			blending: THREE.AdditiveBlending
		});

		particles = new THREE.Points(geometry, material);

		cloud = new THREE.Group();
		cloud.add(particles);
		cloud.add(createConnectionLines());
		scene.add(cloud);
	}

	function createParticleTexture(): THREE.CanvasTexture {
		const canvas = document.createElement('canvas');
		canvas.width = 64;
		canvas.height = 64;
		const ctx = canvas.getContext('2d')!;
		const gradient = ctx.createRadialGradient(32, 32, 0, 32, 32, 32);
		gradient.addColorStop(0, 'rgba(255, 255, 255, 1)');
		gradient.addColorStop(0.4, 'rgba(255, 255, 255, 0.6)');
		gradient.addColorStop(1, 'rgba(255, 255, 255, 0)');
		ctx.fillStyle = gradient;
		ctx.fillRect(0, 0, 64, 64);
		return new THREE.CanvasTexture(canvas);
	}

	function createConnectionLines(): THREE.LineSegments {
		const maxDistSq = CONNECTION_DISTANCE * CONNECTION_DISTANCE;
		const pairs: number[] = [];
		for (let i = 0; i < PARTICLE_COUNT; i++) {
			const ix = particlePositions[i * 3];
			const iy = particlePositions[i * 3 + 1];
			const iz = particlePositions[i * 3 + 2];
			for (let j = i + 1; j < PARTICLE_COUNT; j++) {
				const dx = ix - particlePositions[j * 3];
				if (dx > CONNECTION_DISTANCE || dx < -CONNECTION_DISTANCE) continue;
				const dy = iy - particlePositions[j * 3 + 1];
				const dz = iz - particlePositions[j * 3 + 2];
				if (dx * dx + dy * dy + dz * dz < maxDistSq) pairs.push(i, j);
			}
		}

		const pairCount = pairs.length / 2;
		lineCount = Math.min(MAX_CONNECTIONS, pairCount);
		for (let i = 0; i < lineCount; i++) {
			const r = i + Math.floor(Math.random() * (pairCount - i));
			for (let k = 0; k < 2; k++) {
				const tmp = pairs[i * 2 + k];
				pairs[i * 2 + k] = pairs[r * 2 + k];
				pairs[r * 2 + k] = tmp;
			}
		}

		const positions = new Float32Array(lineCount * 6);
		const colors = new Float32Array(lineCount * 6);
		linePhases = new Float32Array(lineCount);
		lineSpeeds = new Float32Array(lineCount);
		for (let i = 0; i < lineCount; i++) {
			for (let k = 0; k < 2; k++) {
				const p = pairs[i * 2 + k] * 3;
				positions[i * 6 + k * 3] = particlePositions[p];
				positions[i * 6 + k * 3 + 1] = particlePositions[p + 1];
				positions[i * 6 + k * 3 + 2] = particlePositions[p + 2];
			}
			linePhases[i] = Math.random() * Math.PI * 2;
			lineSpeeds[i] = 0.2 + Math.random() * 0.5;
		}

		const geometry = new THREE.BufferGeometry();
		geometry.setAttribute('position', new THREE.BufferAttribute(positions, 3));
		lineColorAttr = new THREE.BufferAttribute(colors, 3);
		geometry.setAttribute('color', lineColorAttr);

		const material = new THREE.LineBasicMaterial({
			color: 0x0ea5e9,
			vertexColors: true,
			transparent: true,
			opacity: 0.5,
			depthWrite: false,
			blending: THREE.AdditiveBlending
		});

		lines = new THREE.LineSegments(geometry, material);
		return lines;
	}

	function onMouseMove(event: MouseEvent) {
		mouseX = (event.clientX / window.innerWidth) * 2 - 1;
		mouseY = -(event.clientY / window.innerHeight) * 2 + 1;
	}

	function animate(time = 0) {
		animationId = requestAnimationFrame(animate);

		if (cloud) {
			cloud.rotation.y += 0.0005;
			cloud.rotation.x += 0.0002;

			cloud.position.x += (mouseX * 0.5 - cloud.position.x) * 0.02;
			cloud.position.y += (mouseY * 0.5 - cloud.position.y) * 0.02;

			updateLineTwinkle(time);
			syncParticleColor(time);
		}

		renderer.render(scene, camera);
	}

	function updateLineTwinkle(time: number) {
		const t = time * 0.001;
		const colors = lineColorAttr.array as Float32Array;
		for (let i = 0; i < lineCount; i++) {
			const wave = Math.max(0, Math.sin(t * lineSpeeds[i] + linePhases[i]));
			const brightness = wave * wave * wave;
			for (let k = 0; k < 6; k++) colors[i * 6 + k] = brightness;
		}
		lineColorAttr.needsUpdate = true;
	}

	function syncParticleColor(time: number) {
		if (time - lastHueSync < HUE_SYNC_INTERVAL_MS) return;
		lastHueSync = time;

		const bg = getComputedStyle(document.body).backgroundColor;
		const match = bg.match(/rgb\((\d+),\s*(\d+),\s*(\d+)/);
		if (!match) return;

		bgColor.setRGB(+match[1] / 255, +match[2] / 255, +match[3] / 255, THREE.SRGBColorSpace);
		const { h } = bgColor.getHSL({ h: 0, s: 0, l: 0 }, THREE.SRGBColorSpace);
		const hue = (h + PARTICLE_HUE_OFFSET + 1) % 1;
		const material = particles.material as THREE.PointsMaterial;
		material.color.setHSL(hue, PARTICLE_SATURATION, PARTICLE_LIGHTNESS, THREE.SRGBColorSpace);
		const lineMaterial = lines.material as THREE.LineBasicMaterial;
		lineMaterial.color.setHSL(hue, PARTICLE_SATURATION, LINE_LIGHTNESS, THREE.SRGBColorSpace);
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
