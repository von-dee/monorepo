import { W3Exports } from "./types";

/* eslint-disable @typescript-eslint/naming-convention */
export class WasmPromise<T> {
  private _result: T;
  private _exports: { values: W3Exports };
  private _view: Int32Array;
  private _dataAddr: number;
  private _sleeping = false;
  private _setAsync: (asyncifyAwait: Promise<void>) => void;

  static create<T>(
    func: (...args: unknown[]) => Promise<T>,
    config: {
      memory: WebAssembly.Memory;
      exports: { values: W3Exports };
      setAsync: (asyncifyAwait: Promise<void>) => void;
    }
  ): (...args: unknown[]) => T {
    console.log("INIT")
    const instance = new WasmPromise<T>();
    instance._exports = config.exports;
    instance._view = new Int32Array(config.memory.buffer);
    instance._setAsync = config.setAsync;

    return (...args: unknown[]) => {
      console.log("GETTING ASYNCIFY_STORAGE")
      instance._dataAddr = instance._exports.values._w3_asyncify_storage();
      console.log("GOT_ASYNCIFY_STORAGE")

      let resolve: (() => void) | undefined;
      let reject: ((err: Error) => void) | undefined;
      instance._setAsync(new Promise((res, rej) => {
        resolve = res;
        reject = rej;
      }))

      instance.execAsyncFunc(() => {
        console.log("executing")
        func(...args).then((result: T) => {
          console.log("then...")
          console.log("start_rewind", instance._dataAddr);
          instance._exports.values.asyncify_start_rewind(instance._dataAddr);
          instance._result = result;

          console.log("resolve", resolve)
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
      });

      return instance._result;
    };
  }

  private execAsyncFunc(exec: () => void) {
    if (!this._sleeping) {
      this._view[this._dataAddr >> 2] = this._dataAddr + 8;
      this._view[(this._dataAddr + 4) >> 2] = 100 * 1024;

      console.log("start_unwind")
      this._exports.values.asyncify_start_unwind(this._dataAddr);
      this._sleeping = true;
      exec();
    } else {
      console.log("stop_rewind")
      this._exports.values.asyncify_stop_rewind();
      this._sleeping = false;
    }
  }
}
