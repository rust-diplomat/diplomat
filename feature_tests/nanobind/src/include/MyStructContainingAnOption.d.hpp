#ifndef SOMELIB_MyStructContainingAnOption_D_HPP
#define SOMELIB_MyStructContainingAnOption_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "DefaultEnum.d.hpp"
#include "MyStruct.d.hpp"
#include "diplomat_runtime.hpp"
namespace somelib {
struct MyStruct;
struct MyStructContainingAnOption;
class DefaultEnum;
} // namespace somelib



namespace somelib {
namespace capi {
    struct MyStructContainingAnOption {
      somelib::capi::MyStruct_option a;
      somelib::capi::DefaultEnum_option b;
    };

    typedef struct MyStructContainingAnOption_option {union { MyStructContainingAnOption ok; }; bool is_ok; } MyStructContainingAnOption_option;
} // namespace capi
} // namespace


namespace somelib {
struct MyStructContainingAnOption {
    std::optional<somelib::MyStruct> a;
    std::optional<somelib::DefaultEnum> b;

  inline static somelib::MyStructContainingAnOption new_();

  inline static somelib::MyStructContainingAnOption filled();

    inline somelib::capi::MyStructContainingAnOption AsFFI() const;
    inline static somelib::MyStructContainingAnOption FromFFI(somelib::capi::MyStructContainingAnOption c_struct);
};

} // namespace
#endif // SOMELIB_MyStructContainingAnOption_D_HPP
