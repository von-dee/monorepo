const asyncifyStorage = new ArrayBuffer(100 * 1024);

export function w3_asyncify_storage(): u32 {
  return changetype<u32>(asyncifyStorage);
}
