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

import _ from 'lodash';

export const getResidentsIn = async (): Promise<STableData> => {
   let addr = import.meta.env.VITE_BACKEND_ADDR;
   let port = import.meta.env.VITE_BACKEND_PORT;

   let response = await fetch('http://' + addr + ':' + port + '/api/residents');
   let data = await response.json();
   return {
      data:data,
   } as STableData;
};


export function getRoundTrips(rdata:STimestampResident[]):STimestampResident[][] {
   const groupedStamps = _.groupBy(rdata, 'rfid');

   let roundTrips:STimestampResident[][] = [];
   Object.keys(groupedStamps).forEach((key) => {
      let group = groupedStamps[key];
      let roundTrip = [];
      
      for (let i = 0; i < group.length; i++) {
         roundTrip.push(group[i]);
         if (i > 0 && group[i].unit === group[i].destinationId) {
            roundTrips.push(roundTrip);
            roundTrip = [];
         }
      }
   });

   return roundTrips;
}


/**
 * 
 * Calculate resident timestamps that have returned.
 * @returns 
 */
export function getReturnedResidents(rdata:STimestampResident[]):{[key:string]:STimestampResident[]} {
   const lastReturnedResidents:{[key:string]:STimestampResident[]} = {};

   rdata.forEach((stamp) => {
      if (!lastReturnedResidents[stamp.rfid]) {
         lastReturnedResidents[stamp.rfid] = [];
      }
      if (stamp.unit === stamp.destinationId) {
         lastReturnedResidents[stamp.rfid].push(stamp);
      }
   });

   return lastReturnedResidents;
}

/**
 * We also gotta calculate the priority residents, which are residents whom, as far as we can tell,
 * have left and not yet returned.
 */
export function getPriorityOutResidents(rdata:STimestampResident[]):STimestampResident[] {
   
   console.log("Getting Priority Out Residents: Start RD: ", rdata);


   const unreturnedResidents:{[key:string]: STimestampResident} = {};

   rdata.forEach((stamp) => {
      if (stamp.unit !== stamp.destinationId) {
         unreturnedResidents[stamp.rfid] = stamp;
      } else {
         delete unreturnedResidents[stamp.rfid];
      }
   });

   return Object.values(unreturnedResidents);

};



export const getResidentsOut = async (): Promise<STableData> => {
   console.log("Executed GetResidentsOut");
   let addr = import.meta.env.VITE_BACKEND_ADDR;
   let port = import.meta.env.VITE_BACKEND_PORT;

   let responseResidents = await fetch('http://' + addr + ':' + port + '/api/residents');
   let residentsData = await responseResidents.json();

   console.log("Residents From DB:", residentsData);

   let responseTimestamps = await fetch('http://' + addr + ':' + port + '/api/timestamps');
   let timestampsData = await responseTimestamps.json();
   
   console.log("Timestamps From DB: ", timestampsData);

   let residentsOut:STimestampResident[] = [];
   for (let timestamp of timestampsData) {
  
      if (timestamp.date === null) {
         continue;
      }
      let resident = residentsData.find((resident:SResident) => resident.rfid === timestamp.rfid);
   
      if (resident === undefined) {
         continue;
      }

      let responseLocation = await fetch("http://" + addr + ":" + port + "/api/locations/" + timestamp.dest);
      let locationData = await responseLocation.json();
   
      if (locationData === undefined && locationData.name === undefined) {
         continue;
      }
      
      let timestampResident:STimestampResident = {
         rfid: resident.rfid,
         name: resident.name,
         doc: resident.doc,
         room: resident.room,
         unit: resident.unit,
         timestampLeft: timestamp.time,
         destinationId: timestamp.dest,
         destinationLabel: locationData.name
      }
      residentsOut.push(timestampResident);
   }

   
   console.log("Result Of Getting Residents Out: ", residentsOut);

   /**
    * Get the residents who are still out.
    */  
   let priorityOutResidents = getPriorityOutResidents([...residentsOut]);

   /**
    * Get the residents who have returned.
    */
   let residentStamps:STimestampResident[][] = getRoundTrips([...residentsOut]).reverse();
      
   console.log("Resident Stamps: ", residentStamps);
   console.log("Priority Stamps: ", priorityOutResidents);

   let rstamps:STimestampResident[] = [];

   residentStamps.forEach((residentArr) => {
      rstamps.push(...residentArr);
   });

   let returnObj = {data:rstamps, priorityData:priorityOutResidents};
   return returnObj;

}


