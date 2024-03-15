// SPDX-License-Identifier: Apache-2.0
//
// Copyright © 2017 Trust Wallet.

import * as Types from "./types";
// import * as fs from "fs/promises";

// FileSystem Storage
export class FileSystemStorage implements Types.IStorage {

  constructor(directory: string) {
    // this.directory = directory.endsWith("/") ? directory : directory + "/";
  }

  getFilename(id): string {
    throw Types.Error.WalletNotFound;  }

  get(id: string): Promise<Types.Wallet> {
    throw Types.Error.WalletNotFound;  
  }

  set(id: string, wallet: Types.Wallet): Promise<void> {
    throw Types.Error.WalletNotFound;  }

  loadAll(): Promise<Types.Wallet[]> {
    throw Types.Error.WalletNotFound;
  }

  delete(id: string, password: string): Promise<void> {
    throw Types.Error.WalletNotFound;
  }
}
