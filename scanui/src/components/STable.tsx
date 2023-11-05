/*
 *
 * Copyright (c) {11/5/23, 4:30 PM} Lorenzo A. Banks and Preston Thorpe. All rights reserved.
 * {STable.tsx}
 * {STable.tsx}
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



import {SLocation, SResident, STimestamp} from "../types/Models.ts";
import {createSignal, JSXElement, onMount} from "solid-js";

export interface STableProps {
    type: "Resident" | "Location" | "Timestamp";
    data: SResident[] | SLocation[] | STimestamp[];
}

export interface STableColumn {
    label:string;
    sortable:boolean;
    icon?:JSXElement;
}

export const STable = (props:STableProps) => {

    const [tableData, setTableData] = createSignal<SResident[]|SLocation[]|STimestamp[]|null>(null);
    const [tableColumns, setTableColumns] = createSignal<STableColumn[]|null>(null);

    onMount(() => {
        // Load Table Columns
        if (props.type === "Resident") {
            let columns:STableColumn[] = [
                {
                    label:"RFID",
                    sortable:false
                },
                {
                    label:"Name",
                    sortable:false,
                },
                {
                  label:"DOC#",
                  sortable: false,
                },
                {
                    label:"POD",
                    sortable:false,
                },
                {
                    label:"Room",
                    sortable: false,
                }
            ];
            setTableColumns(columns);
        } else if (props.type === "Location") {
            let columns:STableColumn[] = [
                {
                    label:"ID",
                    sortable:false,
                },
                {
                    label:"Name",
                    sortable:false,
                }
            ];
            setTableColumns(columns);
        } else if (props.type === "Timestamp") {
            let columns:STableColumn[] = [
                {
                    label:"RFID",
                    sortable:false,
                },
                {
                    label:"Destination",
                    sortable:false
                },
                {
                    label:"Timestamp",
                    sortable:false
                }
            ];
            setTableColumns(columns);
        }

        // Load Table Data From Props
        setTableData(props.data);
    });


    const renderResidentRow = (resident:SResident) => {
        return (
            <tr class={"stable-body-row"}>
                <td class={"stable-body-cell"}>{resident.rfid}</td>
                <td class={"stable-body-cell"}>{resident.name}</td>
                <td class={"stable-body-cell"}>{resident.doc}</td>
                <td class={"stable-body-cell"}>{resident.pod}</td>
                <td class={"stable-body-cell"}>{resident.room}</td>
            </tr>
        );
    };

    const renderLocationRow = (location:SLocation) => {
        return (
            <tr class={"stable-body-row"}>
                <td class={"stable-body-cell"}>{location.id}</td>
                <td class={"stable-body-cell"}>{location.name}</td>
            </tr>
        );
    };

    const renderTimestampRow = (timestamp:STimestamp) => {
        return (
            <tr class={"stable-body-row"}>
                <td class={"stable-body-cell"}>{timestamp.rfid}</td>
                <td class={"stable-body-cell"}>{timestamp.destinationId}</td>
                <td class={"stable-body-cell"}>{timestamp.timestamp}</td>
            </tr>
        );
    };

    const RenderRow = (prop:{item:STimestamp|SLocation|SResident}) => {
        if (props.type === "Resident") {
            return renderResidentRow(prop.item as SResident);
        } else if (props.type === "Location") {
            return renderLocationRow(prop.item as SLocation);
        } else if (props.type === "Timestamp") {
            return renderTimestampRow(prop.item as STimestamp);
        }
    }


    return(
      <table class={"stable"}>
          <thead class={"stable-header"}>
            <tr class={"stable-header-row"}>
                {tableColumns()?.map((item) => (
                    <th class={"stable-header-cell"}>{item.label}</th>
                ))}
            </tr>
          </thead>
          <tbody class={"stable-body"}>
          {tableData() !== null ? (
              <>
                {tableData()!.map((item) => (
                    <RenderRow item={item} />
                ))}
              </>
          ) : (
              <p>No Data!</p>
          )}
          </tbody>
      </table>
    );

}
