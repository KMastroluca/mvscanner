import {STimestamp} from "../types/Models";

export const createTimestamp = async (newTimestamp:STimestamp) => {

   let addr = import.meta.env.VITE_BACKEND_ADDR;
   let port = import.meta.env.VITE_BACKEND_PORT;

   let api = "http://" + addr + ":" + port + "/api/timestamps";
   
   // Format Timestamp to match backend
   let timestampPayload = {
      rfid:newTimestamp.rfid,
      dest:newTimestamp.destinationId
   };

   let timestampResponse = await fetch(api, {
      method:"POST",
      headers:{
         accept:"application/json",
         "Content-Type": "application/json"
      },
      body: JSON.stringify(timestampPayload),
   });

   let resp = await timestampResponse.json();

   console.log(resp);
};