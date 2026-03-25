import { withMermaid } from "vitepress-plugin-mermaid";

/** @type {import('vitepress').UserConfig} */
const config = {
  title: "Modbus MQTT Bridge",
  description: "Documentation for setup, configuration, operations, and integrations",
  base: "/modbus-mqtt-bridge/",
  cleanUrls: true,
  lastUpdated: true,
  themeConfig: {
    logo: "/logo.svg",
    nav: [
      { text: "Guide", link: "/" },
      { text: "Configuration", link: "/configuration" },
      { text: "Deployment", link: "/deployment" }
    ],
    search: {
      provider: "local"
    },
    sidebar: [
      { text: "Overview", link: "/" },
      { text: "Getting Started", link: "/getting-started" },
      { text: "Concepts", link: "/concepts" },
      { text: "Architecture", link: "/architecture" },
      { text: "Recipes", link: "/recipes" },
      { text: "Topic Contract", link: "/topic-contract" },
      { text: "Configuration", link: "/configuration" },
      { text: "Deployment", link: "/deployment" },
      { text: "Troubleshooting", link: "/troubleshooting" },
      { text: "Operations", link: "/operations" }
    ],
    editLink: {
      pattern: "https://github.com/tobiaswaelde/modbus-mqtt-bridge/edit/master/docs/:path",
      text: "Edit this page on GitHub"
    },
    lastUpdatedText: "Last updated",
    socialLinks: [
      { icon: "github", link: "https://github.com/tobiaswaelde/modbus-mqtt-bridge" }
    ],
    footer: {
      message: "Released under GPL-3.0-or-later",
      copyright: "Copyright 2026 Modbus MQTT Bridge"
    }
  }
};

export default withMermaid(config);
