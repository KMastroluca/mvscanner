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

import { createEffect, createResource, createSignal } from 'solid-js';
import {makeAbortable, makeCache, createAggregated} from '@solid-primitives/resource';
import './App.css'
import { getResidentsOut, getResidentsIn } from './api/ResidentInOut';
import { STable } from './components/STable';

function App() {

  const [outResidentsData] = createResource(getResidentsOut, { initialValue: [] });
  const [inResidentsData] = createResource(getResidentsIn, {initialValue: []});

  return (
    <div class={"flex flex-row justify-end gap-x-2 px-2 py-3"}>
    <div class={"flex w-[49em]"}>
      {outResidentsData.loading ? (<div>Loading...</div>): (
        <STable type='TimestampResident' data={outResidentsData()} />
      )}
    </div>

    <div class={"flex w-[24em]"}>
      {inResidentsData.loading ? (<div>Loading...</div>): (
        <STable type='Resident' data={inResidentsData()} />
      )}
    </div>

    
    </div>
  );
;

}

export default App
