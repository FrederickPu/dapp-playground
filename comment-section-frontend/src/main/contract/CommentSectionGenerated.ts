/* eslint-disable @typescript-eslint/no-unused-vars */
/* eslint-disable @typescript-eslint/no-non-null-assertion */
import BN from "bn.js";
import {
  AbiParser,
  AbstractBuilder, BigEndianReader,
  FileAbi, FnKinds, FnRpcBuilder, RpcReader,
  ScValue,
  ScValueEnum, ScValueOption,
  ScValueStruct,
  StateReader, TypeIndex,
  StateBytes,
  BlockchainAddress
} from "@partisiablockchain/abi-client";
import {BigEndianByteOutput} from "@secata-public/bitmanipulation-ts";

const fileAbi: FileAbi = new AbiParser(Buffer.from(
  "5042434142490b000005040000000004010000000d436f6e74726163745374617465000000030000000d61646d696e6973747261746f720d00000015636f6e6361745f6d6573736167655f726573756c7412030000000c6e756d5f636f6d6d656e74731203010000000b536563726574566172496400000001000000067261775f69640301000000134576656e74537562736372697074696f6e496400000001000000067261775f696408010000000f45787465726e616c4576656e74496400000001000000067261775f69640800000006010000000a696e697469616c697a65ffffffff0f00000000170000000b6164645f6d65737361676540000000000000000c7365637265745f696e707574081100000011696e7075747465645f7661726961626c65cbe680ff0b000000000200000016636f6d707574655f636f6e6361745f6d65737361676501000000001300000017636f6e6361745f636f6d707574655f636f6d706c657465dbed85ab0b0000000014000000146f70656e5f636f6e6361745f7661726961626c6588e8f3900b000000000000",
  "hex"
)).parseAbi();

type Option<K> = K | undefined;

export interface ContractState {
  administrator: BlockchainAddress;
  concatMessageResult: Option<number>;
  numComments: Option<number>;
}

export function newContractState(administrator: BlockchainAddress, concatMessageResult: Option<number>, numComments: Option<number>): ContractState {
  return {administrator, concatMessageResult, numComments};
}

function fromScValueContractState(structValue: ScValueStruct): ContractState {
  return {
    administrator: BlockchainAddress.fromBuffer(structValue.getFieldValue("administrator")!.addressValue().value),
    concatMessageResult: structValue.getFieldValue("concat_message_result")!.optionValue().valueOrUndefined((sc1) => sc1.asNumber()),
    numComments: structValue.getFieldValue("num_comments")!.optionValue().valueOrUndefined((sc2) => sc2.asNumber()),
  };
}

export function deserializeContractState(state: StateBytes): ContractState {
  const scValue = new StateReader(state.state, fileAbi.contract, state.avlTrees).readState();
  return fromScValueContractState(scValue);
}

export interface SecretVarId {
  rawId: number;
}

export function newSecretVarId(rawId: number): SecretVarId {
  return {rawId};
}

function fromScValueSecretVarId(structValue: ScValueStruct): SecretVarId {
  return {
    rawId: structValue.getFieldValue("raw_id")!.asNumber(),
  };
}

export interface EventSubscriptionId {
  rawId: number;
}

export function newEventSubscriptionId(rawId: number): EventSubscriptionId {
  return {rawId};
}

function fromScValueEventSubscriptionId(structValue: ScValueStruct): EventSubscriptionId {
  return {
    rawId: structValue.getFieldValue("raw_id")!.asNumber(),
  };
}

export interface ExternalEventId {
  rawId: number;
}

export function newExternalEventId(rawId: number): ExternalEventId {
  return {rawId};
}

function fromScValueExternalEventId(structValue: ScValueStruct): ExternalEventId {
  return {
    rawId: structValue.getFieldValue("raw_id")!.asNumber(),
  };
}

export function initialize(): Buffer {
  const fnBuilder = new FnRpcBuilder("initialize", fileAbi.contract);
  return fnBuilder.getBytes();
}

export function computeConcatMessage(): Buffer {
  const fnBuilder = new FnRpcBuilder("compute_concat_message", fileAbi.contract);
  return fnBuilder.getBytes();
}

