import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { BorrowedFields } from "./BorrowedFields.mjs"
import { BorrowedFieldsWithBounds } from "./BorrowedFieldsWithBounds.mjs"

export class NestedBorrowedFields {
  constructor(underlying, edges_x, edges_y, edges_z) {
    this.fields = new BorrowedFields(underlying, edges_x, edges_y);
    this.bounds = new BorrowedFieldsWithBounds(underlying + 24, edges_x, edges_y);
    this.bounds2 = new BorrowedFieldsWithBounds(underlying + 48, edges_z);
  }
}
