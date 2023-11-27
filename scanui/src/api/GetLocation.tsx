import { Component, JSXElement, createSignal } from "solid-js";
import { GET } from "./API";


interface GetLocationProps {
   id:number;
}

export const GetLocationById:Component<GetLocationProps> = (props:GetLocationProps):JSXElement => {

   let addr = import.meta.env.VITE_BACKEND_ADDR;
   let port = import.meta.env.VITE_BACKEND_PORT;


   let [locationName, setLocationName] = createSignal<string|number|undefined>("");


   GET(`http://${addr}:${port}/api/locations/${props.id}`).then((response) => {
      if (response && response!.success && response!.data) {
         setLocationName(response!.data!.at(0)!.name);
      } else {
         setLocationName("Error");
      }
   }).catch((err) => {
      console.error(err);
   });
   
   return (<>{locationName()}</>);


};