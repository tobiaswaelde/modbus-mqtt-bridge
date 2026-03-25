/** @type {import('vitepress').UserConfig} */
export default {
  title: "Modbus MQTT Bridge",
  description: "Documentation for setup, configuration, operations, and integrations",
  base: "/modbus-mqtt-bridge/",
  cleanUrls: true,
  themeConfig: {
    logo: "/logo.svg",
    nav: [
      { text: "Guide", link: "/" },
      { text: "Configuration", link: "/configuration" },
      { text: "GitHub", link: "https://github.com/tobiaswaelde/modbus-mqtt-bridge" }
    ],
    sidebar: [
      { text: "Overview", link: "/" },
      { text: "Getting Started", link: "/getting-started" },
      { text: "Concepts", link: "/concepts" },
      { text: "Architecture", link: "/architecture" },
      { text: "Recipes", link: "/recipes" },
      { text: "Topic Contract", link: "/topic-contract" },
      { text: "Configuration", link: "/configuration" },
      { text: "Docker", link: "/docker" },
      { text: "Troubleshooting", link: "/troubleshooting" },
      { text: "Operations", link: "/operations" }
    ],
    socialLinks: [
      { icon: "github", link: "https://github.com/tobiaswaelde/modbus-mqtt-bridge" }
    ],
    footer: {
      message: "Released under GPL-3.0-or-later",
      copyright: "Copyright 2026 Modbus MQTT Bridge"
    }
  }
};
