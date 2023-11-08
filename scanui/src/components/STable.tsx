/*
 *
 * Copyright (c) {11/7/23, 10:03 PM} Lorenzo A. Banks and Preston Thorpe. All rights reserved.
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
import {Component, createEffect, createMemo, createSignal, For, JSXElement, onMount} from "solid-js";

import '../styles/STable.css';

export interface STableProps {
    type: "Resident" | "Location" | "Timestamp";
    data: SResident[]|STimestamp[]|SLocation[];
    actions?: STableAction[];
}

export interface STableAction {
    actionLabel:string;
    actionFunction:() => void;
}

export interface STableColumn {
    label:string;
    sortable:boolean;
    icon?:JSXElement;
}

export interface STableRowItem {
    selected:boolean;
    item:SResident|SLocation|STimestamp;
}


export const STable: Component<STableProps> = (props:STableProps) => {

    const [tableData, setTableData] = createSignal<STableRowItem[]|null>(null);
    const [tableColumns, setTableColumns] = createSignal<STableColumn[]|null>(null);
    const [tableAllSelect, setTableAllSelect] = createSignal({allSelected:false});
    const [tableSelectedItems, setTableSelectedItems] = createSignal<STableRowItem[]>([]);

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

        let tbldat:STableRowItem[] = [];
        // Load Table Data From Props
        props.data.map((item) => {
            tbldat.push({selected:false, item:item} as STableRowItem);
        });
        setTableData(tbldat);

    });

    const handleSelectRow = (e:Event, item:STableRowItem|null) => {
        //console.log("Handle Select Row ", item);
        e.preventDefault();
        if (tableData() !== null) {
            let tableCpy = [...tableData() as STableRowItem[]];

            // If item === null, we actually want to toggle all rows
            if (item === null) {
                console.log("Selected AllSelect Toggle, ", !tableAllSelect().allSelected);
                // Basically, toggle this and see if we can catch the logic in the Effect;
                setTableAllSelect({allSelected:!tableAllSelect().allSelected});
                return;
            } else {

                let rowIndex = tableCpy.findIndex(i => i === item);
                // Toggle The Select
                if (rowIndex !== -1) {
                    tableCpy[rowIndex] = {...tableCpy[rowIndex], selected: !tableCpy[rowIndex].selected};
                }
            }
            console.log("New Table Copy:", tableCpy);
            setTableData(tableCpy);
        }

    }

    const handleAllSelectChange = () => {
        console.log("handleAllSelect Change Executed!");
        let dataCpy:STableRowItem[] = [];
        // If We're not active, then activating means selecting all items
        if (!tableAllSelect().allSelected) {
            dataCpy = tableData()?.map((item) => {
                return {item:item.item, selected:true};
            })!;
        } else {
            dataCpy = tableData()?.map((item) => {
                return {item:item.item, selected:false};
            })!;
        }
        setTableAllSelect({allSelected:!tableAllSelect().allSelected});
        setTableData(dataCpy);
    }


    // Effect To Handle All Select Stuff
    createEffect(() => {
        console.log("[+] Effect: Account For Selected Items");
        createMemo(() => {
            let selected: STableRowItem[] = [];
            tableData()?.map((item) => {
                if (item.selected) {
                    selected.push(item);
                }
            });
            if (selected.length > 0) {
                console.log("Updated Selected Items!: ", selected.length);
                setTableSelectedItems(selected);
            } else {
                console.log("Selected Items Set To 0");
                setTableSelectedItems([]);
            }
        });
    })



    return(
        <div class={"stable-wrapper"}>
          <table class={"stable"}>
              {/* **** TABLE HEADER *****************************************************************/}
              <thead>
                <tr>
                    {/* **** HEADER CHECKBOX *****************************************************************/}
                    <th>
                        <div class={"cell-inner"}>
                            <label class={"stable-checkbox"}>
                                <input class={"stable-checkbox-input"}
                                       type={"checkbox"}
                                       checked={tableAllSelect().allSelected}
                                    onChange={() => handleAllSelectChange()}/>
                                    <span class={"stable-checkbox-checkmark"}></span>
                            </label>
                        </div>
                    </th>
                    {/* **** TABLE HEADER ITEMS *****************************************************************/}
                    {tableColumns()?.map((item) => (
                        <th><div class={"cell-inner"}>{item.label}</div></th>
                    ))}
                    {/* **** ACTIONS HEADER CELL *****************************************************************/}
                    {props.actions !== undefined ? (
                        <th>
                            <div class={"cell-inner"}>
                                <span>Actions</span>
                            </div>
                        </th>
                    ) : false}
                </tr>
              </thead>
              {/* **** TABLE BODY *****************************************************************/}
              <tbody>

              {/* **** RESIDENT TABLE LAYOUT *****************************************************************/}
                {props.type === "Resident" ? (
                    <For each={tableData()} fallback={<p>No Items</p>}>
                        {(item) => {
                            const residentItem = item.item as SResident;
                            return (
                                <tr>
                                    <td>
                                        <div class={"cell-inner"}>
                                            <label class={"stable-checkbox"}>
                                                    <input class={"stable-checkbox-input"}
                                                           type={"checkbox"}
                                                           checked={item.selected}
                                                           onChange={(e) => handleSelectRow(e, item)}/>
                                                    <span class={"stable-checkbox-checkmark"}></span>
                                            </label>
                                        </div>
                                    </td>
                                    <td><div class={"cell-inner"}>{residentItem.rfid}</div></td>
                                    <td><div class={"cell-inner"}>{residentItem.name}</div></td>
                                    <td><div class={"cell-inner"}>{residentItem.doc}</div></td>
                                    <td><div class={"cell-inner"}>{residentItem.pod}</div></td>
                                    <td><div class={"cell-inner"}>{residentItem.room}</div></td>
                                    <td>
                                        <div class={"cell-inner"}>
                                            <For each={props.actions} >
                                                {(action) => (
                                                    <button onClick={action.actionFunction}>
                                                        {action.actionLabel}
                                                    </button>
                                                )}
                                            </For>
                                        </div>
                                    </td>
                                </tr>
                            );
                        }}
                    </For>
                ):false}

              {/* **** LOCATION TABLE LAYOUT *****************************************************************/}
                {props.type === "Location" ? (
                    <For each={tableData()} fallback={<p>No Items</p>}>
                        {(item) => {
                            const locationItem = item.item as SLocation;
                            return (
                                <tr>
                                    <td>
                                        <div class={"cell-inner"}>
                                            <label class={"stable-checkbox"}>
                                                <input class={"stable-checkbox-input"}
                                                       type={"checkbox"}
                                                       checked={item.selected}
                                                       onChange={(e) => handleSelectRow(e, item)}/>
                                                <span class={"stable-checkbox-checkmark"}></span>
                                            </label>
                                        </div>
                                    </td>
                                    <td><div class={"cell-inner"}>{locationItem.id}</div></td>
                                    <td><div class={"cell-inner"}>{locationItem.name}</div></td>
                                    <td>
                                        <div class={"cell-inner"}>
                                            <For each={props.actions}>
                                                {(action) => (
                                                    <button onClick={action.actionFunction}>
                                                        {action.actionLabel}
                                                    </button>
                                                )}
                                            </For>
                                        </div>
                                    </td>
                                </tr>
                            );
                        }}
                    </For>
                ):false}


              {/* **** TIMESTAMP TABLE LAYOUT *****************************************************************/}
                {props.type === "Timestamp" ? (
                    <For each={tableData()} fallback={<p>No Items</p>}>
                        {(item) => {
                            const timestampItem = item.item as STimestamp;
                            return (
                                <tr>
                                    <td>
                                        <div class={"cell-inner"}>
                                            <label class={"stable-checkbox"}>
                                                <input class={"stable-checkbox-input"}
                                                       type={"checkbox"}
                                                       checked={item.selected}
                                                       onChange={(e) => handleSelectRow(e, item)}/>
                                                <span class={"stable-checkbox-checkmark"}></span>
                                            </label>
                                        </div>
                                    </td>
                                    <td><div class={"cell-inner"}>{timestampItem.rfid}</div></td>
                                    <td><div class={"cell-inner"}>{timestampItem.destinationId}</div></td>
                                    <td><div class={"cell-inner"}>{timestampItem.timestamp}</div></td>
                                    <td>
                                        <div class={"cell-inner"}>
                                            <For each={props.actions} >
                                                {(action) => (
                                                    <button onClick={action.actionFunction}>
                                                        {action.actionLabel}
                                                    </button>
                                                )}
                                            </For>
                                        </div>
                                    </td>
                                </tr>
                            );
                        }}
                    </For>
                ):false}

              </tbody>
          </table>

            <div class={"flex flex-row"}>
                {tableSelectedItems().length > 0 ? (
                    <span>{tableSelectedItems().length.toString()} Selected Rows</span>
                ):false}
            </div>
        </div>
    );

}
