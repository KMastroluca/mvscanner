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

import { createResource, createSignal, onCleanup, onMount } from 'solid-js';
import './App.css'
import { getResidentsOut, getResidentsIn } from './api/ResidentInOut';
import { STable, STableAction } from './components/STable';
import { createResident } from './api/CreateResident';
import { createTimestamp } from './api/CreateTimestamp';
import { initScanner, cleanupScanner } from './components/Scanner';
import { ResidentIDModal } from './components/ResidentIDModal';

import { TbCaretRight, TbCaretDown } from 'solid-icons/tb';
import { SResident } from './types/Models';
import { ResidentEditModal } from './components/EditResidentModal';
import { GetResidentByRFID } from './api/GetResident';
import toast, { Toaster } from 'solid-toast';
import { updateResident } from './api/UpdateResident';

import loadingAnim from './assets/loading.gif';

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

  const displayNewResidentModal = (rfid: string) => {
    console.log("DISPLAY NEW RESIDENT MODAL RN");
    setAppNewResidentModalOpen(true);
    setAppNewResidentModalRFID(rfid);
  };

  onMount(() => {
    initScanner({ displayNewResidentModal, refetchData });
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

      <div class={"flex flex-row w-full h-10 sticky top-0 z-10 bg-slate-400"}>

      </div>
      <div class={"flex flex-row justify-end gap-x-2 px-2 py-3"}>

        <div class={"flex flex-col w-[5rem] bg-slate-300 border-2"}>
          <nav class={"flex w-full text-xl uppercase"}>
            <ul class={"w-full"}>
              <li><a class={`flex items-center hover:bg-neutral-200 border-b-[1px] justify-between w-full px-3 py-4 ${appDisplayHousingUnit() === AppDisplayHousingUnit.ALL ? 'bg-neutral-200' : 'bg-white'}`} href="#">AU {appDisplayHousingUnit() === AppDisplayHousingUnit.ALL ? <TbCaretRight /> : <TbCaretDown />}</a></li>
              <li><a class={`flex items-center hover:bg-neutral-200 border-b-[1px] justify-between w-full px-3 py-4 ${appDisplayHousingUnit() === AppDisplayHousingUnit.ALPHA ? 'bg-neutral-200' : 'bg-white'}`} href="#">A {appDisplayHousingUnit() === AppDisplayHousingUnit.ALPHA ? <TbCaretRight /> : <TbCaretDown />}</a></li>
              <li><a class={`flex items-center hover:bg-neutral-200 border-b-[1px] justify-between w-full px-3 py-4 ${appDisplayHousingUnit() === AppDisplayHousingUnit.BRAVO ? 'bg-neutral-200' : 'bg-white'}`} href="#">B {appDisplayHousingUnit() === AppDisplayHousingUnit.BRAVO ? <TbCaretRight /> : <TbCaretDown />}</a></li>
              <li><a class={`flex items-center hover:bg-neutral-200 border-b-[1px] justify-between w-full px-3 py-4 ${appDisplayHousingUnit() === AppDisplayHousingUnit.CHARLIE ? 'bg-neutral-200' : 'bg-white'}`} href="#">C {appDisplayHousingUnit() === AppDisplayHousingUnit.CHARLIE ? <TbCaretRight /> : <TbCaretDown />}</a></li>
              <li><a class={`flex items-center hover:bg-neutral-200 border-b-[1px] justify-between w-full px-3 py-4 ${appDisplayHousingUnit() === AppDisplayHousingUnit.DELTA ? 'bg-neutral-200' : 'bg-white'}`} href="#">D {appDisplayHousingUnit() === AppDisplayHousingUnit.DELTA ? <TbCaretRight /> : <TbCaretDown />}</a></li>
              <li><a class={`flex items-center hover:bg-neutral-200 border-b-[1px] justify-between w-full px-3 py-4 ${appDisplayHousingUnit() === AppDisplayHousingUnit.ECHO ? 'bg-neutral-200' : 'bg-white'}`} href="#">E {appDisplayHousingUnit() === AppDisplayHousingUnit.ECHO ? <TbCaretRight /> : <TbCaretDown />}</a></li>
            </ul>
          </nav>
        </div>


        <div class={"flex w-[44rem] justify-center items-center"}>
          {outResidentsData.loading ? (<img src={loadingAnim} class={"w-10 h-10"} />) : (
            <STable type='TimestampResident' data={outResidentsData()} />
          )}
        </div>

        <div class={"flex w-[42rem] justify-center items-center"}>
          {inResidentsData.loading ? (<img src={loadingAnim} class={"w-10 h-10"} />) : (
            <STable type='Resident' data={inResidentsData()} actions={residentActions} />
          )}
        </div>

      </div>
    </div>
  );
}

export default App
