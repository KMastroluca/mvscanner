import { STimestamp } from "../types/Models";
import { API } from "./API";

export const createTimestamp = async (newTimestamp: STimestamp) => {

  const uri = "timestamps";
  // Format Timestamp to match backend
  let timestampPayload = {
    rfid: newTimestamp.rfid,
    location: newTimestamp.destinationId
  };

  let timestampResponse = await API.POST(uri, timestampPayload);

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
