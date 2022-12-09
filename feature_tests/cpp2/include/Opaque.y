#ifndef Opaque_HPP
#define Opaque_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ImportedStruct.y"
#include "MyStruct.y"




class Opaque {
public:
	static std::unique_ptr<Opaque> new_();

	void assert_struct(MyStruct s) const;

	static size_t returns_usize();

	static ImportedStruct returns_imported();

	inline capi::Opaque AsFFI() {
		return reinterpret_cast::<capi::Opaque>(this);
	}

	~Opaque() {
		capi::Opaque_destroy(AsFFI());
	}

private:
	Opaque() = delete;
}





#endif // Opaque_HPP
