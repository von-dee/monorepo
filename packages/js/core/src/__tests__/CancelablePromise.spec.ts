import {
  CancelablePromise,
  createCancelablePromise
} from "../";

describe("CancelablePromise", () => {
  it("resolves", async () => {
    const promise = new Promise<boolean>((resolve) => {
      setTimeout(() => {
        resolve(true);
      }, 200);
    });
    const cancelable = createCancelablePromise(promise);
    const result = await cancelable;

    expect(result).toBe(true);
  });

  it("rejects", async () => {
    const promise = new Promise<boolean>((resolve, reject) => {
      setTimeout(() => {
        resolve(true);
      }, 400);
      setTimeout(() => {
        reject(new Error("rejection"));
      }, 200);
    });
    const cancelable = createCancelablePromise(promise);

    try {
      await cancelable;
    } catch (e) {
      expect(e.message).toBe("rejection");
      return;
    }

    expect("").toBe("should never reach here");
  });

  it("cancels", async () => {
    let result = false;
    const promise = new Promise<boolean>((resolve) => {
      setTimeout(() => {
        resolve(true);
      }, 200);
    });
    promise.then((value) => result = value);

    const cancelable = createCancelablePromise(promise);
    cancelable.cancel();

    await new Promise((resolve) => {
      setTimeout(() => {
        resolve(true);
      }, 100);
    });

    expect(result).toBe(false);
  });

  it("calls onCancel", async () => {
    let result = false;
    let onCancel = "foo";
    const promise = new CancelablePromise<boolean>(
      (resolve) => {
        setTimeout(() => {
          resolve(true);
        }, 200);
      },
      () => onCancel = "bar"
    );
    promise.then((value) => result = value);
    promise.cancel();

    await new Promise((resolve) => {
      setTimeout(() => {
        resolve(true);
      }, 100);
    });

    expect(result).toBe(false);
    expect(onCancel).toBe("bar");
  });
});
