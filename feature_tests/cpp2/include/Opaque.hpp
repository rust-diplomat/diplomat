#ifndef Opaque_HPP
#define Opaque_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "Opaque.d.hpp"
#include "Opaque.h"




static std::unique_ptr<Opaque> inline Opaque::new_() {
	// TODO
}
void inline Opaque::assert_struct(MyStruct s) const {
	// TODO
}
static size_t inline Opaque::returns_usize() {
	// TODO
}
static ImportedStruct inline Opaque::returns_imported() {
	// TODO
}
inline capi::Opaque* Opaque::AsFFI() {
	return reinterpret_cast<capi::Opaque*>(this);
}
inline Opaque::~Opaque() {
	capi::Opaque_destroy(AsFFI());
}


#endif // Opaque_HPP
