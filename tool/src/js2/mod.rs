use std::collections::HashMap;
use std::path::Path;

use diplomat_core::Env;

use diplomat_core::hir::{self, TypeContext, TypeId};

use crate::common::FileMap;

use self::formatter::JSFormatter;

mod formatter;

/// Wrapper for generating all export types.
/// 
/// .d.ts definitions, basically. Although we include .mjs so anyone using modules also knows what they're importing.
pub struct JSGenerationContext<'tcx> {
    pub tcx: &'tcx TypeContext,
    formatter : JSFormatter<'tcx>,

    files : FileMap,
}

impl<'tcx> JSGenerationContext<'tcx> {
    pub fn run(tcx : &'tcx TypeContext, strip_prefix : Option<String>) -> FileMap {
        let this = Self {
            tcx,
            formatter: JSFormatter::new(tcx, strip_prefix),

            files: FileMap::default(),
        };
        this.init();

        this.files
    }

    /// Setup. Write out all the pre-written files.
    /// 
    /// Then iterate through all the types we get from the TypeContext to create separate out files.
    pub fn init(&self) {
        self.files.add_file("diplomat-runtime.mjs".into(), include_str!("runtime.mjs").into());
        self.files.add_file("diplomat-runtime.d.ts".into(), include_str!("runtime.d.ts").into());
        self.files.add_file("diplomat-wasm.mjs".into(), include_str!("wasm.mjs").into());

        
        // TODO: All of this.

        for (id, ty) in self.tcx.all_types() {
            if ty.attrs().disable {
                continue;
            }

            self.generate_file_from_type(id);
        }
        
        self.files.add_file("index.mjs".into(), "export { FFIError } from './diplomat-runtime.mjs';".into());
        self.files.add_file("index.d.ts".into(), "".into());
    }

    fn generate_file_from_type(&self, type_id : TypeId) {
        let type_def = self.tcx.resolve_type(type_id);

        let name = self.formatter.fmt_type_name(type_id);
        

        self.files.add_file(self.formatter.fmt_mjs_file_name(&name), "This is a test".into());
        self.files.add_file(self.formatter.fmt_ts_file_name(&name), "This is another test".into());

    } 
}
