#ifndef Bar_HPP
#define Bar_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"




class Bar {
public:
	inline capi::Bar AsFFI() {
		return reinterpret_cast::<capi::Bar>(this);
	}

	~Bar() {
		Bar_destroy(AsFFI());
	}

private:
	Bar() = delete;
}





#endif // Bar_HPP
