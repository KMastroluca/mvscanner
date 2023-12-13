import toast from "solid-toast";
import { SResident } from "../types/models";
import { API } from "./API";



export const updateResident = async (rfid:string, resident:SResident):Promise<boolean> => {

   let response = await API.PATCH(`/api/residents/${rfid}`, resident);
   console.log("updateResident: response: ", response);
   if (response === null) {
      console.warn("updateResident: unable to update resident with RFID: " + rfid);   
      return false;
   }
   if (response!.success !== true) {
      console.warn("updateResident: unable to update resident with RFID: " + rfid);   
      return false;
   }

   toast.success("Resident Updated Successfully", {duration: 1000});

   return true;
}