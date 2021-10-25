export class CancelablePromise<T> {

  public isCanceled: boolean = false;

  private readonly _promise: Promise<T>;
  private _resolve: (value?: T | PromiseLike<T>) => void;
  private _reject: (rease?: unknown) => void;
  private _onCancel?: () => void;

  constructor(
    executor: (
      resolve: (result: T | PromiseLike<T>) => void,
      reject: (reason?: unknown) => void,
    ) => void,
    onCancel?: () => void,
  ) {
    this._promise = new Promise((resolve, reject) => {
      return executor(
        (...args) => !this.isCanceled && resolve(...args),
        (error) => !this.isCanceled && reject(error),
      )
    });
    this._onCancel = onCancel;
  }

  cancel(): void {
    this.isCanceled = true;
    this._onCancel && this._onCancel();
  }

  then(
    onfulfilled?: (value: T) => T | PromiseLike<T>,
    onrejected?: (reason: any) => PromiseLike<never>
  ): Promise<T> {
    return this._promise.then(onfulfilled, onrejected)
  }

  catch(
    onRejected?: (reason: unknown) => PromiseLike<never>
  ): Promise<T> {
    return this._promise.catch(onRejected);
  }

  resolve(value?: T | PromiseLike<T>): void {
    return this._resolve(value);
  }

  static resolve<T>(value: T | PromiseLike<T>): CancelablePromise<T> {
    return new CancelablePromise((resolve) => {
      resolve(value);
    });
  }

  reject(reason?: unknown): void {
    return this._reject(reason);
  }
}

export function createCancelablePromise<T>(
  promise: Promise<T>,
  onCancel?: () => void
): CancelablePromise<T> {
  return new CancelablePromise<T>((resolve, reject) => {
    return promise
      .then((...args) => resolve(...args))
      .catch((error) => reject(error));
  }, onCancel);
}
