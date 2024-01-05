/* tslint:disable */
/* eslint-disable */
/**
* @param {string} code
* @returns {string}
*/
export function codekit_create_ean8(code: string): string;
/**
* @param {string} code
* @returns {string}
*/
export function codekit_create_ean13(code: string): string;
/**
* @param {string} code
* @returns {string}
*/
export function codekit_create_codabar(code: string): string;
/**
* @param {string} code
* @returns {string}
*/
export function codekit_create_code39(code: string): string;
/**
* @param {string} code
* @returns {string}
*/
export function codekit_create_code93(code: string): string;
/**
* @param {string} code
* @returns {string}
*/
export function codekit_create_i2of5(code: string): string;
/**
* @param {string} canvas_id
* @param {string} code
* @param {number} bar_width
* @param {number} height
*/
export function codekit_draw_ean8(canvas_id: string, code: string, bar_width: number, height: number): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly codekit_create_ean8: (a: number, b: number, c: number) => void;
  readonly codekit_create_ean13: (a: number, b: number, c: number) => void;
  readonly codekit_create_codabar: (a: number, b: number, c: number) => void;
  readonly codekit_create_code39: (a: number, b: number, c: number) => void;
  readonly codekit_create_code93: (a: number, b: number, c: number) => void;
  readonly codekit_create_i2of5: (a: number, b: number, c: number) => void;
  readonly codekit_draw_ean8: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
