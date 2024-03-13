import type { Config } from "tailwindcss";

const config: Config = {
  content: [
    "./src/pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/components/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/app/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      backgroundImage: {
        "gradient-radial": "radial-gradient(var(--tw-gradient-stops))",
        "gradient-conic":
          "conic-gradient(from 180deg at 50% 50%, var(--tw-gradient-stops))",
      },
    },
  },
  plugins: [require('daisyui')],
  daisyui: {
    themes: [
      {
        "light": {
          "primary": "#d45628",
          "primary-content": "#ffffff",
          "secondary": "#6ea545",
          "accent": "#941914",
          "neutral": "#c9c9c9",
          "base-100": "#f5f5f4",
          "base-200": "#f0f0ee",
          "base-300": "#f2f2f1",
          "info": "#1e40af",
          "success": "#22c55e",
          "warning": "#facc15",
          "error": "#ef4444",
        },
        "dark": {
          "primary": "#d45628",
          "secondary": "#6ea545",
          "accent": "#941914",
          "neutral": "#575656",
          "base-100": "#202124",
          "info": "#1e40af",
          "success": "#22c55e",
          "warning": "#facc15",
          "error": "#ef4444",
        }
      }
    ]
  }
};
export default config;
