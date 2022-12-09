#ifndef Foo_D_HPP
#define Foo_D_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.h"
#include "BorrowedFields.d.hpp"
#include "BorrowedFieldsReturning.d.hpp"
#include "Foo.d.h"


class Bar;


class Foo {
public:
	inline static std::unique_ptr<Foo> new_(std::string_view x);

	inline std::unique_ptr<Bar> get_bar() const;

	inline static std::unique_ptr<Foo> new_static(std::string_view x);

	inline BorrowedFieldsReturning as_returning() const;

	inline static std::unique_ptr<Foo> extract_from_fields(BorrowedFields fields);

	inline const capi::Foo* AsFFI() const;
	inline capi::Foo* AsFFI();

	inline ~Foo();

private:
	Foo() = delete;
};





#endif // Foo_D_HPP
