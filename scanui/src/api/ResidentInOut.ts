/*
 *
 * Copyright (c) {11/8/23, 3:09 PM} Lorenzo A. Banks and Preston Thorpe. All rights reserved.
 * {ResidentInOut.ts}
 * {ResidentInOut.ts}
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


import {STableData} from '../components/STable';
import { SResident, STimestampResident } from "../types/Models"

export const getResidentsIn = async (): Promise<STableData> => {
   let addr = import.meta.env.VITE_BACKEND_ADDR;
   let port = import.meta.env.VITE_BACKEND_PORT;

   let response = await fetch('http://' + addr + ':' + port + '/api/residents');
   let data = await response.json();
   return {
      data:data,
   } as STableData;
};



/**
 * We also gotta calculate the priority residents, which are residents whom, as far as we can tell,
 * have left and not yet returned.
 */
export function getPriorityOutResidents(rdata:STimestampResident[]):STimestampResident[] {
   console.log("Getting Priority Out Residents.");
   
   let residentLocations:Map<string, STimestampResident> = new Map();

   for (let timestamp of rdata) {
      /**
       * Okay, we iterate thru and find all the entries that signal a resident has left
       */

      // If the resident has left the unit
      if (timestamp.room !== timestamp.destinationLabel ) {
         
         // Check if we have a location for this resident.
         let currentLocation = residentLocations.get(timestamp.rfid);


         // If we do, and thier current timestamp is later than the one we stored, or it doesnt exist
         if (!currentLocation || timestamp.timestampLeft > currentLocation.timestampLeft) {
            // update the set
            residentLocations.set(timestamp.rfid, timestamp);
         } 

      }
   }



   let residentsWithoutReturn: STimestampResident[] = Array.from(residentLocations.values());
   return residentsWithoutReturn;



};



export const getResidentsOut = async (): Promise<STableData> => {
   console.log("Executed GetResidentsOut");
   let addr = import.meta.env.VITE_BACKEND_ADDR;
   let port = import.meta.env.VITE_BACKEND_PORT;

   let responseResidents = await fetch('http://' + addr + ':' + port + '/api/residents');
   let residentsData = await responseResidents.json();
   console.log("Residents:", residentsData);
   

   let responseTimestamps = await fetch('http://' + addr + ':' + port + '/api/timestamps');
   let timestampsData = await responseTimestamps.json();
   console.log("Timestamps:", timestampsData);

   let residentsOut:STimestampResident[] = [];
   for (let timestamp of timestampsData) {
      console.log("Timestamp: ", timestamp);
      if (timestamp.date === null) {
         continue;
      }
      let resident = residentsData.find((resident:SResident) => resident.rfid === timestamp.rfid);
      console.log("Resident: ", resident);
      if (resident === undefined) {
         continue;
      }

      let responseLocation = await fetch("http://" + addr + ":" + port + "/api/locations/" + timestamp.dest);
      let locationData = await responseLocation.json();
      console.log("Location Data:", locationData);
      if (locationData === undefined && locationData.name === undefined) {
         continue;
      }




      let timestampResident:STimestampResident = {
         rfid: resident.rfid,
         name: resident.name,
         doc: resident.doc,
         room: resident.room,
         timestampLeft: timestamp.time,
         destinationLabel: locationData.name
      }
      residentsOut.push(timestampResident);
   }

   
   let priorityOutResidents = getPriorityOutResidents(residentsOut);

   let returnObj = {data:residentsOut, priorityData:priorityOutResidents};
   return returnObj;

}
