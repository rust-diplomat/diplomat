#ifndef Opaque_HPP
#define Opaque_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.h"
#include "Opaque.d.hpp"
#include "Opaque.h"




inline std::unique_ptr<Opaque> Opaque::new_() {
	// TODO
}
inline void Opaque::assert_struct(MyStruct s) const {
	// TODO
}
inline size_t Opaque::returns_usize() {
	// TODO
}
inline ImportedStruct Opaque::returns_imported() {
	// TODO
}
inline capi::Opaque* Opaque::AsFFI() {
	return reinterpret_cast<capi::Opaque*>(this);
}
inline Opaque::~Opaque() {
	capi::Opaque_destroy(AsFFI());
}


#endif // Opaque_HPP
