declare global {
  interface Window {
    facilityLocationId: number;
    scannedRFID: string;
    lastScannedRFID: string;
    lastKeyPress: number;
    scanApiUrl: string;
    testScanMode: boolean;
  }
}

export {}; // This line is needed to ensure this file is treated as a module.
