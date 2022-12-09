#ifndef BorrowedFields_HPP
#define BorrowedFields_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"




struct BorrowedFields {
	const std::span<uint16_t> a;
	std::string_view b;
};





#endif // BorrowedFields_HPP
