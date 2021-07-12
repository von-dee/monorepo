/* eslint-disable @typescript-eslint/explicit-module-boundary-types */
/* eslint-disable @typescript-eslint/no-floating-promises */
/* eslint-disable @typescript-eslint/no-explicit-any */

export class NewWasmPromise {
  static sleeping = false;
  static result: boolean;
  static view: Int32Array;

  static create(
    asyncFunc: (...args: any[]) => Promise<boolean>,
    config: { exports: Record<string, any>; memory: WebAssembly.Memory }
  ) {
    const dataAddr = 16;

    const wrapperFunc = (...args: any[]) => {
      console.log("INSIDE MAIN: ", config.exports.values.asyncify_get_state());

      if (!this.sleeping) {
        console.log("MOVING VIEW");
        this.view[dataAddr >> 2] = dataAddr + 8;
        this.view[(dataAddr + 4) >> 2] = 1024;
        console.log("START UNWIND");
        config.exports.values.asyncify_start_unwind(dataAddr);
        this.sleeping = true;

        asyncFunc(...args).then((data) => {
          this.result = data;
          console.log("START REWIND");
          config.exports.values.asyncify_start_rewind(dataAddr);
          console.log("CALL MAIN 2");
          wrapperFunc(...args);
        });
      } else {
        console.log("STOP REWIND");
        config.exports.values.asyncify_stop_rewind();
        this.sleeping = false;
      }
    };

    return (...args: any[]) => {
      console.log(config.exports.values);
      this.view = new Int32Array(config.memory as any);
      console.log("CALL MAIN 1");
      console.log("BEFORE MAIN: ", config.exports.values.asyncify_get_state());
      wrapperFunc(...args);
      console.log("AFTER MAIN: ", config.exports.values.asyncify_get_state());
      console.log("STOP UNWIND");
      config.exports.values.asyncify_stop_unwind();
      console.log(
        "AFTER UNWOUND: ",
        config.exports.values.asyncify_get_state()
      );

      return this.result;
    };
  }
}
