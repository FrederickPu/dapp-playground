/*
 * Copyright (C) 2022 - 2023 Partisia Blockchain Foundation
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

import { ContractAbi } from "@partisiablockchain/abi-client";
import { BlockchainPublicKey } from "@partisiablockchain/zk-client";
import { ShardedClient } from "./client/ShardedClient";
import { TransactionApi } from "./client/TransactionApi";
import { ConnectedWallet } from "./ConnectedWallet";
import { CommentSectionApi } from "./contract/CommentSectionApi";
import { updateContractState } from "./WalletIntegration";

export const CLIENT = new ShardedClient("https://node1.testnet.partisiablockchain.com", [
  "Shard0",
  "Shard1",
  "Shard2",
]);

let contractAddress: string | undefined;
let currentAccount: ConnectedWallet | undefined;
let contractAbi: ContractAbi | undefined;
let concatApi: CommentSectionApi | undefined;
let engineKeys: BlockchainPublicKey[] | undefined;

export const setAccount = (account: ConnectedWallet | undefined) => {
  currentAccount = account;
  setConcatApi();
};

export const resetAccount = () => {
  currentAccount = undefined;
};

export const isConnected = () => {
  return currentAccount != null;
};

export const setContractAbi = (abi: ContractAbi | undefined) => {
  contractAbi = abi;
  setConcatApi();
};

export const getContractAbi = () => {
  return contractAbi;
};

export const setConcatApi = () => {
  if (currentAccount != undefined && contractAbi != undefined && engineKeys !== undefined) {
    const transactionApi = new TransactionApi(currentAccount, updateContractState);
    concatApi = new CommentSectionApi(
      transactionApi,
      currentAccount.address,
      contractAbi,
      engineKeys
    );
  }
};

export const getConcatApi = () => {
  return concatApi;
};

export const getEngineKeys = () => {
  return engineKeys;
};

export const setEngineKeys = (keys: BlockchainPublicKey[] | undefined) => {
  engineKeys = keys;
  setConcatApi();
};

export const getContractAddress = () => {
  return contractAddress;
};

export const setContractAddress = (address: string) => {
  contractAddress = address;
};
