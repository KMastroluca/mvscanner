/**
 * @module PageRenderer
 * Renders A Page Using One Or More Templates
 */

/**
 * Takes The Parent Template Element And Injects It Into The DOM At The Root Node
 * @param {HTMLElement} parentPageElement - The Outer Template Element
 */
export const renderPage = (parentPageElement) => {
    let rootElement = document.getElementById("root");
    if (rootElement !== undefined) {

        // Check For More Than 1 Child Element
        if (rootElement.children.length > 1) {
            console.error("[-] A Component Cannot Have More Than 1 Child Element At The Top Level");
            document.body.innerText = "[-] A Component Cannot Have More Than 1 Child Element At The Top Level";
        } else {
            console.info("[+] renderPage:(): Found #root Element, Proceeding With Page Render.");
            rootElement.appendChild(parentPageElement);
        }

    } else {
        // Display Error Message
        console.error("[-] renderPage(): Unable To Find #root Element For Page Render");
        document.body.innerText = "Whoops! Couldn't Find The #root Element!";
    }
}