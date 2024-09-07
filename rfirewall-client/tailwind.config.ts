import type { Config } from "tailwindcss";

const config: Config = {
  content: [
    "./src/pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/components/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/app/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      colors: {
        softCoral: "#FF6F61", // For "Add Rule" button, hamburger menu
        softCoralHover: "#E55C50", // For "Add Rule" button hover
        mutedTeal: "#4CAF92", // For "Start" button
        mutedTealHover: "#3A8D77", // For "Start" button hover
        vibrantSlate: "#537EC5", // For the rFirewall logo (option 1)
        royalBlue: "#345995", // For the rFirewall logo (option 2)
        paleMustard: "#FFCC66", // For highlights, notifications
        lightBeige: "#F7F3E9", // Background for the main content area
        mutedSlateGray: "#2C2E33", // For the header background
        softNavy: "#354259", // Alternative header background color
        charcoalGray: "#4A4A4A", // For primary text
      },
    },
  },
  plugins: [],
};
export default config;
