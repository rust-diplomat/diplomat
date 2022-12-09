#ifndef Opaque_HPP
#define Opaque_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"




	static std::unique_ptr<Opaque> inline new_() {
	// TODO
}
	void inline assert_struct(MyStruct s) const {
	// TODO
}
	static size_t inline returns_usize() {
	// TODO
}
	static ImportedStruct inline returns_imported() {
	// TODO
}
inline capi::Opaque* AsFFI() {
	return reinterpret_cast::<capi::Opaque>(this);
}
inline ~Opaque() {
	capi::Opaque_destroy(AsFFI());
}


#endif // Opaque_HPP
