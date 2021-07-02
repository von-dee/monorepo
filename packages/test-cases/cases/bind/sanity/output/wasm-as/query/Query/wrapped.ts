import {
  queryMethod,
  objectMethod
} from "../../index";
import {
  deserializequeryMethodArgs,
  serializequeryMethodResult,
  deserializeobjectMethodArgs,
  serializeobjectMethodResult
} from "./serialization";

export function queryMethodWrapped(argsBuf: ArrayBuffer): ArrayBuffer {
  const args = deserializequeryMethodArgs(argsBuf);
  const result = queryMethod({
    str: args.str,
    optStr: args.optStr,
    en: args.en,
    optEnum: args.optEnum,
    enumArray: args.enumArray,
    optEnumArray: args.optEnumArray
  });
  return serializequeryMethodResult(result);
}

export function objectMethodWrapped(inputPtr): ArrayBuffer {
  // const input: Input_objectMethod = changetype<Input_objectMethod>(inputPtr)
  const result = objectMethod(input);
  return serializeobjectMethodResult(result);
}
