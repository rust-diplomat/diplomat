#ifndef ICU4XFixedDecimal_HPP
#define ICU4XFixedDecimal_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"




class ICU4XFixedDecimal {
public:
	static std::unique_ptr<ICU4XFixedDecimal> new_(int32_t v);

	void multiply_pow10(int16_t power);

	void negate();

	DiplomatResult<std::string, std::monostate> to_string() const;

	inline capi::ICU4XFixedDecimal AsFFI() {
		return reinterpret_cast::<capi::ICU4XFixedDecimal>(this);
	}

	~ICU4XFixedDecimal() {
		capi::ICU4XFixedDecimal_destroy(AsFFI());
	}

private:
	ICU4XFixedDecimal() = delete;
}





#endif // ICU4XFixedDecimal_HPP
