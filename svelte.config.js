import adapter from '@sveltejs/adapter-static'; 

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {  // some settings for adapter-static:
		adapter: adapter({
			pages: 'build',
			assets: 'build',
			fallback: 'index.html'
		}),
		alias: {
			$components: 'src/lib/components',
			$books: 'src/mock/books'
		}
	}
};

export default config;