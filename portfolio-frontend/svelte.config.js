import adapter from '@sveltejs/adapter-netlify';

const config = {
	kit: {
		adapter: adapter({
			split: true
		})
	},
	vitePlugin: {
		dynamicCompileOptions: ({ filename }) =>
			filename.includes('node_modules') ? undefined : { runes: true }
	}
};

export default config;
