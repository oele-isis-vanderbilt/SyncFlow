import type { Config } from 'tailwindcss';
import { plugin, content } from 'flowbite-react/tailwind';

const config: Config = {
  darkMode: ['variant', '&:is(.dark *)&:not(.light *)'],
  content: [
    './pages/**/*.{js,ts,jsx,tsx,mdx}',
    './components/**/*.{js,ts,jsx,tsx,mdx}',
    './app/**/*.{js,ts,jsx,tsx,mdx}',
    content(),
  ],
  theme: {
    extend: {
      backgroundImage: {
        'gradient-radial': 'radial-gradient(var(--tw-gradient-stops))',
        'gradient-conic':
          'conic-gradient(from 180deg at 50% 50%, var(--tw-gradient-stops))',
      },
    },
  },
  plugins: [plugin(), require('tailwind-scrollbar')],
};
export default config;
