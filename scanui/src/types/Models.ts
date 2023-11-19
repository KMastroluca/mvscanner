/*
 *
 * Copyright (c) {11/8/23, 2:10 PM} Lorenzo A. Banks and Preston Thorpe. All rights reserved.
 * {Models.ts}
 * {Models.ts}
 *
 * This software is protected by copyright laws and international copyright treaties, as
 * well as other intellectual property laws and treaties. The software is licensed, not sold.
 *
 * However, you are not permitted to use the software as is, or distribute it without
 * obtaining a license from the authors. Unauthorized use of the software may result in
 * severe civil and criminal penalties, and will be prosecuted to the maximum extent
 * possible under law.
 *
 * THIS SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
 *  BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 *  ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 *  CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 *  SOFTWARE.
 *
 * In no case shall the authors or copyright holders be liable for any claim, damages or
 * other liability arising from, out of or in connection with the softwareor the use or
 * other dealings in the software.
 * ********************************************************************************
 */

/**  
 * This is the model for data returned about
 * a resident and their location. As seen by the server.
 * */
export interface STimestampResident {
    [key:string]:string|number;
    rfid:string;
    name:string;
    doc:string;
    room:string;
    timestampLeft:string;
    destinationLabel:string; // The idea is this comes from the Timestamp destinationId, and is resolved to a location name
}


export interface SResident {
    [key:string]:string |number;
    rfid:string;
    name:string;
    doc:string;
    room:string;
}



export interface SLocation {
    [key:string]:string|number;
    id:number;
    name:string;
}



export interface STimestamp {
    [key:string]:string|number;
    rfid:string;
    destinationId:number;
    timestamp:string;
}

