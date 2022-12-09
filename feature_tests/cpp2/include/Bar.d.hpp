#ifndef Bar_D_HPP
#define Bar_D_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "Bar.d.h"




class Bar {
public:
	inline capi::Bar* AsFFI();

	inline ~Bar();

private:
	Bar() = delete;
};





#endif // Bar_D_HPP
