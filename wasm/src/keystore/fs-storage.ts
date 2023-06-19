// Copyright © 2017-2022 Trust Wallet.
//
// This file is part of Trust. The full Trust copyright notice, including
// terms governing use, modification, and redistribution, is contained in the
// file LICENSE at the root of the source code distribution tree.

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
