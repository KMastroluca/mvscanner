export type SortType = StringSort | NumericalSort | DateSort;
export type SortDataString = { type: "String" };
export type SortDataNumerical = { type: "Numerical" };
export type SortDataDate = { type: "Date" };
export type SortDataType = SortDataString | SortDataNumerical | SortDataDate;

export enum StringSort {
  Alphabetical = 0,
  AlphaReverse = 1,
}

export enum NumericalSort {
  Ascending = 2,
  Descending = 3, // Corrected from Decending
}

export enum DateSort {
  LatestFirst = 4,
  OldestFirst = 5,
}
