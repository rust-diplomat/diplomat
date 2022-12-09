#ifndef Opaque_D_HPP
#define Opaque_D_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ImportedStruct.d.hpp"
#include "MyStruct.d.hpp"
#include "Opaque.d.h"




class Opaque {
public:
	static std::unique_ptr<Opaque> inline new_();

	void inline assert_struct(MyStruct s) const;

	static size_t inline returns_usize();

	static ImportedStruct inline returns_imported();

	inline capi::Opaque* AsFFI();

	inline ~Opaque();

private:
	Opaque() = delete;
};





#endif // Opaque_D_HPP
