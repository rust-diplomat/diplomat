#ifndef SOMELIB_ns_RenamedMixinTest_D_HPP
#define SOMELIB_ns_RenamedMixinTest_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"


namespace somelib::ns {
namespace capi {
    struct RenamedMixinTest;
    extern "C" {
    void namespace_MixinTest_destroy(RenamedMixinTest* self);
    }
} // namespace capi
} // namespace

namespace somelib::ns {
class RenamedMixinTest;
using RenamedMixinTestRef = somelib::diplomat::Ref<RenamedMixinTest, const somelib::ns::capi::RenamedMixinTest>;
using RenamedMixinTestRefMut = somelib::diplomat::Ref<RenamedMixinTest, somelib::ns::capi::RenamedMixinTest>;

class RenamedMixinTest : public somelib::diplomat::OpaquePointer<RenamedMixinTest, somelib::ns::capi::RenamedMixinTest, somelib::ns::capi::namespace_MixinTest_destroy> {
public:

  inline static std::string hello();
  template<typename W>
  inline static void hello_write(W& writeable_output);

};

} // namespace
#endif // SOMELIB_ns_RenamedMixinTest_D_HPP
