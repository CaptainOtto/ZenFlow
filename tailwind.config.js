module.exports = {
  content: ["./index.html", "./src/**/*.{html,svelte,js,ts}"],
  darkMode: "class",
  theme: {
    colors: {
      black: {
        DEFAULT: "#000000",
        100: "#212121",
      },
      white: {
        DEFAULT: "#FFFFFF",
      },
      orange: {
        orange: "#ff9800",
        "orange-50": "#fff3e0",
        100: "#ffe0b2",
        200: "#ffcc80",
        300: "#ffb74d",
        400: "#ffa726",
        500: "#ff9800",
        600: "#fb8c00",
        700: "#f57c00",
        800: "#ef6c00",
        900: "#e65100",
      },
    },
    fontSize: {
      sm: "0.8rem",
      base: "1rem",
      xl: "1.25rem",
      "2xl": "1.563rem",
      "3xl": "1.953rem",
      "4xl": "2.441rem",
      "5xl": "3.052rem",
    },
  },
  plugins: [],
  variants: {
    extend: {
      backgroundColor: ["hover"],
    },
  },
};
