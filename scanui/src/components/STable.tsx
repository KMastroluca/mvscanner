/*
 *
 * Copyright (c) {11/8/23, 3:22 PM} Lorenzo A. Banks and Preston Thorpe. All rights reserved.
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


import {SLocation, SResident, STimestamp, STimestampResident} from "../types/Models.ts";
import {Component, createEffect, createMemo, createSignal, For, JSXElement, onMount} from "solid-js";

import '../styles/STable.css';
import {TbArrowsSort, TbSettings2, TbEdit} from "solid-icons/tb";

export interface STableProps {
    type: "Resident" | "Location" | "Timestamp" | "TimestampResident";
    data: STableData;
    actions?: STableAction[];
}

export interface STableData {
   data: SResident[]|STimestamp[]|SLocation[]|STimestampResident[];
   priorityData?: SResident[]|STimestamp[]|SLocation[]|STimestampResident[];
}



export interface STableAction {
    actionLabel:string;
    actionFunction:() => void;
}

export interface STableColumn {
    label:string;
    key?:string;
    sortable:boolean;
    icon?:JSXElement;
    sortType?:SortDataType;
}

export interface STableRowItem {
    selected:boolean;
    item:SResident|SLocation|STimestamp|STimestampResident;
}

export type SortType = StringSort|NumericalSort|DateSort;
export type SortDataString = {
  type:"String";
};
export type SortDataNumerical = {
    type:"Numerical";
};
export type SortDataDate = {
  type:"Date";
};
export type SortDataType = SortDataString|SortDataNumerical|SortDataDate;
export enum StringSort {
    Alphabetical = 0,
    AlphaReverse = 1,
}

export enum NumericalSort {
    Ascending = 2,
    Decending = 3,
}

export enum DateSort {
    LatestFirst = 4,
    OldestFirst = 5,
}

export const STable: Component<STableProps> = (props:STableProps) => {

    const [tableData, setTableData] = createSignal<STableRowItem[]|null>(null);
    const [priorityTableData, setPriorityTableData] = createSignal<STableRowItem[]|null>(null);
   

    const [tableColumns, setTableColumns] = createSignal<STableColumn[]|null>(null);
    const [tableAllSelect, setTableAllSelect] = createSignal({allSelected:false});
    const [tableSelectedItems, setTableSelectedItems] = createSignal<STableRowItem[]>([]);

    // Sorting State

    const [currentSort, setCurrentSort] = createSignal<{type:SortType, columnIndex:number}|null>(null);
    const [previousSort, setPreviousSort] = createSignal<{type:SortType, columnIndex:number}|null>(null);

    onMount(() => {
        // Load Table Columns
        if (props.type === "Resident") {
            let columns:STableColumn[] = [
                {
                    label:"Resident Name:",
                    sortable:true,
                    sortType:{type:"String"},
                    key:"name",
                },
                {
                  label:"DOC #:",
                    key:"doc",
                  sortable: false,
                },
                {
                    label:"POD - Housing:",
                    key:"pod",
                    sortable:false,
                }
            ];
            setTableColumns(columns);
        } else if (props.type === "Location") {
            let columns:STableColumn[] = [
                {
                    label:"ID",
                    sortable:true,
                    sortType: {type:"Numerical"},
                },
                {
                    label:"Name",
                    sortable:true,
                    sortType:{type:"String"},
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
                    sortable:false,
                },
                {
                    label:"Timestamp",
                    sortable:true,
                    sortType: {type:"Date"},
                }
            ];
            setTableColumns(columns);
        } else if (props.type === "TimestampResident") {
            let columns:STableColumn[] = [
                {
                    label:"Resident Name:",
                    sortable:true,
                    sortType: {type:"String"},
                    key:"name",
                },
                {
                    label:"DOC #:",
                    sortable:false,
                },
                {
                    label:"POD - Housing:",
                    sortable:true,
                    sortType: {type:"String"},
                    key:"room",
                },
                {
                    label:"Departed At:",
                    sortable:true,
                    sortType: {type:"Date"},
                    key:"timestampLeft",
                },
                {
                    label:"Destination:",
                    sortable:true,
                    sortType: {type:"String"},
                    key:"destinationLabel",
                }
            ];
            setTableColumns(columns);
        }
   
      
         let stdat = props.data as STableData;


        let tbldat:STableRowItem[] = [];
        // Load Table Regular Data From Props
        stdat.data.map((item) => {
            tbldat.push({selected:false, item:item} as STableRowItem);
        });
        setTableData(tbldat);

        // Check For Priority Data
        if (stdat.priorityData) {
            let priorityTblDat:STableRowItem[] = [];
            stdat.priorityData.map((item) => {
               priorityTblDat.push({selected:false, item:item} as STableRowItem);
            });
            setPriorityTableData(priorityTblDat);
        }

    });

    /**
     * @function handleSelectRow - Handles Selecting A Row
     * @param e 
     * @param item 
     * @returns 
     */
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

    };

    /**
     * @function handleAllSelectChange - Handles Selecting All Rows
     */
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
    };


    /**
     * This effect is used to set the selected items in the table.
     */
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
    });


    // Sorting ********************************************


    /**
     * Toggles The Current Sorting Algorithim No Matter What Data Type
     */
    const toggleCurrentSort = () => {
        if (currentSort() !== null) {
            if (tableColumns()![currentSort()!.columnIndex].sortType!.type === "String") {
                if (currentSort()!.type === StringSort.Alphabetical) {
                    setCurrentSort({columnIndex: currentSort()!.columnIndex, type: StringSort.AlphaReverse});
                } else if (currentSort()!.type === StringSort.AlphaReverse) {
                    setCurrentSort({columnIndex: currentSort()!.columnIndex, type: StringSort.Alphabetical});
                }
            } else if (tableColumns()![currentSort()!.columnIndex].sortType!.type === "Numerical") {
                if (currentSort()!.type === NumericalSort.Ascending) {
                    setCurrentSort({columnIndex:currentSort()!.columnIndex, type:NumericalSort.Decending});
                } else if (currentSort()!.type === NumericalSort.Decending) {
                    setCurrentSort({columnIndex:currentSort()!.columnIndex, type:NumericalSort.Ascending});
                }
            } else if (tableColumns()![currentSort()!.columnIndex].sortType!.type === "Date") {
                if (currentSort()!.type == DateSort.LatestFirst) {
                    setCurrentSort({columnIndex:currentSort()!.columnIndex, type:DateSort.OldestFirst});
                } else if (currentSort()!.type == DateSort.OldestFirst) {
                    setCurrentSort({columnIndex:currentSort()!.columnIndex, type:DateSort.LatestFirst});
                }
            }
        } else {
            console.error("[-] This should never happen, we tried to toggle sort, but there was no sort applied!");
        }
    }

    /**
     * @function setCurrentSortColumn
     * @param columnIndex 
     * @returns 
     * This sets the current sort column.
     */
    const setCurrentSortColumn = (columnIndex:number) => {
        console.log("Run setCurrentSortColumn");
        // If we already have a sort set, with this columnIndex and we're running this function again,
        // lets assume we want to toggle to the other sort option
        if (currentSort() !== null) {
            // Yes, we were already sorting before.
            console.log("We were already sorting before!")
            if (currentSort()!.columnIndex === columnIndex) {
                // Yes we want to toggle.
                console.log("Toggling current sort");
                toggleCurrentSort();
                return;
            }
            // Otherwise, we want to switch columnIndexes and we just run the function like normal.
        }

        if (tableColumns() === undefined) {
            console.error("[-] Tried to call setCurrentSortColumn but tableColumns() is undefined! - ", tableColumns());
            return;
        }

        if (columnIndex > tableColumns()!.length) {
            console.error("[-] Tried to set a sort column, for a column that doesnt exist, columnIndex was > tableColumns().length");
            return;
        }

        let sortColumn = tableColumns()![columnIndex];
        if (sortColumn.sortType) {
            console.log("Found sortType on sortColumn:", sortColumn, sortColumn.sortType);
            if (sortColumn.sortType.type === "String") {
                console.log("Sort Type Was String");
                setCurrentSort({type:StringSort.Alphabetical, columnIndex:columnIndex});
            } else if (sortColumn.sortType.type === "Numerical") {
                console.log("Sort Type Was Numerical");
                setCurrentSort({type:NumericalSort.Ascending, columnIndex:columnIndex});
            } else if (sortColumn.sortType.type === "Date") {
                console.log("Sort Type Was Date");
                setCurrentSort({type:DateSort.LatestFirst, columnIndex:columnIndex});
            }
        }

    };
    /**
     * @function reSortData - Sorts The Data Based On The Current Sort
     * @param data {type:SortType, columnIndex:number}
     */
    const reSortData = (data:{type:SortType, columnIndex:number}) => {
        console.log("[+] Re-Sort Data Was Executed!", data);
        let columnToSort = tableColumns()![data.columnIndex];
        console.log(columnToSort);
        if (columnToSort.key) {
            let key = columnToSort.key!;
            console.log("[+] Key To Sort By:", key);
            // Key Is Set To Compare To Data Objects
            switch(data.type) {
                case StringSort.Alphabetical: {
                    console.log("[+] Alphabetical Sorting");
                    let tableCopy = [...tableData()!];
                    tableCopy.sort((a, b) => {
                        if (a.item.hasOwnProperty(key) && b.item.hasOwnProperty(key)) {
                            if (typeof a.item[key] === 'string' && typeof b.item[key] === 'string') {
                                return (a.item[key] as string).localeCompare((b.item[key] as string));
                            } else {
                                return 0;
                            }
                        } else {
                            return 0;
                        }
                    });
                    console.log("[+] This is the copy of the new data to sort.", tableCopy);

                    // Setting This Data Here Works, But Causes A Rerender Loop.
                    // Set The Previous Sort, So We Know Not To Re-Render
                    
                    setTableData(tableCopy);
                    console.log("[+] Completed Alphabetical Sort");
                    break;
                }
                case StringSort.AlphaReverse: {
                    console.log("[+] Reverse Alphabetical Sorting");
                    let tableCopy = [...tableData()!];
                    tableCopy.sort((a, b) => {
                       if (a.item.hasOwnProperty(key) && b.item.hasOwnProperty(key)) {
                           if (typeof a.item[key] === 'string' && typeof b.item[key] === 'string') {
                               return -(a.item[key] as string).localeCompare((b.item[key] as string));
                           } else {
                               return 0;
                           }
                       } else {
                           return 0;
                       }
                    });
                    console.log("[+] Result Of Reverse Alphabetical Sort: ",tableCopy)
                    setTableData(tableCopy);
                    console.log("[+] Completed Reverse Alphabetical Sort.");
                    break;
                }
                case DateSort.LatestFirst: {
                    console.log("[+] Executing Date Sort Latest First");

                    // For this, we need to convert the date strings to Date objects
                    let tableCopy = [...tableData()!];

                    tableCopy.sort((a, b) => {
                        let aDate = new Date(a.item[key] as string);
                        let bDate = new Date(b.item[key] as string);
                        return aDate.getTime() - bDate.getTime();
                    });

                    console.log("[+] Result Of Date Sort Latest First: ", tableCopy);
                    setTableData(tableCopy);
                    console.log("[+] Completed Date Sort Latest First");
                    break;
                }
                default:
                    console.error("[-] Sorting Algorithim Not Implemented!");
            }
        }
    };

    /**
     * This effect is used to call reSortData when the sort state has changed.
     */
    createEffect(() => {
        if (currentSort() !== previousSort()) {
            console.log("[+] Effect: Sort Has Changed, Setting Previous Sort, Setting New Current Sort");
            reSortData(currentSort()!);
            // Set Previous Sort To Current Sort
            setPreviousSort(currentSort());
        }
    }, [currentSort()?.type, currentSort()?.columnIndex]);

    /**
     * This effect just prints a log when the table data changes.
     * But might be used in the future for something.
     */
    createEffect(() => {
        console.log("Table Data Changed:", tableData());

    });





    return(
        <div class={"stable-wrapper"}>
          <table class={"stable"}>
              {/* **** TABLE HEADER *****************************************************************/}
              <thead>
                <tr>
                    {/* **** HEADER CHECKBOX *****************************************************************/}
                    <th style={`w-[${tableColumns()}]`}>
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
                    {tableColumns()?.map((item, index) => (
                        <th>
                            <div class={"cell-inner"}>
                                {item.label}
                                {item.sortable ? (
                                    <div class={"flex w-full justify-end"}>
                                        <button onClick={() => setCurrentSortColumn(index)}>
                                            <TbArrowsSort size={16} />
                                        </button>
                                    </div>
                                ): false}
                            </div>
                        </th>
                    ))}
                    {/* **** ACTIONS HEADER CELL *****************************************************************/}
                    {props.actions !== undefined || props.actions ? (
                        <th>
                            <div class={"cell-inner"}>
                                <span class="flex w-full justify-center"><TbSettings2 size={24} /></span>
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
                                    <td><div class={"cell-inner"}>{residentItem.name}</div></td>
                                    <td><div class={"cell-inner"}>{residentItem.doc}</div></td>
                                    <td><div class={"cell-inner"}>{residentItem.room}</div></td>
                                    {props.actions ? (
                                    <td>
                                        <div class={"cell-inner"}>
                                            <For each={props.actions} >
                                                {(action) => {

                                                    if (action.actionLabel === "Edit") { 
                                                        return (
                                                            <button onClick={action.actionFunction}>
                                                                <TbEdit size={24} />
                                                            </button>
                                                        );
                                                    }

                                                    return (
                                                    <button onClick={action.actionFunction}>
                                                        {action.actionLabel}
                                                    </button>
                                                    );
                                                }}
                                            </For>
                                        </div>
                                    </td>
                                    ) : false}
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

                {/* **** TIMESTAMP-RESIDENT TABLE LAYOUT *****************************************************************/}
                {props.type === "TimestampResident" ? (
                  <>
                    <For each={priorityTableData()} fallback={<></>} >
                     {(item) => {
                        let timestampItem = item.item as STimestampResident;

                        return (
                           <tr class={"text-white"}>
                              <td class={"bg-red-500"}>
                                 <div class={"cell-inner"}>
                                    <label class={"stable-checkbox"}>
                                       <input class={"stable-checkbox-input"}
                                          type={"checkbox"}
                                          checked={item.selected}
                                          onChange={(e) => handleSelectRow(e, item)}
                                       />
                                       <span class={"stable-checkbox-checkmark"}></span>
                                    </label>   
                                 </div>
                              </td>
                              <td class={"bg-red-500"}><div class={"cell-inner"}>{timestampItem.name}</div></td>
                              <td class={"bg-red-500"}><div class={"cell-inner"}>{timestampItem.doc}</div></td>
                              <td class={"bg-red-500"}><div class={"cell-inner"}>{timestampItem.room}</div></td>
                              <td class={"bg-red-500"}><div class={"cell-inner"}>{timestampItem.timestampLeft}</div></td>
                              <td class={"bg-red-500"}><div class={"cell-inner"}>{timestampItem.destinationLabel}</div></td>
                           </tr>
                        );
                     }}
                    </For>

                    <For each={tableData()} fallback={<p>No Items</p>}>
                        {(item) => {
                            const timestampItem = item.item as STimestampResident;
                            return (
                                <tr>
                                    <td>
                                        <div class={"cell-inner"}>
                                            <label class={"stable-checkbox"}>
                                                <input class={"stable-checkbox-input"}
                                                type={"checkbox"}
                                                checked={item.selected}
                                                onChange={(e) => handleSelectRow(e, item)} />
                                                <span class={"stable-checkbox-checkmark"}></span>
                                            </label>
                                        </div>
                                    </td>
                                    <td><div class={"cell-inner"}>{timestampItem.name}</div></td>
                                    <td><div class={"cell-inner"}>{timestampItem.doc}</div></td>
                                    <td><div class={"cell-inner"}>{timestampItem.room}</div></td>
                                    <td><div class={"cell-inner"}>{timestampItem.timestampLeft}</div></td>
                                    <td><div class={"cell-inner"}>{timestampItem.destinationLabel}</div></td>
                                    {props.actions ? ( 
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
                                    ) : false}
                                </tr>
                            );
                        }}
                    </For>
                  </>

                ) : false}


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
