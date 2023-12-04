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


import { STableData } from '../components/STable';
import { SLocation, SResident, STimestamp, STimestampResident } from "../types/Models"

import _ from 'lodash';
import { API } from './API';

export const getResidentsIn = async (): Promise<STableData> => {

  let response = await API.GET('residents');

  if (!response) {
    console.error("Error: No response from server");
    return {} as STableData;
  }

  if (!response.success) {
    console.warn("Warning: Residents not retrieved");
    console.warn(response.message);
    return {} as STableData;
  }

  return {
    data: response.data as SResident[],
  } as STableData;
};









export const getResidentsOut = async (): Promise<STableData> => {
  console.log("Executed GetResidentsOut");

  let residentsResponse = await API.GET('residents');

  if (!residentsResponse) {
    console.error("Error: No response from server");
    return {} as STableData;
  }

  if (!residentsResponse.success) {
    console.warn("Warning: Residents not retrieved");
    console.warn(residentsResponse.message);
    return {} as STableData;
  }

  let residentsData: SResident[] = residentsResponse.data as SResident[];

  console.log("Residents From DB:", residentsData);

  let timestampsResponse = await API.GET('timestamps');

  if (!timestampsResponse) {
    console.error("Error: No response from server");
    return {} as STableData;
  }

  if (!timestampsResponse.success) {
    console.warn("Warning: Timestamps not retrieved");
    console.warn(timestampsResponse.message);
    return {} as STableData;
  }


  let timestampsData: STimestamp[] = timestampsResponse.data as STimestamp[];

  console.log("Timestamps From DB: ", timestampsData);

  let residentsOut: STimestampResident[] = [];
  for (let timestamp of timestampsData) {

    if (timestamp.date === null) {
      continue;
    }
    let resident = residentsData.find((resident: SResident) => resident.rfid === timestamp.rfid);

    if (resident === undefined) {
      continue;
    }

    let responseLocation = await API.GET('locations/' + timestamp.location);

    if (!responseLocation) {
      console.error("Error: No response from server");
      return {} as STableData;
    }

    if (!responseLocation.success) {
      console.warn("Warning: Location not retrieved");
      console.warn(responseLocation.message);
      return {} as STableData;
    }

    let locationData = responseLocation.data as SLocation[];

    console.log("Location Data: ", locationData);

    if (locationData.at(0) === undefined && locationData.at(0)!.name === undefined) {
      continue;
    }

    let timestampResident: STimestampResident = {
      rfid: resident.rfid,
      name: resident.name,
      doc: resident.doc,
      room: resident.room,
      unit: resident.unit,
      timestampLeft: timestamp.time!,
      location: timestamp.location,
      destinationLabel: locationData.at(0)!.name
    }
    residentsOut.push(timestampResident);
  }


  console.log("Result Of Getting Residents Out: ", residentsOut);

  let latestTimestamps = getLatestTimestamps(residentsOut);
  let onlyAway = getOnlyAway(latestTimestamps);
  console.log("Latest Timestamps: ", residentsOut);

  let returnObj = { data: latestToOld(residentsOut), priorityData: onlyAway };
  return returnObj;

}



const latestToOld = (data: STimestampResident[]): STimestampResident[] => {
  let sortedData = _.sortBy(data, (timestampResident: STimestampResident) => new Date(timestampResident.timestampLeft).getTime());
  return sortedData;
}

const getLatestTimestamp = (data: STimestampResident[], rfid: string): STimestampResident | undefined => {

  let filteredData = data.filter((timestampResident: STimestampResident) => timestampResident.rfid === rfid);
  let sortedData = _.sortBy(filteredData, (timestampResident: STimestampResident) => new Date(timestampResident.timestampLeft).getTime());
  let latestTimestamp = sortedData.pop();

  return latestTimestamp;
}

const getLatestTimestamps = (data: STimestampResident[]): STimestampResident[] => {
  let rfidList = _.uniq(data.map((timestampResident: STimestampResident) => timestampResident.rfid));
  let latestTimestamps: STimestampResident[] = [];
  for (let rfid of rfidList) {
    let latestTimestamp = getLatestTimestamp(data, rfid);
    if (latestTimestamp !== undefined) {
      latestTimestamps.push(latestTimestamp);
    }
  }
  return latestTimestamps;
}

const getOnlyAway = (data: STimestampResident[]): STimestampResident[] => {
  let filteredData = data.filter((timestampResident: STimestampResident) => timestampResident.destinationLabel === timestampResident.room);
  return filteredData;
}
