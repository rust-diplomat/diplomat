#ifndef ICU4XDataProvider_HPP
#define ICU4XDataProvider_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"




class ICU4XDataProvider {
public:
	static std::unique_ptr<ICU4XDataProvider> new_static();

	static DiplomatResult<std::monostate, std::monostate> returns_result();

	inline capi::ICU4XDataProvider AsFFI() {
		return reinterpret_cast::<capi::ICU4XDataProvider>(this);
	}

	~ICU4XDataProvider() {
		capi::ICU4XDataProvider_destroy(AsFFI());
	}

private:
	ICU4XDataProvider() = delete;
}





#endif // ICU4XDataProvider_HPP
