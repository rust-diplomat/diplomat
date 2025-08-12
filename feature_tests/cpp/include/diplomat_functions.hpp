#ifndef diplomat_functions_HPP
#define diplomat_functions_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {

} // namespace capi
} // namespace


  

inline void free_callback_holder(std::function<diplomat::result<std::monostate, std::monostate>()> f) {
  diplomat::capi::free_callback_holder({new decltype(f)(std::move(f)), diplomat::fn_traits(f).template c_run_callback_result<std::monostate, std::monostate, diplomat::capi::DiplomatCallback_free_callback_holder_f_result>, diplomat::fn_traits(f).c_delete});
}

#endif // diplomat_functions_HPP
