import { SResident } from "../types/Models";

export const createResident = async (newResident: SResident) => {
  let addr = import.meta.env.VITE_BACKEND_ADDR;
  let port = import.meta.env.VITE_BACKEND_PORT;

  let api = "http://" + addr + ":" + port + "/api/residents";

  let residentResponse = await fetch(api, {
    method: "POST",
    headers: {
      accept: "application/json",
      "Content-Type": "application/json",
    },
    body: JSON.stringify(newResident),
  });

  let resp = await residentResponse.json();

  if (resp.success === false) {
    console.error("Error: Resident not created");
    console.error(resp.message);
    return;
  } else {
    console.log(resp.message);
  }
};
