#ifndef SOMELIB_free_functions_D_HPP
#define SOMELIB_free_functions_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {

} // namespace capi
} // namespace

namespace somelib {



  inline void free_callback_holder(std::function<somelib::diplomat::result<std::monostate, std::monostate>()> f);



} // namespace
#endif // SOMELIB_free_functions_D_HPP
