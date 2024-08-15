declare namespace wasm_bindgen {
	/* tslint:disable */
	/* eslint-disable */
	/**
	*/
	export function main(): void;
	/**
	*/
	export class APApplet {
	  free(): void;
	/**
	* @param {HTMLElement} div_plot
	* @param {HTMLTextAreaElement} text_area
	* @param {number} duration
	* @returns {APApplet}
	*/
	  static new(div_plot: HTMLElement, text_area: HTMLTextAreaElement, duration: number): APApplet;
	/**
	*/
	  plot(): void;
	/**
	* @param {number} duration
	*/
	  set_duration(duration: number): void;
	/**
	* @param {number} dt
	*/
	  set_timestep(dt: number): void;
	/**
	* @param {string} solver
	*/
	  set_solver(solver: string): void;
	}
	
}

declare type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

declare interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_apapplet_free: (a: number, b: number) => void;
  readonly apapplet_new: (a: number, b: number, c: number) => number;
  readonly apapplet_plot: (a: number, b: number) => void;
  readonly apapplet_set_duration: (a: number, b: number) => void;
  readonly apapplet_set_timestep: (a: number, b: number) => void;
  readonly apapplet_set_solver: (a: number, b: number, c: number) => void;
  readonly main: () => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_start: () => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
declare function wasm_bindgen (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
