let scannedRFID = "";
let lastKeyPress = 0;
let scanLocation = localStorage.getItem("scanLocation");

if (scanLocation === null) {
  scanLocation = prompt("Please enter the scan location");
  localStorage.setItem("scanLocation", scanLocation);
}
/*
 * Resident object
 * @param {String} name
 * @param {String} doc
 * @param {String} room
 * @param {int} unit
 * @param {String} rfid
 * @returns {Resident}
 * */
class Resident {
  name;
  doc;
  room;
  unit;
  rfid;
}

document.body.addEventListener("keydown", (event) => {
  const currentTime = new Date().getTime();

  if (/^\d$/.test(event.key)) {
    scannedRFID += event.key;
    lastKeyPress = currentTime;
  } else if (event.key === "Enter") {
    if (scannedRFID.length === 17 && currentTime - lastKeyPress < 100) {
      handleScan(scannedRFID);
    }
    scannedRFID = "";
  } else {
    scannedRFID = "";
  }
});

async function handleScan(rfid) {
  const apiUrl = `http://172.16.20.42/api/residents/${rfid}`;

  try {
    const response = await fetch(apiUrl);
    if (!response.ok) {
      throw new Error(`HTTP error! Status: ${response.status}`);
    }

    const data = await response.json();
    if (data === null) {
      promptResident(rfid);
      return;
    } else {
      alert(`Resident ${data.name} has been scanned in at ${scanLocation}`);
    }
  } catch (error) {
    console.error("Error fetching resident data:", error);
  }
}

/*** User hasn't been seen yet:
 * Prompt user to enter resident details
 * @param {String} rfid
 * @returns {Promise<Resident>}
 **/
async function promptResident(rfid) {
  let name = prompt("Please enter the resident's name");
  let doc = prompt("Please enter the resident's doctor");
  let room = prompt("Please enter the resident's room number");
  let unit = prompt("Please enter the resident's unit number");
  let user = new Resident(name, doc, room, unit, rfid);
  try {
    const response = await fetch("http://localhost:8080/api/residents", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(user),
    });
    if (!response.ok) {
      throw new Error(`HTTP error! Status: ${response.status}`);
    }
  } catch (error) {
    console.error("Error adding resident:", error);
  }
}
