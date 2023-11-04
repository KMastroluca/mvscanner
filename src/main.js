/**
 * @module
 * Main Entry Point For The Application
 */

import {renderPage} from "./types/pagerenderer.js";
import Template from "./types/template.js";

// Alpine JS
import Alpine from "alpinejs";

import App from "./app.js";



document.addEventListener("DOMContentLoaded", () => {
    // Initialize Alpine
    Alpine.start();

    // Load Page
    const t = new Template(App());
    renderPage(t.getTemplateElement());
});


document.addEventListener("alpine:init", () => {
    console.info("[+] Alpine Initialized!");
})

