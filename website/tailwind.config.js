module.exports = {
  content: [
    "./pages/**/*.{js,ts,jsx,tsx}",
    "./components/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      fontFamily: {
        Poppins: ["Poppins", "sans-serif"],
      },
      colors: {
        grey: "#E9E9E9",
        radial: { bl: "#201B06", tr: "#0B101A" },
      },
      fontSize: {
        huge: "92px",
      },
    },
  },
  plugins: [],
};
