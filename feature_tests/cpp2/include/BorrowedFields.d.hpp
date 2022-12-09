#ifndef BorrowedFields_D_HPP
#define BorrowedFields_D_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.h"




struct BorrowedFields {
	std::span<const uint16_t> a;
	std::string_view b;
};





#endif // BorrowedFields_D_HPP
