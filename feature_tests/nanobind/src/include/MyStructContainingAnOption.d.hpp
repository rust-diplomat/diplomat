#ifndef MyStructContainingAnOption_D_HPP
#define MyStructContainingAnOption_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "DefaultEnum.d.hpp"
#include "MyStruct.d.hpp"
#include "diplomat_runtime.hpp"

struct MyStruct;
class DefaultEnum;


namespace diplomat {
namespace capi {
    struct MyStructContainingAnOption {
      diplomat::capi::MyStruct_option a;
      diplomat::capi::DefaultEnum_option b;
    };

    typedef struct MyStructContainingAnOption_option {union { MyStructContainingAnOption ok; }; bool is_ok; } MyStructContainingAnOption_option;

} // namespace capi
} // namespace


struct MyStructContainingAnOption {
  std::optional<MyStruct> a;
  std::optional<DefaultEnum> b;

  inline static MyStructContainingAnOption new_();

  inline static MyStructContainingAnOption filled();

  inline diplomat::capi::MyStructContainingAnOption AsFFI() const;
  inline static MyStructContainingAnOption FromFFI(diplomat::capi::MyStructContainingAnOption c_struct);
};


#endif // MyStructContainingAnOption_D_HPP
