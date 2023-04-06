import { useState } from "./state";

export type Task = (...args: any) => Promise<any>;
export type TaskResult<T extends Task> = T extends (
  ...args: any
) => Promise<infer U>
  ? U
  : never;
export type FirstTaskParams<T extends any[]> = Parameters<T[0]>;
export type LastTaskReturn<T> = T extends [...any[], infer R]
  ? R extends Task
    ? ReturnType<R>
    : never
  : never;

export function chainedTask<T extends Task[]>(
  ...tasks: T
): (...args: FirstTaskParams<T>) => Promise<LastTaskReturn<T>> {
  let result: any;
  return async (...args) => {
    result = args;
    for (const task of tasks) {
      result = [await task(...result)];
    }
    return result[0] as any as LastTaskReturn<T>;
  };
}

export interface TaskInstance<T extends Task> {
  exec(...params: [...Parameters<T>]): Promise<TaskResult<T>>;
  map<R = any>(
    fn: (params: TaskResult<T>) => R | Promise<R>
  ): TaskInstance<(...args: [...Parameters<T>]) => Promise<R>>;
  reset: () => void;
  result: TaskResult<T> | null;
  initial: boolean;
  failed: boolean;
  success: boolean;
  error: Error | null;
  finished: boolean;
  running: boolean;
}

export function useTask<T extends Task>(
  task: T,
  config?: {
    autoExecParams?: Parameters<T>;
    default?: TaskResult<T>;
    msg?: (m: string) => void;
  }
): TaskInstance<T> {
  const { state, resetState } = useState({
    result: config?.default ?? (null as null | TaskResult<T>),
    initial: true,
    failed: false,
    success: false,
    error: null as null | Error,
    running: false,
    finished: false,
  });

  const tasks: Array<Task> = [];

  async function exec(...params: [...Parameters<T>]): Promise<TaskResult<T>> {
    state.initial = false;
    state.running = true;
    state.success = false;
    state.failed = false;
    state.error = null;
    state.finished = false;

    return tasks[0](...params)
      .then(async (val: any) => {
        for (const task of tasks.slice(1)) {
          val = await task(val);
        }
        state.result = val;
        state.success = true;
        return val;
      })
      .catch((e: Error) => {
        state.error = e;
        state.failed = true;
        if (config?.msg) {
          config.msg(e.toString());
        }
        throw e;
      })
      .finally(() => {
        state.running = false;
        state.finished = true;
      });
  }

  function map<U>(fn: (params: TaskResult<T>) => Promise<U>) {
    tasks.push(fn);
    return Object.assign(state, {
      exec,
      map,
      reset: resetState,
    });
  }

  const ret = map(task);

  if (config?.autoExecParams) {
    exec.apply(null, config.autoExecParams);
  }

  return ret;
}
