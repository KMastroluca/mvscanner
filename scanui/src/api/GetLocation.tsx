import { Component, JSXElement, createSignal } from "solid-js";
import { API } from "./API";


interface GetLocationProps {
  id: number;
}

export const GetLocationById: Component<GetLocationProps> = (props: GetLocationProps): JSXElement => {


  let [locationName, setLocationName] = createSignal<string | number | undefined>("");


  API.GET(`locations/${props.id}`).then((response) => {
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
