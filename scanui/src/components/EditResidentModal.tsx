// createEffect was importec but never used, so it was removed
import { Component, Show, createSignal } from "solid-js";
import { Portal } from "solid-js/web";
import { SResident } from "../types/models";

// imported but never used
// import { ResidentIDModalHousingUnit } from "../types/enums";

import {
  DeconstructedResidentRoom,
  ResidentEditModalProps,
  EditResidentModalValidationErrors,
} from "../types/componentTypes";

const parseResidentRoom = (room: string): DeconstructedResidentRoom => {
  const match = room.match(/^([ABC])-([0-9]+)([TB])$/i);

  if (!match) {
    console.error("Invalid room format:", room);
    return {
      // default resident room if no match is found
      podLetter: "",
      roomNumber: 0,
      bunk: "T",
    } as DeconstructedResidentRoom;
  }

  const [, podLetter, roomNumberStr, bunk] = match;
  const roomNumber = parseInt(roomNumberStr, 10);
  console.log("pod", podLetter);
  console.log("roomNumber", roomNumber);
  console.log("bunk", bunk);
  return {
    podLetter,
    roomNumber,
    bunk: bunk as "T" | "B",
  };
};

export const ResidentEditModal: Component<ResidentEditModalProps> = (
  props: ResidentEditModalProps
) => {
  let residentRoomDecons: DeconstructedResidentRoom;
  if (props.currentResident() !== null) {
    residentRoomDecons = parseResidentRoom(props.currentResident()!.room);
  } else {
    residentRoomDecons = {
      podLetter: "",
      roomNumber: 0,
      bunk: "T",
    } as DeconstructedResidentRoom;
  }

  const nameSplit = props.currentResident()?.name.split(" ");
  let firstName: string;
  let lastName: string;

  if (
    nameSplit !== undefined &&
    nameSplit.length > 1 &&
    nameSplit[0] !== undefined &&
    nameSplit[1] !== undefined
  ) {
    firstName = nameSplit[0];
    lastName = nameSplit[1];
  } else {
    firstName = "";
    lastName = "";
  }

  const [residentHousingUnit, setNewResidentHousingUnit] = createSignal<
    number | undefined
  >(props.currentResident()?.unit);
  const [residentRoom, setResidentRoom] = createSignal<number>(
    residentRoomDecons.roomNumber
  );
  const [residentBunk, setResidentBunk] = createSignal<"T" | "B" | null>(
    residentRoomDecons.bunk
  );
  const [residentFirstName, setResidentFirstName] = createSignal<
    string | undefined
  >(firstName);
  const [residentLastName, setResidentLastName] = createSignal<
    string | undefined
  >(lastName);
  const [residentPod, setResidentPod] = createSignal<string | undefined>(
    residentRoomDecons.podLetter
  );
  const [residentDOC, setResidentDOC] = createSignal<string | undefined>(
    props.currentResident()?.doc
  );

  const [residentValidationErrors, setResidentValidationErrors] =
    createSignal<EditResidentModalValidationErrors>({
      firstNameError: null,
      lastNameError: null,
      docError: null,
      housingUnitError: null,
      podError: null,
      roomError: null,
      bunkError: null,
    });

  const handleChangeHousingUnit = (e: any) => {
    setNewResidentHousingUnit(e.target.value);
  };

  const handleChangeRoom = (e: any) => {
    setResidentRoom(e.target.value);
  };

  const handleChangeBunk = (e: any) => {
    setResidentBunk(e.target.value);
  };

  const handleChangeFirstName = (value: string) => {
    setResidentFirstName(value);
  };

  const handleChangeLastName = (value: string) => {
    setResidentLastName(value);
  };

  const handleChangeDOC = (value: string) => {
    setResidentDOC(value);
  };
  // declared but never used
  //
  // const handleFormChange = createEffect(() => {
  //   console.log("FORM CHANGE");
  //   validateResident();
  // }, [residentBunk, residentDOC, residentFirstName, residentLastName, residentRoom, residentHousingUnit]);

  // declared but never used
  //
  // const validateResident = () => {
  //   let errors = {};

  //   if (residentFirstName() === "") {
  //     errors = {
  //       ...errors,
  //       firstNameError: "First Name is required"
  //     };
  //   }

  //   if (residentLastName() === "") {
  //     errors = {
  //       ...errors,
  //       lastNameError: "Last Name is required"
  //     };
  //   }

  //   if (/^[a-zA-Z\s]*$/.test(residentFirstName() as string) === false) {
  //     errors = {
  //       ...errors,
  //       firstNameError: "First Name must contain only letters!"
  //     };
  //   }

  //   if (/^[a-zA-Z\s]*$/.test(residentLastName() as string) === false) {
  //     errors = {
  //       ...errors,
  //       lastNameError: "Last Name must contain only letters!"
  //     };
  //   }

  //   if (residentDOC() === "") {
  //     errors = {
  //       ...errors,
  //       docError: "DOC # is required"
  //     };
  //   }

  //   if ((/^\d+$/.test(residentDOC()!) === false) && residentDOC() !== "") {
  //     errors = {
  //       ...errors,
  //       docError: "DOC # must contain only numbers!"
  //     };
  //   }

  //   if (residentHousingUnit() === null) {
  //     errors = {
  //       ...errors,
  //       housingUnitError: "Housing Unit is required"
  //     };
  //   }

  //   let housingUnits = [1, 6, 7, 14, 17];
  //   let a = residentHousingUnit();

  //   if (residentHousingUnit() !== undefined && housingUnits.includes(residentHousingUnit()! as number) !== false) {
  //     errors = {
  //       ...errors,
  //       housingUnitError: "Housing Unit is invalid! " + residentHousingUnit()?.toString()
  //     };
  //   }

  //   if (residentRoom() < 0) {
  //     errors = {
  //       ...errors,
  //       roomError: "Room is required"
  //     };
  //   }

  //   if (residentBunk() === null) {
  //     errors = {
  //       ...errors,
  //       bunkError: "Bunk is required"
  //     };
  //   }

  //   if (residentBunk() !== "T" && residentBunk() !== "B") {
  //     errors = {
  //       ...errors,
  //       bunkError: "Bunk is invalid!"
  //     };
  //   }

  //   setResidentValidationErrors(errors as ResidentIDModalValidationErrors);
  // };

  const handleChangePod = (value: string) => {
    setResidentPod(value);
  };

  const handleResidentEdit = () => {
    console.log("Edit Existing Resident");

    if (
      residentValidationErrors().firstNameError !== null ||
      residentValidationErrors().lastNameError !== null ||
      residentValidationErrors().docError !== null ||
      residentValidationErrors().housingUnitError !== null ||
      residentValidationErrors().roomError !== null ||
      residentValidationErrors().bunkError !== null
    ) {
      console.log("Form Is Invalid!");
      alert("Form Was Invalid!");
      return;
    }

    let newResident = {
      id: props.currentResident()!.id,
      rfid: props.currentResident()!.rfid,
      name: residentFirstName() + " " + residentLastName(),
      doc: residentDOC(),
      unit: residentHousingUnit(),
      room:
        residentHousingUnit()!.toString() +
        "-" +
        residentRoom().toString() +
        residentBunk(),
      current_location: props.currentResident()!.current_location,
    } as SResident;

    props.editResident(newResident);
  };

  return (
    <Show when={props.open()}>
      <Portal mount={document.body}>
        <div
          class={"fixed top-0 left-0 right-0 bottom-0 bg-black opacity-70 z-40"}
        />
        <div
          class={
            "flex flex-col w-3/5 h-fit fixed top-[50%] left-[50%] bg-white translate-x-[-50%] translate-y-[-50%] z-50"
          }
        >
          <header
            class={
              "flex flex-col w-full h-20 bg-slate-400 items-start justify-center px-6"
            }
          >
            <h2 class={"text-2xl font-extrabold text-slate-100"}>
              EDIT RESIDENT
            </h2>
          </header>
          <section class="flex flex-col gap-5 w-full h-fit bg-slate-50 px-10 py-3">
            <section class={"flex w-full h-fit"}>
              <div class={"flex flex-col gap-2 w-full"}>
                <label class={"text-lg font-bold"}>RFID:</label>
                <input
                  value={props.currentResident()?.rfid}
                  type={"text"}
                  disabled
                  class={"bg-neutral-200 border-[1px] py-3 px-3"}
                  id={"editRFID"}
                  name={"editRFID"}
                />
              </div>
            </section>
            <section class={"flex w-full h-fit columns-2 gap-4"}>
              <div class={"flex w-full"}>
                <div class="flex flex-col gap-2 w-full">
                  <label
                    for={"newResidentFirstName"}
                    class={"text-lg font-bold"}
                  >
                    Resident First Name:
                  </label>
                  <input
                    value={residentFirstName()}
                    onChange={(e) => handleChangeFirstName(e.target.value)}
                    class={"bg-white border-[1px] py-3 px-3"}
                    type={"text"}
                    id={"editResidentFirstName"}
                    name={"editResidentFirstName"}
                    placeholder="First Name"
                  />
                  {residentValidationErrors().firstNameError !== null ? (
                    <span class={"text-red-500"}>
                      {residentValidationErrors().firstNameError}
                    </span>
                  ) : (
                    false
                  )}
                </div>
              </div>
              <div class={"flex w-full"}>
                <div class="flex flex-col gap-2 w-full">
                  <label
                    for={"newResidentLastName"}
                    class={"text-lg font-bold"}
                  >
                    Resident Last Name:
                  </label>
                  <input
                    value={residentLastName()}
                    onChange={(e) => handleChangeLastName(e.target.value)}
                    class={"bg-white border-[1px] py-3 px-3"}
                    type={"text"}
                    id={"editResidentLastName"}
                    name={"editResidentLastName"}
                    placeholder="Last Name"
                  />
                  {residentValidationErrors().lastNameError !== null ? (
                    <span class={"text-red-500"}>
                      {residentValidationErrors().lastNameError}
                    </span>
                  ) : (
                    false
                  )}
                </div>
              </div>
            </section>
            <section class={"flex w-full h-fit"}>
              <div class={"flex flex-col gap-2 w-full"}>
                <label class={"text-lg font-bold"}>DOC #:</label>
                <input
                  value={residentDOC()}
                  onChange={(e) => handleChangeDOC(e.target.value)}
                  type={"text"}
                  class={"bg-white border-[1px] py-3 px-3"}
                  id={"editDOC"}
                  name={"editDOC"}
                />
                {residentValidationErrors().docError !== null ? (
                  <span class={"text-red-500"}>
                    {residentValidationErrors().docError}
                  </span>
                ) : (
                  false
                )}
              </div>
            </section>
            <section class={"flex w-full h-fit gap-4 columns-3"}>
              <div class={"flex gap-2 w-full"}>
                <label class={"flex flex-col w-full"}>
                  <span class={"text-lg font-bold"}>Housing Unit:</span>
                  <select
                    value={residentHousingUnit() as number}
                    onChange={(e) => handleChangeHousingUnit(e)}
                    name={"editResidentUnit"}
                    class={"px-3 py-3 border-[1px]"}
                  >
                    <option value={1}>ALPHA UNIT</option>
                    <option value={6}>BRAVO UNIT</option>
                    <option value={7}>CHARLIE UNIT</option>
                    <option value={14}>DELTA UNIT</option>
                    <option value={16}>ECHO UNIT</option>
                  </select>
                </label>
                {residentValidationErrors().housingUnitError !== null ? (
                  <span class={"text-red-500"}>
                    {residentValidationErrors().housingUnitError}
                  </span>
                ) : (
                  false
                )}
              </div>
              <div class={"flex gap-2 w-full"}>
                <label class={"flex flex-col w-full"}>
                  <span class={"text-lg font-bold"}>Pod:</span>
                  <select
                    value={residentPod() !== undefined ? residentPod() : "A"}
                    name={"editResidentPod"}
                    class={"px-3 py-3 border-[1px]"}
                    onChange={(e) => handleChangePod(e.target.value)}
                  >
                    <option value={"A"}>A</option>
                    <option value={"B"}>B</option>
                    <option value={"C"}>C</option>
                  </select>
                </label>
                {residentValidationErrors().housingUnitError !== null ? (
                  <span class={"text-red-500"}>
                    {residentValidationErrors().housingUnitError}
                  </span>
                ) : (
                  false
                )}
              </div>
              <div class={"flex gap-2 w-full"}>
                <label class={"flex flex-col w-full"}>
                  <span class={"text-lg font-bold"}>Pod:</span>
                  {residentValidationErrors().podError !== null ? (
                    <span class={"text-red-500"}>
                      {residentValidationErrors().podError}
                    </span>
                  ) : (
                    false
                  )}
                  <select
                    value={residentPod()}
                    name={"editResidentPod"}
                    class={"px-3 py-3 border-[1px]"}
                    onChange={(e) => handleChangePod(e.target.value)}
                  >
                    <option value={"A"}>A</option>
                    <option value={"B"}>B</option>
                    <option value={"C"}>C</option>
                  </select>
                </label>
              </div>
              <div class={"flex gap-2 w-full"}>
                <label class={"flex flex-col w-full"}>
                  <span class={"text-lg font-bold"}>Room:</span>
                  <select
                    value={residentRoom()}
                    name={"editResidentRoom"}
                    class={"px-3 py-3 border-[1px]"}
                    onChange={(e) => handleChangeRoom(e)}
                  >
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
                  </select>
                </label>
                {residentValidationErrors().roomError !== null ? (
                  <span class={"text-red-500"}>
                    {residentValidationErrors().roomError}
                  </span>
                ) : (
                  false
                )}
              </div>
              <div class={"flex gap-2 w-full"}>
                <label class={"flex flex-col w-full"}>
                  <span class={"text-lg font-bold"}>Bunk:</span>
                  <select
                    value={residentBunk() as string}
                    onChange={(e) => handleChangeBunk(e)}
                    name={"newResidentUnit"}
                    class={"px-3 py-3 border-[1px]"}
                  >
                    <option value={"T"}>TOP</option>
                    <option value={"B"}>BOTTOM</option>
                  </select>
                </label>
              </div>
            </section>
          </section>
          <section class="mt-10">
            <button
              class={
                "flex flex-row w-full text-xl h-20 hover:bg-green-300 bg-green-200 items-center justify-center px-6"
              }
              onClick={() => handleResidentEdit()}
            >
              Edit Resident
            </button>
            <button
              class={
                "flex flex-row w-full text-xl h-20 hover:bg-red-300 bg-red-200 items-center justify-center px-6"
              }
              onClick={() => props.close()}
            >
              Close
            </button>
          </section>
        </div>
      </Portal>
    </Show>
  );
};
