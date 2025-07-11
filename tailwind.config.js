/** @type {import("tailwindcss").Config} */
module.exports = {
  mode: "jit",
  purge: false,
  safelist: [
    { pattern: /.*/ }, // <- all classes
  ],
   content: [
    "./src/**/*.rs",        // Rust files
    "./index.html",         // Arquivo HTML principal
    "./src/**/*.html",      // Todos os arquivos HTML dentro de src
    "./src/**/*.css",       // CSS também
    "./src/**/*.js",        // Se houver JS, também deve estar incluso
    "./src/**/*.tsx",       // Arquivos TypeScript/React se aplicável
  ],
  theme: {
    extend: {
      fontFamily: {
        "bebas-neue": ["Bebas Neue", "sans-serif"],
        "fira-code": ["Fira Code", "monospace"],
        "lato": ["Lato", "sans-serif"],
        "league-spartan": ["League Spartan", "sans-serif"],
        "poppins": ["Poppins", "sans-serif"],
      },
      keyframes: {
        rotate: {
          "0%": { transform: "rotate(0deg)" },
          "100%": { transform: "rotate(360deg)" },
        },
        show: {
          "0%": { opacity: "0" },
          "100%": { opacity: "1" },
        },
        elastic: {
          "0%": {
            transform: "scale(1)",
          },
          "30%": {
            transform: "scale(1.3)",
          },
          "50%": {
            transform: "scale(0.9)",
          },
          "70%": {
            transform: "scale(1.2)",
          },
          "100%": {
            transform: "scale(1)",
          },
        },
        "snow-fall": {
          "0%": { "transform": "translateY(-100vh) rotate(0deg)", "opacity": "1" },
          "20%": { "opacity": "0.9" },
          "40%": { "opacity": "0.8" },
          "60%": { "opacity": "0.7" },
          "80%": { "opacity": "0.5" },
          "100%": { "transform": "translateY(100vh) rotate(360deg)", "opacity": "0" }
        },
        "snow-fall-slow": {
          "0%": { "transform": "translateY(-100vh) rotate(0deg)", "opacity": "1" },
          "25%": { "opacity": "0.9" },
          "50%": { "opacity": "0.7" },
          "75%": { "opacity": "0.5" },
          "100%": { "transform": "translateY(100vh) rotate(360deg)", "opacity": "0" }
        },
        "snow-fall-fast": {
          "0%": { "transform": "translateY(-100vh) rotate(0deg)", "opacity": "1" },
          "15%": { "opacity": "0.9" },
          "30%": { "opacity": "0.8" },
          "50%": { "opacity": "0.6" },
          "70%": { "opacity": "0.4" },
          "100%": { "transform": "translateY(100vh) rotate(360deg)", "opacity": "0" }
        },
        "rocket-launch": {
          "0%": { "transform": "translateY(100vh)", "opacity": "0" },
          "20%": { "opacity": "0.5" },
          "40%": { "opacity": "0.7" },
          "60%": { "opacity": "0.9" },
          "80%": { "opacity": "1" },
          "100%": { "transform": "translateY(-100vh)", "opacity": "0" }
        },
        "rocket-launch-slow": {
          "0%": { "transform": "translateY(100vh)", "opacity": "0" },
          "25%": { "opacity": "0.5" },
          "50%": { "opacity": "0.7" },
          "75%": { "opacity": "0.9" },
          "100%": { "transform": "translateY(-100vh)", "opacity": "0" }
        },
        "rocket-launch-fast": {
          "0%": { "transform": "translateY(100vh)", "opacity": "0" },
          "15%": { "opacity": "0.5" },
          "30%": { "opacity": "0.7" },
          "50%": { "opacity": "0.9" },
          "70%": { "opacity": "1" },
          "100%": { "transform": "translateY(-100vh)", "opacity": "0" }
        }
      },
      animation: {
        show: "show 0.5s ease-in-out",
        "rotate-loop": "rotate 2s linear infinite",
        "elastic-loop": "elastic 1s ease-in-out infinite",
        "snow-fall-loop": "snow-fall 15s linear infinite",
        "snow-fall-slow-loop": "snow-fall-slow 20s linear infinite",
        "snow-fall-fast-loop": "snow-fall-fast 10s linear infinite",
        "rocket-launch-loop": "rocket-launch 10s ease-in infinite",
        "rocket-launch-slow-loop": "rocket-launch-slow 20s ease-in infinite",
        "rocket-launch-fast-loop": "rocket-launch-fast 5s ease-in infinite"
      }
    }
  },
  variants: {
    extend: {
      fontWeight: ["responsive", "hover", "focus"],
    },
  },
  plugin: [
    
  ]
};