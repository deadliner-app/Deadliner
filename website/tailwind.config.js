module.exports = {
  content: [
    "./pages/**/*.{js,ts,jsx,tsx}",
    "./components/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      width: {
        app_screenshot: 465,
        icon: 45,
        85: "350px",
      },
      height: {
        app_screenshot: 766,
        icon: 45,
        85: "350px",
      },
      screens: {
        xl: "1313px",
      },
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
