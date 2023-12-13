/*
 * Copyright (c) {11/8/23, 3:09 PM} Lorenzo A. Banks and Preston Thorpe. All rights reserved.
 * {App.tsx}
 * {App.tsx}
 *
 * This software is protected by copyright laws and international copyright treaties, as
 * well as other intellectual property laws and treaties. The software is licensed, not sold.
 *
 * However, you are not permitted to use the software as is, or distribute it without
 * obtaining a license from the authors. Unauthorized use of the software may result in
 * severe civil and criminal penalties, and will be prosecuted to the maximum extent
 * possible under law.
 *
 * THIS SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
 *  BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 *  ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 *  CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 *  SOFTWARE.
 *
 * In no case shall the authors or copyright holders be liable for any claim, damages or
 * other liability arising from, out of or in connection with the softwareor the use or
 * other dealings in the software.
 * ********************************************************************************
 */
// Solid Imports
import {
  For,
  createEffect,
  createResource,
  createSignal,
  onCleanup,
  onMount,
} from "solid-js";

import "./App.css";

// API Imports
import { getResidentsOut, getResidentsIn } from "./api/ResidentInOut";
import { createResident } from "./api/CreateResident";
import { GetResidentByRFID } from "./api/GetResident";
import { updateResident } from "./api/UpdateResident";
import { GetAllLocations } from "./api/GetLocation";
// imported but not used
// import { createTimestamp } from './api/CreateTimestamp';

// Components
import toast, { Toaster } from "solid-toast";
import { STable } from "./components/STable";
import { initScanner, cleanupScanner } from "./components/Scanner";
import { ResidentIDModal } from "./components/ResidentIDModal";
import { ResidentEditModal } from "./components/EditResidentModal";

// Assets
import loadingAnim from "./assets/loading.gif";
// imported but not used
// import { BiSolidRightArrowCircle } from 'solid-icons/bi';

// Enums
// imported but not used
// import { AppDisplayHousingUnit } from './types/enums';

// Types
import { STableAction } from "./types/tableTypes";
import { SLocation, SResident } from "./types/models";

// Date
import DatePicker, { PickerValue, utils } from "@rnwonder/solid-date-picker";

