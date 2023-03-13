/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{vue,js,jsx,ts,tsx,html}", "index.html"],
  theme: {
    screens: {
      sm: "640px",
      md: "1280",
      lg: "1400",
      xl: "1536px",
      xxl: "1920px",
    },
    extend: {
      colors: {
        primary: "var(--primary-color)",
        info: "var(--info-color)",
        success: "var(--success-color)",
        warning: "var(--warning-color)",
        error: "var(--error-color)",
      },
      animation: {
        spin: "spin 2s linear infinite",
        rotateY: "rotateY .5s linear",
      },
      keyframes: {
        spin: {
          from: {
            transform: "rotate(0deg)",
          },
          to: {
            transform: "rotate(360deg)",
          },
        },
      },
    },
  },
  plugins: [],
};
