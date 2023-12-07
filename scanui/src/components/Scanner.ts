import { API } from "../api/API";
import toast, { Toaster } from "solid-toast";

/**
 * Here we declare a window global which holds the scannedRFID string.
 */
declare global {
  interface Window {
    facilityLocationId: number;
    scannedRFID: string;
    lastScannedRFID: string;
    lastKeyPress: number;
    scanApiUrl: string;
    testScanMode: boolean;
  }
}

window.testScanMode = true;

interface ScannerProps {
  displayNewResidentModal: (rfid: string) => void;
  refetchData: () => void;
}

export const initScanner = (props: ScannerProps) => {
  console.log("Attaching Scanner Event Listeners");


  /**
   * Make sure that the facilityLocationId is set.
   * if there is an error setting, it we just reload the page.
   * So the user must try again.
   */
  if (!localStorage.getItem("facilityLocationId")) {
    window.facilityLocationId = parseInt(prompt("Enter Facility Location ID: ", "0")!, 10);
    if (isNaN(window.facilityLocationId)) {
      alert("Invalid Facility Location ID");
      location.reload();
    } else {
      localStorage.setItem("facilityLocationId", window.facilityLocationId.toString());
    }
  } else {
    window.facilityLocationId = parseInt(localStorage.getItem("facilityLocationId")!, 10);
  }


  window.scannedRFID = "";

  window.addEventListener("keydown", (event: KeyboardEvent) => {
    const currentTime = new Date().getTime();

    if (/^\d$/.test(event.key)) {
      window.scannedRFID += event.key;
      window.lastKeyPress = currentTime;
    } else if (event.key === "Enter") {
      console.log("Enter Pressed");
      if (window.scannedRFID.length === 17 && currentTime - window.lastKeyPress < 100) {
        console.log("Scanned RFID: ", window.scannedRFID);
        handleScan(window.scannedRFID, props);
      } else if (window.testScanMode === true) {
        console.log("Executing Test Scan");
        handleScan("00000000000000000", props);
      }
      window.lastScannedRFID = window.scannedRFID;
      window.scannedRFID = "";
    }
  });
}

export const cleanupScanner = () => {
  window.removeEventListener("keydown", () => { });

  window.scannedRFID = "";
  window.lastKeyPress = 0;
  window.facilityLocationId = 0;
  window.scanApiUrl = "";
};


export const handleScan = async (rfid: string, props: ScannerProps) => {
  window.scanApiUrl = `http://172.16.20.42:8080/api/timestamps`;


  try {

    const originalResidentResponse = await API.GET(`residents/${rfid}`);

    const response = await fetch(window.scanApiUrl, {
      method: "POST",
      headers: API.headers,
      body: JSON.stringify({
        rfid: rfid,
        location: window.facilityLocationId
      }),
    });


    if (!response.ok) {
      throw Error(response.statusText);
    }


    const data = await response.json();
    console.log("Scan Response: ", data);





    if (data.success === false) {
      // If the scan was not successful in this case, that means the resident is not in the database
      // Prompt the user to add the resident to the database
      let addResident = window.confirm("Resident Not Found, Add Resident?");
      if (addResident) {
        props.displayNewResidentModal(rfid);
      }
      return;
    }

    if (data.data.at(0).resident.current_location === 0) {
      // Resident is leaving, prompt user for location
      let dest = window.prompt("Enter Destination: ", "1");
      if (dest === null) {
        toast.error("Invalid Destination, Scan Again");
        return;
      }

      if (isNaN(parseInt(dest, 10))) {
        toast.error("Invalid Destination, Scan Again");
        return;
      }




      let residentResp = await API.GET(`residents/${rfid}`);
      if (!residentResp) {
        toast.error("Error: No response from server when fetching resident");
        return;
      }
      if (!residentResp.success) {
        toast.error(residentResp.message);
        return;
      }


      let currentlocationResp = await API.GET(`locations/${residentResp!.data!.at(0)!.current_location}`);
      if (!currentlocationResp) {
        toast.error("Error: No response from server when fetching current location");
        return;
      }
      if (!currentlocationResp.success) {
        toast.error(currentlocationResp.message);
        return;
      }


      let response = await API.POST("timestamps", { location: parseInt(dest, 10), rfid: rfid });

      if (!response) {
        toast.error("Error: No response from server");
        return;
      }

      if (!response.success) {
        toast("Warning: Timestamp not created");
        toast(response.message);
        return;
      }

      let locationResp = await API.GET(`locations/${dest}`);
      if (!locationResp) {
        toast("Error: No response from server when fetching location");
        return;
      }
      if (!locationResp.success) {
        toast.error("Warning: Location not found, scan again");
        toast.error(locationResp.message);

        return;
      }

      console.log("Timestamp Created: ", response.data);

      props.refetchData();

      console.log("Resident: ", residentResp.data);


      toast.success(`Resident ${originalResidentResponse?.data!.at(0)?.name} Leaving Pod for ${locationResp!.data!.at(0)!.name}`);
    } else {
      console.log("Resident Arriving at: ", data.data);
      let arrivingLocation = await API.GET(`locations/${window.facilityLocationId}`);
      if (!arrivingLocation) {
        toast.error("Error: No response from server when fetching location");
        return;
      }
      if (!arrivingLocation.success) {
        toast.error(arrivingLocation.message);
      }
      toast.success(`Resident ${originalResidentResponse!.data!.at(0)!.name} Arriving at ${arrivingLocation!.data!.at(0)!.name}`);
      props.refetchData();
    }




  } catch (error) {
    console.error("Error Fetching Resident Data After Scan: ", error);
    return;
  }
}
