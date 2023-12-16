/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{html,tsx,ts,js,jsx}"],
  theme: {
    extend: {},
  },
  plugins: [require("daisyui"), require("tailwind-scrollbar")],
};