function App() {
  // --------------------------- State Management ---------------------------

  // Resource States (Async)
  const [outResidentsData, { refetch: refetchOutResidents }] = createResource(
    getResidentsOut,
    { initialValue: { data: [], priorityData: [] } }
  );
  const [inResidentsData, { refetch: refetchInResidents }] = createResource(
    getResidentsIn,
    { initialValue: { data: [] } }
  );

  // Modal States
  const [appNewResidentModalOpen, setAppNewResidentModalOpen] =
    createSignal(false);
  const [appEditResidentModalOpen, setAppEditResidentModalOpen] =
    createSignal(false);

  // Resident States
  const [appNewResidentModalRFID, setAppNewResidentModalRFID] =
    createSignal("");
  const [appEditResident, setAppEditResident] = createSignal<SResident | null>(
    null
  );

  // Facility Location States
  const [facilityLocations, setFacilityLocations] = createSignal<SLocation[]>(
    []
  );
  const [selectedLocation, setSelectedLocation] =
    createSignal<SLocation | null>(null);

  // --------------------------- Location Functions ---------------------------

  // Function to set the default selected location
  const setDefaultSelectedLocation = () => {
    if (window.facilityLocationId) {
      const loc = facilityLocations().find(
        (loc) => loc.id === window.facilityLocationId
      );
      if (loc) {
        setSelectedLocation(loc);
      }
    }
  };

  // Function to resolve the display name for a location
  const resolveLocationNameForMenu = (location: SLocation): string => {
    if (location.name === "SIGNED_OUT") {
      return "FACILITY WIDE";
    }

    // Otherwise, we take the string, split it by underscores, and then join it with spaces.
    const nameParts = location.name.split("_");
    return nameParts.join(" ").toUpperCase();
  };

  // Function to count residents in a location
  const getCountOfResidentsInLocation = (location: SLocation): number => {
    let count = 0;
    for (let i = 0; i < inResidentsData().data.length; i++) {
      let resident = inResidentsData().data.at(i);
      if (!resident) {
        continue;
      }
      if (resident.current_location === location.id) {
        count++;
      }
    }
    return count;
  };

  // Function to handle the selection of a location
  const handleSelectLocation = (location: SLocation) => {
    console.log("SELECTED LOCATION: ", location);
    setSelectedLocation(location);
  };

  // Function to load facility locations
  const loadFacilityLocations = async () => {
    let locs = await GetAllLocations();
    setFacilityLocations(locs);
  };

  // --------------------------- Location Effects ---------------------------

  // Effect to set the default location
  createEffect(() => {
    console.log("Attempting to set the default location.");
    if (facilityLocations().length > 0) {
      console.log("Setting the default location.");
      setDefaultSelectedLocation();
      return;
    }
    console.log("Unable to set the default location.");
  }, [facilityLocations]);

  // --------------------------- Modals ---------------------------

  // Create
  const displayNewResidentModal = (rfid: string) => {
    console.log("DISPLAY NEW RESIDENT MODAL RN");
    setAppNewResidentModalOpen(true);
    setAppNewResidentModalRFID(rfid);
  };

  const handleCloseNewResidentModal = () => {
    setAppNewResidentModalOpen(false);
    setAppNewResidentModalRFID("");
  };

  // Edit

  const displayEditResidentModal = async (rfid: string) => {
    toast("Loading Resident Data...", { duration: 1000 });
    let residentReference: SResident | null = await GetResidentByRFID(rfid);
    if (residentReference === null) {
      console.error("Unable to find resident with RFID: " + rfid);
      return;
    }
    console.log("EDIT RESIDENT FOUND RFID: " + rfid, residentReference);
    setAppEditResident(residentReference);
    setAppEditResidentModalOpen(true);
  };

  const handleCloseEditResidentModal = () => {
    setAppEditResidentModalOpen(false);
    setAppEditResident(null);
  };

  const handleEditResidentDone = (resident: SResident) => {
    // Make API call to update resident
    let result = updateResident(resident.rfid, resident);
    console.log("RESULT: ", result);

    // Refetch the data
    refetchOutResidents();
    refetchInResidents();
  };

  //  --------------------------- Date Range  ---------------------------
  const defaultDateRangeFrom = {
    year: new Date().getFullYear(),
    month: new Date().getMonth(),
    day: new Date().getDate(),
  };

  const defaultDateRangeTo = {
    year: new Date(+new Date() + 86400000).getFullYear(),
    month: new Date(+new Date() + 86400000).getMonth(),
    day: new Date(+new Date() + 86400000).getDate(),
  };

  const [dateRange, setDateRange] = createSignal<PickerValue>({
    value: {
      startDateObject: defaultDateRangeFrom,
      endDateObject: defaultDateRangeTo,
    },
    label: "From -> To",
  });

  // --------------------------- Lifecycle Hooks ---------------------------
  onMount(() => {
    console.log(
      "Component mounted, initializing scanner and loading locations."
    );

    initScanner({ displayNewResidentModal, refetchData });

    // Load the facility locations and log the data
    loadFacilityLocations().then((locations) => {
      console.log("Loaded Facility Locations:", locations);
    });
  });

  onCleanup(() => {
    cleanupScanner();
  });

  // --------------------------- Resident Actions ---------------------------
  const residentActions: STableAction[] = [
    {
      actionLabel: "Edit",
      actionFunction: (props: { rfid: string }) => {
        console.log("Execute Edit Action/Pull Up Edit Modal");
        displayEditResidentModal(props.rfid);
      },
    },
  ];

  const handleCreateNewResident = (newResident: SResident) => {
    createResident(newResident);
  };

  const refetchData = () => {
    refetchInResidents();
    refetchOutResidents();
  };

  // --------------------------- Debugging Effects ---------------------------
  createEffect(() => {
    console.log("Out Residents Data:", outResidentsData());
  });

  createEffect(() => {
    console.log("In Residents Data:", inResidentsData());
  });

  return (
    <div class={"flex flex-col w-screen h-screen"}>
      <Toaster
        position="bottom-right"
        gutter={8}
        containerClassName=""
        toastOptions={{
          className: "",
          duration: 5000,
          style: {
            background: "#363636",
            color: "#fff",
          },
        }}
      />

      {appNewResidentModalRFID() ? (
        <ResidentIDModal
          close={handleCloseNewResidentModal}
          open={appNewResidentModalOpen}
          create={handleCreateNewResident}
          rfid={appNewResidentModalRFID()}
        />
      ) : (
        false
      )}

      {appEditResidentModalOpen() ? (
        <ResidentEditModal
          close={handleCloseEditResidentModal}
          open={appEditResidentModalOpen}
          currentResident={appEditResident}
          editResident={handleEditResidentDone}
        />
      ) : (
        false
      )}

      <div class="flex flex-row w-full h-12 sticky top-0 z-10 px-10 py-2 justify-center space-x-4">
        <div class={"flex flex-row py-4 h-12 items-center justify-start gap-8"}>
          <div class={"flex items-center justify-center h-full w-[80px]"}>
            <label for={"facilityLocationSelect"} class={"text-lg font-bold"}>
              Facility Location
            </label>
          </div>
          <div class={"flex items-center justify-center"}>
            <select
              name="facilityLocationSelect"
              value={selectedLocation()?.id}
              class={
                "flex justify-start items-center rounded border-gray-300 border-2  px-2 py-2 "
              }
            >
              <For each={facilityLocations()}>
                {(location) => {
                  let locationName = resolveLocationNameForMenu(location);
                  let count = 0;
                  if (location.id === 0) {
                    locationName = "FACILITY WIDE";
                    count = inResidentsData().data.length;
                  } else {
                    count = getCountOfResidentsInLocation(location);
                  }
                  return (
                    <option
                      value={location.id}
                      class={`text-lg ${
                        location.id === 0 ? "font-bold" : false
                      }`}
                      onClick={() => handleSelectLocation(location)}
                    >
                      {locationName} ({count})
                    </option>
                  );
                }}
              </For>
            </select>
          </div>
          <div class={"flex justify-center items-center text-center"}>
            <label class={"text-md font-bold"}>
              Search Range <br /> [
              {utils()
                .convertDateObjectToDate(dateRange().value.startDateObject!)
                .toLocaleDateString()}{" "}
              --{" "}
              {utils()
                .convertDateObjectToDate(dateRange().value.endDateObject!)
                .toLocaleDateString()}
              ]
            </label>
          </div>
          <div class={"flex flex-row justify-center items-center h-12"}>
            <DatePicker
              type="range"
              value={dateRange}
              setValue={setDateRange}
              maxDate={utils().getToday()}
              minDate={utils().convertDateToDateObject(
                new Date(new Date().getDate() + 1)
              )}
              inputClass={
                "text-sm h-10 border-[3px] border-neutral-400 rounded"
              }
              inputWrapperWidth={"8rem"}
            />
          </div>
        </div>
      </div>
      <div class="flex flex-col md:flex-row w-full px-2 py-3 overflow-auto sm:overflow-visible">
        <div class="flex flex-col w-full md:w-1/2 justify-center items-center mb-4 md:mb-0 mx-2">
          <h2 class="mb-4">Residents Out</h2>
          {outResidentsData.loading ? (
            <div class="flex justify-center items-center">
              <img src={loadingAnim} class="w-10 h-10" />
            </div>
          ) : (
            <STable type="TimestampResident" data={outResidentsData()} />
          )}
        </div>

        <div class="flex flex-col w-full md:w-1/2 justify-center items-center mx-2">
          <h2 class="mb-4">Residents In</h2>
          {inResidentsData.loading ? (
            <div class="flex justify-center items-center">
              <img src={loadingAnim} class="w-10 h-10" />
            </div>
          ) : (
            <STable
              type="Resident"
              data={inResidentsData()}
              actions={residentActions}
            />
          )}
        </div>
      </div>
    </div>
  );
}

export default App;
