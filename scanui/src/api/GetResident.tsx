import { SResident } from "../types/Models";


export const GetResidentByRFID = async (rfid:string):Promise<SResident|null> => {

   let addr = import.meta.env.VITE_BACKEND_ADDR;
   let port = import.meta.env.VITE_BACKEND_PORT;

   let response = await fetch(`http://${addr}:${port}/api/residents/${rfid}`);
   let data = await response.json();

   if (data.success !== true) {
      console.warn("GetResidentByRFID: unable to find resident with RFID: " + rfid);   
      return null;
   }

   return data.data;
   
};