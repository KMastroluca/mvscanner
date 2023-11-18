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

import './App.css'
import {STable, STableAction} from "./components/STable.tsx";
import { SResident, STimestampResident} from "./types/Models.ts";
function App() {


  let e:STimestampResident[] = [
    {
      rfid: "83726384958372839",
      name: "Banks, Lorenzo",
      doc: "122750",
      timestamp: "2021-08-23T15:09:00Z",
      housingPod: "A",
      room: "11-T",
      timestampLeft:"2021-08-23T15:09:00Z",
      destinationLabel: "Music Room"
    },
    {
      rfid: "83726384953678293",
      name: "Martin, Sherry",
      doc: "120030",
      timestamp: "2021-08-23T15:09:00Z",
      housingPod: "A",
      room: "19-T",
      timestampLeft:"2021-08-23T15:09:00Z",
      destinationLabel: "Gym"
    },
    {
      rfid: "83726384958372839",
      name: "Decker, John",
      doc: "449320",
      timestamp: "2021-08-23T15:09:00Z",
      housingPod: "A",
      room: "32-B",
      timestampLeft:"2021-08-23T15:09:00Z",
      destinationLabel: "Education"
    }
  ]

  return (
      <>
        <STable type={"TimestampResident"} data={e} />
      </>
  )
}

export default App
