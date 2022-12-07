#ifndef ICU4XLocale_H
#define ICU4XLocale_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"




class ICU4XLocale {
public:
	static std::unique_ptr<ICU4XLocale> new_(std::string_view name);

	static std::unique_ptr<ICU4XLocale> new_from_bytes(const std::span<uint8_t> bytes);

	inline capi::ICU4XLocale AsFFI() {
		return reinterpret_cast::<capi::ICU4XLocale>(this);
	}

	~ICU4XLocale() {
		ICU4XLocale_destroy(AsFFI());
	}

private:
	ICU4XLocale() = delete;
}





#endif // ICU4XLocale_HPP
