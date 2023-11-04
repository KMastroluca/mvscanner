/**
 * @module Template
 * This Is A Class That Represents HTML as an object, so it can be passed around as an object rather than a
 * string before it gets to the page.
 * It takes html in, to create the object, and eventually, renders html out.
 */



class Template {
    /** @private {HTMLElement} */
    templateElement;

    /**
     * Template Constructor
     * @param {string} rawHtml - Raw HTML To Construct The Template With
     */
    constructor(rawHtml) {
        const parser = new DOMParser();
        const doc = parser.parseFromString(rawHtml, "text/html");
        this.templateElement = doc.body.firstElementChild;
    }

    /**
     * Template Element Getter
     * @return {HTMLElement}
     */
    getTemplateElement() {
        return this.templateElement;
    }

}


export default Template;



