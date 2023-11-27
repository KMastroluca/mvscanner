import { POST } from "../api/API";

/**
 * Here we declare a window global which holds the scannedRFID string.
 */
declare global {
   interface Window {
      facilityLocationId: number;
      scannedRFID: string;
      lastScannedRFID: string;
      lastKeyPress: number;
      scanApiUrl:string;
   }
}

export const initScanner = () => {
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

   window.addEventListener("keydown", (event:KeyboardEvent) => {
      const currentTime = new Date().getTime();

      if (/^\d$/.test(event.key)) {
         window.scannedRFID += event.key;
         window.lastKeyPress = currentTime;   
      } else if (event.key === "Enter") {
         console.log("Enter Pressed");
         if (window.scannedRFID.length === 17 && currentTime - window.lastKeyPress < 100) {
            console.log("Scanned RFID: ", window.scannedRFID);
            handleScan(window.scannedRFID);
         }
         window.lastScannedRFID = window.scannedRFID;
         window.scannedRFID = "";
      } else {
         window.scannedRFID = "";
      }
   });
}

export const cleanupScanner = () => {
   window.removeEventListener("keydown", () => {});

   window.scannedRFID = "";
   window.lastKeyPress = 0;
   window.facilityLocationId = 0;
   window.scanApiUrl = "";
};


export const handleScan = async (rfid:string) => {
   window.scanApiUrl = `http://localhost:8080/api/timestamps`;


   try {

      const response = await fetch(window.scanApiUrl, {
         method: "POST",
         headers: {
            accept: "application/json",
            "Content-Type": "application/json",
         },
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
      if (data.length === 0) {
         throw Error("[-] No Response After Scan!");
      }

      if (data.success === false) {
         throw Error(data.message);
      }

      if (data.data.at(0).location === 0) {
         // Resident is leaving, prompt user for location
         let dest = window.prompt("Enter Destination: ", "0");
         if (dest === null) {
            return;
         }

         if (isNaN(parseInt(dest, 10))) {
            alert("Invalid Destination, Scan Again");
            return;
         }

         let response = await POST(window.scanApiUrl, {location: parseInt(dest, 10), rfid: rfid});

         if (!response) {
            console.error("Error: No response from server");
            return;
         }

         if (!response.success) {
            console.warn("Warning: Timestamp not created");
            console.warn(response.message);
            return;
         }

         console.log("Timestamp Created: ", response.data);
         
      }




   } catch (error) {
      console.error("Error Fetching Resident Data After Scan: ", error);
      return;
   }
}