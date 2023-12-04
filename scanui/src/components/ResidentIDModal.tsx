import { Accessor, Component, Show, createEffect, createMemo, createSignal } from "solid-js";
import { Portal } from "solid-js/web";
import { createResident } from "../api/CreateResident";
import { SResident } from "../types/Models";


export enum ResidentIDModalHousingUnit {
  ALPHA = 1,
  BRAVO = 6,
  CHARLIE = 7,
  DELTA = 14,
  ECHO = 17
};



interface ResidentIDModalProps {
   open: Accessor<boolean>;
   rfid: string;
   close: () => void;
   create:(newResident:SResident) => void;
}

type ResidentIDModalValidationErrors = {
   firstNameError:string|null,
   lastNameError:string|null,
   docError:string|null,
   housingUnitError:string|null,
   roomError:string|null,
   bunkError:string|null,
};


export const ResidentIDModal: Component<ResidentIDModalProps> = (props: ResidentIDModalProps) => {

   const [newResidentHousingUnit, setNewResidentHousingUnit] = createSignal<number|string|1|6|7|14|17>(1);
   const [newResidentRoom, setNewResidentRoom] = createSignal<number|string>(0);
   const [newResidentBunk, setNewResidentBunk] = createSignal<"T"|"B"|null>(null);
   const [newResidentFirstName, setNewResidentFirstName] = createSignal<string>("");
   const [newResidentLastName, setNewResidentLastName] = createSignal<string>("");
   const [newResidentDOC, setNewResidentDOC] = createSignal<string>("");
   const [newResidentPod, setNewResidentPod] = createSignal<string>("");

   const [newResidentValidationErrors, setNewResidentValidationErrors] = createSignal<ResidentIDModalValidationErrors>({
      firstNameError: null,
      lastNameError: null,
      docError: null,
      housingUnitError: null,
      roomError: null,
      bunkError: null,
   });

   createEffect(() => {
      console.log("A Form Value Changed!");
      validateNewResident();
   }, [newResidentFirstName, newResidentLastName, newResidentDOC, newResidentHousingUnit, newResidentRoom, newResidentBunk]);

   const handleChangeHousingUnit = (e:any) => {
      console.log(e.currentTarget.value);
      setNewResidentHousingUnit(e.currentTarget.value);
   };

  const handleChangePod = (e: any) => {
    setNewResidentPod(e.target.value);
  }


  const handleChangeRoom = (e: any) => {
    setNewResidentRoom(e.target.value);
  };

  const handleChangeBunk = (e: any) => {
    setNewResidentBunk(e.target.value);
  };

  const handleChangeFirstName = (value: string) => {
    setNewResidentFirstName(value);
  }

  const handleChangeLastName = (value: string) => {
    setNewResidentLastName(value);
  }

   const handleChangeDOC = (value:string) => {
      setNewResidentDOC(value);
   }

   const validateNewResident = () => {
      let errors = {};

      if (newResidentFirstName() === "") {
         errors = {
            ...errors,
            firstNameError: "First Name is required"
         };
      }

      if (newResidentLastName() === "") {
         errors = {
            ...errors,
            lastNameError: "Last Name is required"
         };
      }

      if (/^[a-zA-Z\s]*$/.test(newResidentFirstName()) === false ) {
         errors = {
            ...errors,
            firstNameError: "First Name must contain only letters!"
         };
      }

      if (/^[a-zA-Z\s]*$/.test(newResidentLastName()) === false ) {
         errors = {
            ...errors,
            lastNameError: "Last Name must contain only letters!"
         };
      }

      if (newResidentDOC() === "") {
         errors = {
            ...errors,
            docError: "DOC # is required"
         };
      }

      if (/^\d+$/.test(newResidentDOC()) === false) {
         errors = {
            ...errors,
            docError: "DOC # must contain only numbers!"
         };
      }

      if (newResidentHousingUnit() === null) {
         errors = {
            ...errors,
            housingUnitError: "Housing Unit is required"
         };
      }

      let housingUnits = [1, 6, 7, 14, 17];
      let a = newResidentHousingUnit();
      

      if (housingUnits.includes(parseInt(newResidentHousingUnit() as string)) === false) {
         errors = {
            ...errors,
            housingUnitError: "Housing Unit is invalid! " + newResidentHousingUnit().toString()
         };
      }

      if (parseInt(newResidentRoom() as string) < 0) {
         errors = {
            ...errors,
            roomError: "Room is required"
         };
      }

      if (newResidentBunk() === null) {
         errors = {
            ...errors,
            bunkError: "Bunk is required"
         };
      }

      if (newResidentBunk() !== "T" && newResidentBunk() !== "B") {
         errors = {
            ...errors,
            bunkError: "Bunk is invalid!"
         };
      }

      setNewResidentValidationErrors(errors as ResidentIDModalValidationErrors);
   };

   const housingUnitLetter = (housingUnit:number):string|"A"|"B"|"C"|"D"|"E"|undefined => {
      console.log("HOUSING UNIT LETTER", housingUnit);
      if (housingUnit == 1) {
         return "A";
      } else if (housingUnit == 6) {
         return "B";
      }
      else if (housingUnit == 7) {
         return "C";
      }
      else if (housingUnit == 14) {
         return "D";
      }
      else if (housingUnit == 17) {
         return "E";
      }
      else {
         return undefined;
      }
   };

   const housingRoom = (room:number) => {
      
      let roomString = room.toString();
      let housingRoom = housingUnitLetter(newResidentHousingUnit() as number) + roomString;
      return housingRoom + "-" + newResidentBunk();
   };

   const createNewResident = () => {
      console.log("Create New Resident From Modal");
      
      if (newResidentValidationErrors().bunkError !== undefined ||
       newResidentValidationErrors().docError !== undefined || 
       newResidentValidationErrors().firstNameError !== undefined ||
       newResidentValidationErrors().housingUnitError !== undefined || 
       newResidentValidationErrors().lastNameError !== undefined || 
       newResidentValidationErrors().roomError !== undefined) {
         alert("Whoops!, Please fix the errors on the form before submitting! "); 
         console.log(newResidentValidationErrors());
         return;
      }

      // Reassemble the housing unit/bunk 



      let newResident = {
         rfid: props.rfid,
         name: newResidentFirstName()+" "+newResidentLastName(),
         doc: newResidentDOC(),
         unit: parseInt(newResidentHousingUnit() as string),
         room: housingRoom(newResidentRoom() as number),
         current_location: parseInt(newResidentHousingUnit() as string),
      } as SResident;


      Object.values(newResident).forEach((value, index) => {
         if (value === null || value === undefined || value === "") {
            alert("Whoops!, the resident payload had an empty or invalid value: "+value +" index:"+index); 
            return;
         }
      });

      console.log("Trying To Create New Resident:", newResident);
      props.create(newResident);
      props.close();

   };


  const Modal = () => {

      return (
      <Show when={props.open}>
         <Portal mount={document.body}>
            <div class={"fixed top-0 left-0 right-0 bottom-0 bg-black opacity-70 z-40"} />
            <div class={"flex flex-col w-3/5 h-fit fixed top-[50%] left-[50%] bg-white translate-x-[-50%] translate-y-[-50%] z-50"}>
               <header class={"flex flex-col w-full h-20 bg-slate-400 items-start justify-center px-6"}>
                  <h2 class={"text-2xl font-extrabold text-slate-100"}>NEW RESIDENT</h2>
               </header>
               <section class="flex flex-col gap-5 w-full h-fit bg-slate-50 px-10 py-3">
                  <section class={"flex w-full h-fit"}>
                     <div class={"flex flex-col gap-2 w-full"}>
                        <label class={"text-lg font-bold"}>RFID:</label>
                        <input value={props.rfid} type={"text"} disabled class={"bg-neutral-200 border-[1px] py-3 px-3"} id={"newRFID"} name={"newRFID"} />
                     </div>
                  </section>
                  <section class={"flex w-full h-fit columns-2 gap-4"}>
                     <div class={"flex w-full"}>
                        <div class="flex flex-col gap-2 w-full">
                           <label for={"newResidentFirstName"} class={"text-lg font-bold"}>Resident First Name:</label>
                           <input value={newResidentFirstName()} onChange={(e) => handleChangeFirstName(e.target.value)} class={"bg-white border-[1px] py-3 px-3"} type={"text"} id={"newResidentFirstName"} name={"newResidentFirstName"} placeholder="First Name" />
                           {newResidentValidationErrors().firstNameError !== null ? <span class={"text-red-500"}>{newResidentValidationErrors().firstNameError}</span> : false}
                        </div>
                     </div>
                     <div class={"flex w-full"}>
                        <div class="flex flex-col gap-2 w-full">
                           <label for={"newResidentLastName"} class={"text-lg font-bold"}>Resident Last Name:</label>
                           <input value={newResidentLastName()} onChange={(e) => handleChangeLastName(e.target.value)} class={"bg-white border-[1px] py-3 px-3"} type={"text"} id={"newResidentLastName"} name={"newResidentLastName"} placeholder="Last Name" />
                           {newResidentValidationErrors().lastNameError !== null ? <span class={"text-red-500"}>{newResidentValidationErrors().lastNameError}</span> : false}
                        </div>
                     </div>      
                  </section>
                  <section class={"flex w-full h-fit"}>
                     <div class={"flex flex-col gap-2 w-full"}>
                        <label class={"text-lg font-bold"}>DOC #:</label>
                        <input value={newResidentDOC()} onChange={(e) => handleChangeDOC(e.target.value)} type={"text"} class={"bg-white border-[1px] py-3 px-3"} id={"newDOC"} name={"newDOC"} />
                        {newResidentValidationErrors().docError !== null ? <span class={"text-red-500"}>{newResidentValidationErrors().docError}</span> : false}
                     </div>
                  </section>
                  <section class={"flex w-full h-fit gap-4 columns-3"}>
                        <div class={"flex gap-2 w-full"}>
                           <label class={"flex flex-col w-full"}>
                              <span class={"text-lg font-bold"}>Housing Unit:</span>
                              <select value={newResidentHousingUnit()} onChange={(e) => handleChangeHousingUnit(e)} name={"newResidentUnit"} class={"px-3 py-3 border-[1px]"}>
                                 <option value={1}>ALPHA UNIT</option>
                                 <option value={6}>BRAVO UNIT</option>
                                 <option value={7}>CHARLIE UNIT</option>
                                 <option value={14}>DELTA UNIT</option>
                                 <option value={17}>ECHO UNIT</option>
                              </select>
                           </label>
                           {newResidentValidationErrors().housingUnitError !== null ? <span class={"text-red-500"}>{newResidentValidationErrors().housingUnitError}</span> : false}
                        </div>
                        <div class={"flex gap-2 w-full"}>
                           <label class={"flex flex-col w-full"}>
                              <span class={"text-lg font-bold"}>
                                 Pod:
                              </span>
                              <select value={newResidentPod()} name={"newResidentPod"} class={"px-3 py-3 border-[1px]"} onChange={(e) => handleChangePod(e.target.value)}>
                                 <option value={'A'}>A</option>
                                 <option value={'B'}>B</option>
                                 <option value={'C'}>C</option>
                              </select>
                           </label>
                        </div>                       
                        <div class={"flex gap-2 w-full"}>
                           <label class={"flex flex-col w-full"}>
                              <span class={"text-lg font-bold"}>Room:</span>
                              <select value={newResidentRoom()} name={"newResidentRoom"} class={"px-3 py-3 border-[1px]"} onChange={(e) => handleChangeRoom(e)}>
                                 <option value={"1"}>1</option>
                                 <option value={"2"}>2</option>
                                 <option value={"3"}>3</option>
                                 <option value={"4"}>4</option>
                                 <option value={"5"}>5</option>
                                 <option value={"6"}>6</option>
                                 <option value={"7"}>7</option>
                                 <option value={"8"}>8</option>
                                 <option value={"9"}>9</option>
                                 <option value={"10"}>10</option>    
                                 <option value={"11"}>11</option>
                                 <option value={"12"}>12</option>
                                 <option value={"13"}>13</option>
                                 <option value={"14"}>14</option>
                                 <option value={"15"}>15</option>
                                 <option value={"16"}>16</option>
                                 <option value={"17"}>17</option>
                                 <option value={"18"}>18</option>
                                 <option value={"19"}>19</option>
                                 <option value={"20"}>20</option>       
                                 <option value={"21"}>21</option>
                                 <option value={"22"}>22</option>
                                 <option value={"23"}>23</option>
                                 <option value={"24"}>24</option>
                                 <option value={"25"}>25</option>
                                 <option value={"26"}>26</option>
                                 <option value={"27"}>27</option>
                                 <option value={"28"}>28</option>
                                 <option value={"29"}>29</option>
                                 <option value={"30"}>30</option>    
                                 <option value={"31"}>31</option>
                                 <option value={"32"}>32</option>
                                 <option value={"33"}>33</option>
                                 <option value={"34"}>34</option>
                                 <option value={"35"}>35</option>
                                 <option value={"36"}>36</option>
                                 <option value={"37"}>37</option>
                                 <option value={"38"}>38</option>
                                 <option value={"39"}>39</option>
                                 <option value={"40"}>40</option>   
                              </select>
                           </label>
                           {newResidentValidationErrors().roomError !== null ? <span class={"text-red-500"}>{newResidentValidationErrors().roomError}</span> : false}
                        </div>
                        <div class={"flex gap-2 w-full"}>
                           <label class={"flex flex-col w-full"}>
                              <span class={"text-lg font-bold"}>Bunk:</span>
                              <select value={newResidentBunk()?.toUpperCase() as string} onChange={(e) => handleChangeBunk(e)} name={"newResidentUnit"} class={"px-3 py-3 border-[1px]"}>
                                 <option value={"T"}>TOP</option>
                                 <option value={"B"}>BOTTOM</option>
                              </select>
                           </label>
                           {newResidentValidationErrors().bunkError !== null ? <span class={"text-red-500"}>{newResidentValidationErrors().bunkError}</span> : false}
                        </div>
                     </section>  
               </section>
               <section class="mt-10">
                  <button class={"flex flex-row w-full text-xl h-20 hover:bg-green-300 bg-green-200 items-center justify-center px-6"} onClick={() => createNewResident()}>Create</button>
                  <button class={"flex flex-row w-full text-xl h-20 hover:bg-red-300 bg-red-200 items-center justify-center px-6"} onclick={() => props.close()}>Close</button>
               </section>
            </div>
         </Portal>
      </Show>
    );
  }


  return (
    <Modal />
  );


}
