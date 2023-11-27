import { ServerResponse } from "../types/Models";



export const GET = async (url:string):Promise<ServerResponse|undefined> => {
   try {
      const response = await fetch(url, {
         method: "GET",
         headers: {
            accept: "application/json",
            "Content-Type": "application/json",
         },
      });

      if (!response.ok) {
         throw Error(response.statusText);
      }

      return await response.json();
   } catch (err) {
      console.error("GET ERROR:", err);
      return undefined;
   }
};


export const POST = async (url:string, payload:any):Promise<ServerResponse|undefined> => {
   try {
      const response = await fetch(url, {
         method: "POST",
         headers: {
            accept: "application/json",
            "Content-Type": "application/json",
         },
         body: JSON.stringify([payload]),
      });

      if (!response.ok) {
         throw Error(response.statusText);
      }

      return await response.json();
   } catch (err) {
      console.error("POST ERROR:", err);
      return undefined;
   }
};