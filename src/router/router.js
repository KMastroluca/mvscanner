/**
 * @module Router
 */




class Route {

    /** @private */
    uri;
    /** @private */
    callback;
    /** @private */
    params;

    /**
     * @callback routeCallback
     * @param {Object} params - Object Representing Route Parameters.
     * @returns {string} - String Representing HTML To Be Rendered To The Page
     */


    /**
     * Route Constructor
     * @param {string} uri - The Uri Of The Intended Route eg: /products/{productid}
     * @param {routeCallback} callback - Callback Function, Called When Page Is Loaded With Given Uri
     * @param {Object} params - Object containing params for the route callback function to process
     */
    constructor(uri, callback, params) {
        this.uri = uri;
        this.callback = callback;
        this.params = params;
    }

}