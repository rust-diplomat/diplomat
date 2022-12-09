#ifndef ICU4XFixedDecimal_D_HPP
#define ICU4XFixedDecimal_D_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XFixedDecimal.d.h"




class ICU4XFixedDecimal {
public:
	static std::unique_ptr<ICU4XFixedDecimal> inline new_(int32_t v);

	void inline multiply_pow10(int16_t power);

	void inline negate();

	DiplomatResult<std::string, std::monostate> inline to_string() const;

	inline capi::ICU4XFixedDecimal* AsFFI();

	inline ~ICU4XFixedDecimal();

private:
	ICU4XFixedDecimal() = delete;
};





#endif // ICU4XFixedDecimal_D_HPP
