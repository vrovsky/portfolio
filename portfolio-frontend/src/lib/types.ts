export interface Project {
	name: string;
	description: string;
	github: string;
	live?: string;
	tech: string[];
	image?: string;
	category: 'pet' | 'professional';
}
