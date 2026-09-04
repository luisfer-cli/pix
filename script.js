const translations = {
  es: {
    navShowcase: "Diseños",
    navFeatures: "Funciones",
    navStart: "Empezar",
    status: "RENDERIZANDO SIN ANTIALIASING",
    heroLine1: "Pixel art",
    heroLine2: "como código.",
    heroLead: "Una pequeña DSL, un renderer determinista y todas las herramientas para convertir texto en sprites nítidos.",
    heroCta: "Crear un sprite",
    heroGallery: "Ver diseños",
    statStatic: "Estático",
    statAnimated: "Animado",
    aiEyebrow: "TRANSPARENCIA CREATIVA",
    aiTitle: "Estos diseños están hechos 100% con inteligencia artificial usando pix.",
    showEyebrow: "GALERÍA GENERADA",
    showTitle: "Ideas convertidas en píxeles",
    showLead: "Cada imagen se describió como texto y fue renderizada con el CLI de pix.",
    viewSource: "Ver fuente .pix →",
    featuresEyebrow: "EL TOOLKIT",
    featuresTitle: "Pequeño. Preciso. Potente.",
    featureDetTitle: "Determinista",
    featureDetText: "Misma entrada, mismos píxeles. Sin antialiasing ni resultados inesperados.",
    featurePalTitle: "Paletas estrictas",
    featurePalText: "Colores con nombre y validación para mantener cada asset coherente.",
    featureAnimTitle: "Animación nativa",
    featureAnimText: "Frames, secuencias y FPS exportados directamente a GIF.",
    featureCliTitle: "CLI completo",
    featureCliText: "Valida, formatea y exporta PNG, GIF, spritesheet y metadata JSON.",
    sourceEyebrow: "PIXEL ART LEGIBLE",
    sourceTitle: "El dibujo también es código.",
    sourceText: "Guarda tus sprites en Git, revisa cambios línea a línea y regenera cada asset cuando quieras.",
    fullExample: "Abrir ejemplo completo →",
    copy: "COPIAR",
    copied: "COPIADO ✓",
    terminalSuccess: "✓ LISTO — tus píxeles están vivos",
    startEyebrow: "PLAYER 1, ¿LISTO?",
    startTitle: "Crea tu primer sprite.",
    startText: "Clona el repositorio, escribe unas líneas y deja que pix haga el resto.",
    openRepo: "Abrir repositorio ↗",
    logoEyebrow: "HECHO CON SU PROPIA HERRAMIENTA",
    logoTitle: "Sí, el logo de pix también fue renderizado con pix.",
    logoSource: "Ver código del logo →"
  },
  en: {
    navShowcase: "Showcase",
    navFeatures: "Features",
    navStart: "Get started",
    status: "RENDERING WITHOUT ANTIALIASING",
    heroLine1: "Pixel art",
    heroLine2: "as code.",
    heroLead: "A tiny DSL, a deterministic renderer, and every tool you need to turn text into crisp sprites.",
    heroCta: "Create a sprite",
    heroGallery: "View showcase",
    statStatic: "Static",
    statAnimated: "Animated",
    aiEyebrow: "CREATIVE TRANSPARENCY",
    aiTitle: "These designs were made 100% with artificial intelligence using pix.",
    showEyebrow: "GENERATED GALLERY",
    showTitle: "Ideas turned into pixels",
    showLead: "Every image was described as text and rendered with the pix CLI.",
    viewSource: "View .pix source →",
    featuresEyebrow: "THE TOOLKIT",
    featuresTitle: "Tiny. Precise. Powerful.",
    featureDetTitle: "Deterministic",
    featureDetText: "Same input, same pixels. No antialiasing and no unexpected results.",
    featurePalTitle: "Strict palettes",
    featurePalText: "Named colors and validation keep every asset visually consistent.",
    featureAnimTitle: "Native animation",
    featureAnimText: "Frames, sequences, and FPS exported directly to GIF.",
    featureCliTitle: "Complete CLI",
    featureCliText: "Validate, format, and export PNG, GIF, spritesheets, and JSON metadata.",
    sourceEyebrow: "READABLE PIXEL ART",
    sourceTitle: "The artwork is code, too.",
    sourceText: "Keep sprites in Git, review changes line by line, and regenerate every asset whenever you want.",
    fullExample: "Open full example →",
    copy: "COPY",
    copied: "COPIED ✓",
    terminalSuccess: "✓ DONE — your pixels are alive",
    startEyebrow: "PLAYER 1, READY?",
    startTitle: "Create your first sprite.",
    startText: "Clone the repository, write a few lines, and let pix do the rest.",
    openRepo: "Open repository ↗",
    logoEyebrow: "MADE WITH ITS OWN TOOL",
    logoTitle: "Yes, the pix logo was also rendered with pix.",
    logoSource: "View logo source →"
  }
};

let currentLanguage = "es";

function setLanguage(language) {
  if (!translations[language]) return;
  currentLanguage = language;
  document.documentElement.lang = language;
  document.querySelectorAll("[data-i18n]").forEach((element) => {
    const value = translations[language][element.dataset.i18n];
    if (value) element.textContent = value;
  });
  document.querySelectorAll("[data-lang]").forEach((button) => {
    const active = button.dataset.lang === language;
    button.classList.toggle("active", active);
    button.setAttribute("aria-pressed", String(active));
  });
  document.title = language === "es" ? "pix · Pixel art como código" : "pix · Pixel art as code";
  try { localStorage.setItem("pix-language", language); } catch (_) { /* Storage may be disabled. */ }
}

document.querySelectorAll("[data-lang]").forEach((button) => {
  button.addEventListener("click", () => setLanguage(button.dataset.lang));
});

document.querySelectorAll("[data-copy]").forEach((button) => {
  button.addEventListener("click", async () => {
    const target = document.querySelector(`#${button.dataset.copy}`);
    if (!target) return;
    await navigator.clipboard.writeText(target.textContent.trim());
    button.textContent = translations[currentLanguage].copied;
    setTimeout(() => { button.textContent = translations[currentLanguage].copy; }, 1400);
  });
});

let savedLanguage = "es";
try { savedLanguage = localStorage.getItem("pix-language") || "es"; } catch (_) { /* Use Spanish by default. */ }
setLanguage(savedLanguage);
