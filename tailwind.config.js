/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {
      fontFamily: {
        harry_p: "harry-p",
      },
      keyframes: {
        hp: {
          "0%, 100%": { transform: "translate(0px, 0px) rotate(-2deg)" },
          "50%": { transform: "translate(1px, 1px) rotate(2deg)" },
        },
      },
    },
  },
  plugins: [require("daisyui")],
  daisyui: {
    themes: ["retro", "coffee"],
    darkTheme: "coffee",
  },
};
