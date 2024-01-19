import type { Config } from 'tailwindcss';

export default {
    content: [
        './.torytis/index.css',
        './src/**/*.{ts,tsx,css,scss}',
    ],
    theme: {
      extend: {},
    },
    plugins: [],
  } satisfies Config