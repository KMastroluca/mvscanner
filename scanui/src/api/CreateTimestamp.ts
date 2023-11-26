import {STimestamp} from "../types/Models";
import { POST } from "./API";

export const createTimestamp = async (newTimestamp:STimestamp) => {

   let addr = import.meta.env.VITE_BACKEND_ADDR;
   let port = import.meta.env.VITE_BACKEND_PORT;

   let api = "http://" + addr + ":" + port + "/api/timestamps";
   
   // Format Timestamp to match backend
   let timestampPayload = {
      rfid:newTimestamp.rfid,
      location:newTimestamp.destinationId
   };

   let timestampResponse = await POST(api, timestampPayload);

   if (!timestampResponse) {
      console.error("Error: No response from server");
      return;
   }

   if (!timestampResponse.success) {
      console.warn("Warning: Timestamp not created");
      console.warn(timestampResponse.message);
      return;
   } else {
      console.log("Timestamp Created");
      if (timestampResponse.data) {
         console.log(timestampResponse.data);
      }
   }
   
   
};