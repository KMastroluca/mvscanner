/*
 *
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

import { For, createEffect, createResource, createSignal, onCleanup, onMount } from 'solid-js';
import './App.css'
import { getResidentsOut, getResidentsIn } from './api/ResidentInOut';
import { STable, STableAction } from './components/STable';
import { createResident } from './api/CreateResident';
import { createTimestamp } from './api/CreateTimestamp';
import { initScanner, cleanupScanner } from './components/Scanner';
import { ResidentIDModal } from './components/ResidentIDModal';

import { SLocation, SResident } from './types/Models';
import { ResidentEditModal } from './components/EditResidentModal';
import { GetResidentByRFID } from './api/GetResident';
import toast, { Toaster } from 'solid-toast';
import { updateResident } from './api/UpdateResident';

import loadingAnim from './assets/loading.gif';
import { GetAllLocations } from './api/GetLocation';

import DatePicker, { PickerValue, utils } from '@rnwonder/solid-date-picker';
import { BiSolidRightArrowCircle } from 'solid-icons/bi';

export enum AppDisplayHousingUnit {
  ALL = 0,
  ALPHA = 1,
  BRAVO = 2,
  CHARLIE = 3,
  DELTA = 4,
  ECHO = 5
}

function App() {


  const [appDisplayHousingUnit, setAppDisplayHousingUnit] = createSignal(AppDisplayHousingUnit.ALL);
  const [appNewResidentModalOpen, setAppNewResidentModalOpen] = createSignal(false);
  const [appNewResidentModalRFID, setAppNewResidentModalRFID] = createSignal('');
  const [appEditResidentModalOpen, setAppEditResidentModalOpen] = createSignal(false);
  const [appEditResident, setAppEditResident] = createSignal<SResident | null>(null);

  // Okay, we need to grab a list of all the locations and then display them in a dropdown menu.
  // ******************************************************************************************
  const [facilityLocations, setFacilityLocations] = createSignal<SLocation[]>([]);
  const [selectedLocation, setSelectedLocation] = createSignal<SLocation | null>(null);

  createEffect(() => {
    console.log("Attempting to set default location.")
    if (facilityLocations().length > 0) {
      console.log("Setting default location.");
      setDefaultSelectedLocation();
      return;
    }
    console.log("Unable to set default location.")
  }, [facilityLocations]);

  const setDefaultSelectedLocation = () => {
    if (window.facilityLocationId) {
      let loc = facilityLocations().find((loc) => loc.id === window.facilityLocationId);
      if (loc) {
        setSelectedLocation(loc);
      }
    }
  };

  const resolveLocationNameForMenu = (location: SLocation): string => {
    if (location.name === "SIGNED_OUT") {
      return "FACILITY WIDE";
    }

    // Otherwise, we take the string, split it by underscores, and then join it with spaces.
    let nameParts = location.name.split("_");
    let name = nameParts.join(" ");
    return name.toUpperCase();
  };

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
  }


  const handleSelectLocation = (location: SLocation) => {
    console.log("SELECTED LOCATION: ", location);
    setSelectedLocation(location);
  };

  const loadFacilityLocations = async () => {
    let locs = await GetAllLocations();
    setFacilityLocations(locs);
  };

  // Lets handle range selection. This allows us to select a timeframe to display timestamps for.
  // ******************************************************************************************


  const defaultDateRangeFrom = {
    year: new Date().getFullYear(),
    month: new Date().getMonth(),
    day: new Date().getDate(),
  };

  const defaultDateRangeTo = {
    year: new Date(+ new Date() + 86400000).getFullYear(),
    month: new Date(+ new Date() + 86400000).getMonth(),
    day: new Date(+ new Date() + 86400000).getDate(),
  };

  const [dateRange, setDateRange] = createSignal<PickerValue>({ value: { startDateObject: defaultDateRangeFrom, endDateObject: defaultDateRangeTo }, label: 'From -> To' });


  const displayNewResidentModal = (rfid: string) => {
    console.log("DISPLAY NEW RESIDENT MODAL RN");
    setAppNewResidentModalOpen(true);
    setAppNewResidentModalRFID(rfid);
  };

  onMount(() => {
    initScanner({ displayNewResidentModal, refetchData });

    // Load the facility locations
    loadFacilityLocations();
  });

  onCleanup(() => {
    cleanupScanner();
  });

  const residentActions: STableAction[] = [
    {
      actionLabel: 'Edit',
      actionFunction: (props: { rfid: string }) => {
        console.log("Execute Edit Action/Pull Up Edit Modal");
        handleEditResident(props.rfid);
      }
    }

  ];

  const handleCloseNewResidentModal = () => {
    setAppNewResidentModalOpen(false);
    setAppNewResidentModalRFID('');
  };

  const handleCloseEditResidentModal = () => {
    setAppEditResidentModalOpen(false);
    setAppEditResident(null);
  };

  const handleCreateNewResident = (newResident: SResident) => {
    createResident(newResident);
  };

  const refetchData = () => {
    refetchInResidents();
    refetchOutResidents();
  };

  const [outResidentsData, { refetch: refetchOutResidents }] = createResource(getResidentsOut, { initialValue: { data: [], priorityData: [] } });
  const [inResidentsData, { refetch: refetchInResidents }] = createResource(getResidentsIn, { initialValue: { data: [] } });

  const handleEditResident = async (rfid: string) => {
    toast('Loading Resident Data...', { duration: 1000 });

    let residentReference: SResident | null = await GetResidentByRFID(rfid);

    if (residentReference === null) {
      console.error("Unable to find resident with RFID: " + rfid);
      return;
    }

    setAppEditResident(residentReference);
    console.log("EDIT RESIDENT FOUND RFID: " + rfid, residentReference);
    setAppEditResidentModalOpen(true);

  };

  const handleEditResidentDone = (resident: SResident) => {

    // Make API call to update resident
    let result = updateResident(resident.rfid, resident);
    console.log("RESULT: ", result);



    // Refetch the data
    refetchOutResidents();
    refetchInResidents();
  };

  return (
    <div class={"flex flex-col w-screen h-screen"}>

      <Toaster position='bottom-right' gutter={8} containerClassName='' toastOptions={{
        className: '',
        duration: 5000,
        style: {
          background: '#363636',
          color: '#fff',
        },
      }} />


      {appNewResidentModalRFID() ? (
        <ResidentIDModal close={handleCloseNewResidentModal} open={appNewResidentModalOpen} create={handleCreateNewResident} rfid={appNewResidentModalRFID()} />
      ) : false}

      {appEditResidentModalOpen() ? (
        <ResidentEditModal close={handleCloseEditResidentModal} open={appEditResidentModalOpen} currentResident={appEditResident} editResident={handleEditResidentDone} />
      ) : false}

      <div class={"flex flex-row w-full h-12 sticky top-0 z-10 px-10 py-2"}>
        <div class={"flex flex-row py-4 h-12 items-center justify-start gap-8"}>
          <div class={"flex items-center justify-center h-full w-[80px]"}>
            <label for={"facilityLocationSelect"} class={"text-lg font-bold"} >Facility Location</label>
          </div>
          <div class={"flex items-center justify-center"}>
            <select name="facilityLocationSelect" value={selectedLocation()?.id} class={"flex justify-start items-center rounded border-gray-300 border-2  px-2 py-2 "}>
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
                    <option value={location.id} class={`text-lg ${location.id === 0 ? 'font-bold' : false}`} onClick={() => handleSelectLocation(location)}>{locationName} ({count})</option>
                  );
                }}
              </For>
            </select>
          </div>
          <div class={"flex justify-center items-center text-center"}>
            <label class={"text-md font-bold"}>Search Range <br /> [{utils().convertDateObjectToDate(dateRange().value.startDateObject!).toLocaleDateString()} -- {utils().convertDateObjectToDate(dateRange().value.endDateObject!).toLocaleDateString()}]</label>
          </div>
          <div class={"flex flex-row justify-center items-center h-12"}>
            <DatePicker type="range" value={dateRange} setValue={setDateRange} maxDate={utils().getToday()} minDate={utils().convertDateToDateObject(new Date(new Date().getDate() + 1))} inputClass={"text-sm h-10 border-[3px] border-neutral-400 rounded"} inputWrapperWidth={"8rem"} />
          </div>
        </div>
      </div>
      <div class={"flex flex-row justify-end gap-x-2 px-2 py-3"}>



        <div class={"flex w-[44rem] justify-center"}>
          {outResidentsData.loading ? (<div class={"flex justify-center items-center"}><img src={loadingAnim} class={"w-10 h-10"} /></div>) : (
            <STable type='TimestampResident' data={outResidentsData()} />
          )}
        </div>

        <div class={"flex w-[42rem] justify-center"}>
          {inResidentsData.loading ? (<div class={"flex justify-center items-center"}><img src={loadingAnim} class={"w-10 h-10"} /></div>) : (
            <STable type='Resident' data={inResidentsData()} actions={residentActions} />
          )}
        </div>

      </div>
    </div>
  );
}

export default App
