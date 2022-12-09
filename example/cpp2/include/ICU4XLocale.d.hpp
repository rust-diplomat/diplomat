#ifndef ICU4XLocale_D_HPP
#define ICU4XLocale_D_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.h"
#include "ICU4XLocale.d.h"




class ICU4XLocale {
public:
	inline static std::unique_ptr<ICU4XLocale> new_(std::string_view name);

	inline static std::unique_ptr<ICU4XLocale> new_from_bytes(const std::span<uint8_t> bytes);

	inline capi::ICU4XLocale* AsFFI();

	inline ~ICU4XLocale();

private:
	ICU4XLocale() = delete;
};





#endif // ICU4XLocale_D_HPP
