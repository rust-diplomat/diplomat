#ifndef Two_H
#define Two_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"




class Two {
public:
	inline capi::Two AsFFI() {
		return reinterpret_cast::<capi::Two>(this);
	}

	~Two() {
		Two_destroy(AsFFI());
	}

private:
	Two() = delete;
}





#endif // Two_HPP
