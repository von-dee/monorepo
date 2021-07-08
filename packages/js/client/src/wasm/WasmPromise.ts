import { W3Exports } from "./types";

/* eslint-disable @typescript-eslint/naming-convention */
export class WasmPromise<T> {
  static create<T>(
    func: (...args: unknown[]) => Promise<T>,
    context: {
      memory: WebAssembly.Memory;
      exports: { values: W3Exports };
      setLock: (lock?: Promise<void>) => void;
      getLock: () => Promise<void> | undefined;
    }
  ): (...args: unknown[]) => T {
    const view = new Int32Array(context.memory.buffer);
    let sleeping = false;
    let result: T;

    return (...args: unknown[]) => {
      if (sleeping) {
        console.log("asyncify_stop_rewind")
        context.exports.values.asyncify_stop_rewind();
        sleeping = false;
        context.setLock(undefined);
      } else {
        const dataAddr = context.exports.values._w3_asyncify_storage();

        let resolve: (() => void) | undefined;
        let reject: ((err: Error) => void) | undefined;
        context.setLock(new Promise((res, rej) => {
          resolve = res;
          reject = rej;
        }));

        view[dataAddr >> 2] = dataAddr + 8;
        view[(dataAddr + 4) >> 2] = 100 * 1024;
        console.log("asyncify_start_unwind")
        context.exports.values.asyncify_start_unwind(dataAddr);
        sleeping = true;

        console.log("exec")
        func(...args).then(async (res: T) => {
          const lock = context.getLock();
          if (lock) {
            console.log("awaiting unwind");
            await lock;
          }

          console.log("asyncify_start_rewind")
          console.log(context.memory.buffer)
          context.exports.values.asyncify_start_rewind(dataAddr);
          result = res;

          if (!resolve) {
            throw Error("WasmPromise resolve undefined, this should never happen.");
          }
          resolve();
        }).catch((e) => {
          if (!reject) {
            throw Error("WasmPromise reject undefined, this should never happen.");
          }
          reject(e);
        });
      }

      return result;
    };
  }
}
