/**
 * An error that occurred in Rust.
 */
export class FFIError<E> extends Error {
	error_value: E;
}

export type char = string;
export type pointer = number;