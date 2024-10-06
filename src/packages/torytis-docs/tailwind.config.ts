import type { Config } from "tailwindcss";
import { PluginCreator } from "tailwindcss/types/config";

const staticVariantPlugin: PluginCreator = ({ addVariant }) => {
  addVariant("my-active", [".my-active &", ".my-active&"]);
  addVariant("hocus", ["&:hover", "&:focus"]);
  addVariant("sidebar-toggle-yes", [".sidebar-toggle-yes &", ".sidebar-toggle-yes&"]);
  addVariant("sidebar-toggle-no", [".sidebar-toggle-no &", ".sidebar-toggle-no&"]);
};

const config: Config = {
  content: ["./src/**/*.{js,ts,jsx,tsx,mdx}"],
  theme: {
    extend: {
      width: {
        "sidebar-width": `var(--sidebar-width)`,
        "topbar-height": `var(--topbar-height)`,
        "full-subtraction-sidebar-width": `var(--full-subtraction-sidebar-width)`,
      },
      height: {
        "sidebar-width": `var(--sidebar-width)`,
        "topbar-height": `var(--topbar-height)`,
        "full-subtraction-sidebar-width": `var(--full-subtraction-sidebar-width)`,
      },
      margin: {
        "sidebar-width": `var(--sidebar-width)`,
        "topbar-height": `var(--topbar-height)`,
        "full-subtraction-sidebar-width": `var(--full-subtraction-sidebar-width)`,
      },
      padding: {
        "sidebar-width": `var(--sidebar-width)`,
        "topbar-height": `var(--topbar-height)`,
        "full-subtraction-sidebar-width": `var(--full-subtraction-sidebar-width)`,
      },
      inset: {
        "sidebar-width": `var(--sidebar-width)`,
        "topbar-height": `var(--topbar-height)`,
        "full-subtraction-sidebar-width": `var(--full-subtraction-sidebar-width)`,
      },
      colors: {
        background: "var(--background)",
        foreground: "var(--foreground)",
      },
      keyframes: {
        "fade-in-opacity": {
          "0%": {
            width: "100%",
            height: "100%",
            opacity: "0",
          },
          "1%": {
            width: "100%",
            height: "100%",
            opacity: "0",
          },
          "100%": {
            width: "100%",
            height: "100%",
            opacity: "1",
          },
        },
        "fade-out-opacity": {
          "0%": {
            width: "100%",
            height: "100%",
            opacity: "1",
          },
          "99%": {
            width: "100%",
            height: "100%",
            opacity: "0",
          },
          "100%": {
            width: "0",
            height: "0",
            opacity: "0",
          },
        },
      },
      animation: {
        "fade-in-opacity": "fade-in-opacity 0.3s cubic-bezier(0.175, 0.885, 0.320, 1.275) both",
        "fade-out-opacity": "fade-out-opacity 0.3s cubic-bezier(0.175, 0.885, 0.320, 1.275) both",
      },
    },
  },
  plugins: [staticVariantPlugin],
};
export default config;
