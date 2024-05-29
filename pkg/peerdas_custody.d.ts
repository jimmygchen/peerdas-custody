/* tslint:disable */
/* eslint-disable */
/**
* @returns {number}
*/
export function get_data_column_sidecar_subnet_count(): number;
/**
* @param {string} node_id
* @param {number | undefined} [subnet_count]
* @returns {Uint32Array}
*/
export function get_custody_subnets(node_id: string, subnet_count?: number): Uint32Array;
/**
* @param {string} peer_id
* @param {number | undefined} [subnet_count]
* @returns {Uint32Array}
*/
export function get_custody_subnets_from_peer_id(peer_id: string, subnet_count?: number): Uint32Array;
/**
* @param {Uint32Array} custody_subnets
* @returns {Uint32Array}
*/
export function get_custody_columns(custody_subnets: Uint32Array): Uint32Array;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly get_data_column_sidecar_subnet_count: () => number;
  readonly get_custody_subnets: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly get_custody_subnets_from_peer_id: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly get_custody_columns: (a: number, b: number, c: number) => void;
  readonly ring_core_0_17_8_bn_mul_mont: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
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
