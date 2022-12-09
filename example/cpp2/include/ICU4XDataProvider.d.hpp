#ifndef ICU4XDataProvider_D_HPP
#define ICU4XDataProvider_D_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XDataProvider.d.h"




class ICU4XDataProvider {
public:
	static std::unique_ptr<ICU4XDataProvider> inline new_static();

	static DiplomatResult<std::monostate, std::monostate> inline returns_result();

	inline capi::ICU4XDataProvider* AsFFI();

	inline ~ICU4XDataProvider();

private:
	ICU4XDataProvider() = delete;
};





#endif // ICU4XDataProvider_D_HPP
