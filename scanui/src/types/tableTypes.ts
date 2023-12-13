import { SResident, STimestamp, SLocation, STimestampResident } from "./models"; // Adjust the import as necessary
import { SortDataType } from "./sortingTypes";
import { JSXElement } from "solid-js";

export interface STableProps {
  type: "Resident" | "Location" | "Timestamp" | "TimestampResident";
  data: STableData;
  actions?: STableAction[];
}

export interface STableData {
  data: SResident[] | STimestamp[] | SLocation[] | STimestampResident[];
  priorityData?:
    | SResident[]
    | STimestamp[]
    | SLocation[]
    | STimestampResident[];
}

export interface STableAction {
  actionLabel: string;
  actionFunction: (props: any) => void;
}

export interface STableColumn {
  label: string;
  key?: string;
  sortable: boolean;
  icon?: JSXElement;
  sortType?: SortDataType;
}

export interface STableRowItem {
  selected: boolean;
  item: SResident | SLocation | STimestamp | STimestampResident;
}
