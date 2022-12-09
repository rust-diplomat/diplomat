#ifndef Two_D_HPP
#define Two_D_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "Two.d.h"




class Two {
public:
	inline capi::Two* AsFFI();

	inline ~Two();

private:
	Two() = delete;
};





#endif // Two_D_HPP
