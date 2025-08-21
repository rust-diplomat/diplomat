#ifndef ns_RenamedDeprecated_D_HPP
#define ns_RenamedDeprecated_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"


namespace ns {
namespace capi {

} // namespace capi
} // namespace


namespace ns {
[[deprecated("use Foo")]]
struct RenamedDeprecated {

};

} // namespace
#endif // ns_RenamedDeprecated_D_HPP
