#ifndef MyStructContainingAnOption_HPP
#define MyStructContainingAnOption_HPP

#include "MyStructContainingAnOption.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "DefaultEnum.hpp"
#include "MyStruct.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    diplomat::capi::MyStructContainingAnOption MyStructContainingAnOption_new(void);
    
    diplomat::capi::MyStructContainingAnOption MyStructContainingAnOption_filled(void);
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline MyStructContainingAnOption MyStructContainingAnOption::new_() {
  auto result = diplomat::capi::MyStructContainingAnOption_new();
  return MyStructContainingAnOption::FromFFI(result);
}

inline MyStructContainingAnOption MyStructContainingAnOption::filled() {
  auto result = diplomat::capi::MyStructContainingAnOption_filled();
  return MyStructContainingAnOption::FromFFI(result);
}


inline diplomat::capi::MyStructContainingAnOption MyStructContainingAnOption::AsFFI() const {
  return diplomat::capi::MyStructContainingAnOption {
    /* .a = */ a.has_value() ? (diplomat::capi::MyStruct_option{ { a.value().AsFFI() }, true }) : (diplomat::capi::MyStruct_option{ {}, false }),
    /* .b = */ b.has_value() ? (diplomat::capi::DefaultEnum_option{ { b.value().AsFFI() }, true }) : (diplomat::capi::DefaultEnum_option{ {}, false }),
  };
}

inline MyStructContainingAnOption MyStructContainingAnOption::FromFFI(diplomat::capi::MyStructContainingAnOption c_struct) {
  return MyStructContainingAnOption {
    /* .a = */ c_struct.a.is_ok ? std::optional(MyStruct::FromFFI(c_struct.a.ok)) : std::nullopt,
    /* .b = */ c_struct.b.is_ok ? std::optional(DefaultEnum::FromFFI(c_struct.b.ok)) : std::nullopt,
  };
}


#endif // MyStructContainingAnOption_HPP
