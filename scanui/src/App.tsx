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

import { createResource, onCleanup, onMount } from 'solid-js';
import './App.css'
import { getResidentsOut, getResidentsIn } from './api/ResidentInOut';
import { STable, STableAction} from './components/STable';
import { createResident } from './api/CreateResident';
import { createTimestamp } from './api/CreateTimestamp';
import { initScanner, cleanupScanner } from './components/Scanner';
import { ResidentIDModal } from './components/ResidentIDModal';
function App() {


  onMount(() => {
    initScanner();
  });

  onCleanup(() => {
    
  });


  createTimestamp({
    rfid:"555555231555555",
    location: 12,
  });


  const residentActions:STableAction[] = [
    {
      actionLabel: 'Edit',
      actionFunction: () => {
        console.log("Edit");
      }
    }

  ];


  const [outResidentsData] = createResource(getResidentsOut,  {initialValue:    {data:[], priorityData:[]} });
  const [inResidentsData] = createResource(getResidentsIn,    {initialValue:    {data:[]} });

  return (
    <div class={"flex flex-col w-screen h-screen"}>
      <ResidentIDModal open={true}/>
      <div class={"flex flex-row w-full h-10 sticky top-0 z-10 bg-slate-400"}>
          
      </div>
      <div class={"flex flex-row justify-end gap-x-2 px-2 py-3"}>

        <div class={"flex flex-col w-[20em] bg-slate-300 border-2"}>

        </div>


        <div class={"flex w-[49em]"}>
          {outResidentsData.loading ? (<></>): (
            <STable type='TimestampResident' data={outResidentsData()} />
          )}
        </div>

        <div class={"flex w-[30em]"}>
          {inResidentsData.loading ? (<></>): (
            <STable type='Resident' data={inResidentsData()} actions={residentActions} />
          )}
        </div>

      </div>
    </div>
  );
;

}

export default App
