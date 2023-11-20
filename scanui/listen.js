let scannedRFID = '';
let lastKeyPress = 0;

document.body.addEventListener('keydown', (event) => {
  const currentTime = new Date().getTime();

  if (/^\d$/.test(event.key)) {
    scannedRFID += event.key;
    lastKeyPress = currentTime;
  } else if (event.key === 'Enter') {
    if (scannedRFID.length === 17 && currentTime - lastKeyPress < 100) {
      handleScan(scannedRFID);
    }
    scannedRFID = '';
  } else {
    scannedRFID = '';
  }
});

async function handleScan(rfid) {
  const apiUrl = `http://localhost:8080/api/residents/${rfid}`;

  try {
    const response = await fetch(apiUrl);
    if (!response.ok) {
      throw new Error(`HTTP error! Status: ${response.status}`);
    }

    const data = await response.json();
    printResident(data);
  } catch (error) {
    console.error('Error fetching resident data:', error);
  }
}

function printResident(res) {
  let name = res["name"];
  let doc = res["doc"];
  let room = res["room"];
  let unit = res["unit"];
  alert(`Name: ${name},\n DOC: ${doc},\n Room: ${room},\n Unit: ${unit} found!`)
}
