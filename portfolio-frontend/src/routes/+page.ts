import type { PageLoad } from './$types';
import type { Project } from '$lib/types';
import projectsData from '$lib/data/projects.json';
import { env } from '$env/dynamic/public';

export const load: PageLoad = async ({ fetch }) => {
	const apiUrl = env.PUBLIC_API_URL;

	// Only hit the backend when an API URL is explicitly configured (e.g. local dev).
	// In production with no backend deployed, serve the bundled data directly — no failed request.
	if (apiUrl) {
		try {
			const res = await fetch(`${apiUrl}/projects`);
			if (res.ok) {
				return { projects: (await res.json()) as Project[] };
			}
		} catch {
			console.warn('Backend unavailable, using fallback data');
		}
	}

	return { projects: projectsData as Project[] };
};
