#ifndef ICU4XDataProvider_D_HPP
#define ICU4XDataProvider_D_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.h"
#include "ICU4XDataProvider.d.h"




class ICU4XDataProvider {
public:
	inline static std::unique_ptr<ICU4XDataProvider> new_static();

	inline static DiplomatResult<std::monostate, std::monostate> returns_result();

	inline const capi::ICU4XDataProvider* AsFFI() const;
	inline capi::ICU4XDataProvider* AsFFI();

	inline ~ICU4XDataProvider();

private:
	ICU4XDataProvider() = delete;
};





#endif // ICU4XDataProvider_D_HPP
