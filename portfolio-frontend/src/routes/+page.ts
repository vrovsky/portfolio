import type { PageLoad } from './$types';
import type { Project } from '$lib/types';
import projectsData from '$lib/data/projects.json';

const API_URL = import.meta.env.PUBLIC_API_URL || 'http://localhost:3001';

export const load: PageLoad = async ({ fetch }) => {
	try {
		const res = await fetch(`${API_URL}/projects`);

		if (res.ok) {
			const projects: Project[] = await res.json();
			return { projects };
		}
	} catch {
		console.warn('Backend unavailable, using fallback data');
	}

	return { projects: projectsData as Project[] };
};
