#ifndef RefListParameter_H
#define RefListParameter_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"




class RefListParameter {
public:
	inline capi::RefListParameter AsFFI() {
		return reinterpret_cast::<capi::RefListParameter>(this);
	}

	~RefListParameter() {
		RefListParameter_destroy(AsFFI());
	}

private:
	RefListParameter() = delete;
}





#endif // RefListParameter_HPP
