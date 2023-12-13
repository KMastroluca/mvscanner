import { STimestampResident, SResident } from "./models";
import { Accessor, JSXElement } from "solid-js";
import { IconTree } from "solid-icons";

// ResidentIDModal
export interface ResidentIDModalProps {
  open: Accessor<boolean>;
  rfid: string;
  close: () => void;
  create: (newResident: SResident) => void;
}

export type ResidentIDModalValidationErrors = {
  firstNameError: string | null;
  lastNameError: string | null;
  docError: string | null;
  housingUnitError: string | null;
  roomError: string | null;
  bunkError: string | null;
};

// Popover
export type PopoverContentProps = {
  item: STimestampResident | SResident;
  cellContent: string;
};

// Scanner
export interface ScannerProps {
  displayNewResidentModal: (rfid: string) => void;
  refetchData: () => void;
}

// Icon
type IconEndType = {
  position: "End";
};

type IconStartType = {
  position: "Start";
};

type IconOnlyType = {
  position: "Only";
};

export type IconButtonType = IconEndType | IconStartType | IconOnlyType;

export interface IconButtonProps {
  icon: IconTree;
  children: JSXElement;
  size?: number;
  iconPosition: IconButtonType;
  color?: string;
  onClick: (e: MouseEvent) => void;
}

// EditResidentModal

export type EditResidentModalValidationErrors = {
  firstNameError: string | null;
  lastNameError: string | null;
  docError: string | null;
  housingUnitError: string | null;
  podError: string | null;
  roomError: string | null;
  bunkError: string | null;
};

export interface ResidentEditModalProps {
  open: Accessor<boolean>;
  currentResident: Accessor<SResident | null>;
  editResident: (resident: SResident) => void;
  close: () => void;
}

export interface DeconstructedResidentRoom {
  podLetter: string | "A" | "B" | "C";
  roomNumber: number;
  bunk: "T" | "B";
}
