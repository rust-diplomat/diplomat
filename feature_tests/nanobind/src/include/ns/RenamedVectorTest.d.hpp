#ifndef SOMELIB_ns_RenamedVectorTest_D_HPP
#define SOMELIB_ns_RenamedVectorTest_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"
namespace somelib {
namespace ns {
namespace capi { struct RenamedVectorTest; }
class RenamedVectorTest;
} // namespace ns
} // namespace somelib



namespace somelib::ns {
namespace capi {
    struct RenamedVectorTest;
    extern "C" {
    void namespace_VectorTest_destroy(RenamedVectorTest* self);
    }
} // namespace capi
} // namespace

namespace somelib::ns {
class RenamedVectorTest;
using RenamedVectorTestRef = somelib::diplomat::Ref<RenamedVectorTest, const somelib::ns::capi::RenamedVectorTest>;
using RenamedVectorTestRefMut = somelib::diplomat::Ref<RenamedVectorTest, somelib::ns::capi::RenamedVectorTest>;

class RenamedVectorTest : public somelib::diplomat::OpaquePointer<RenamedVectorTest, somelib::ns::capi::RenamedVectorTest, somelib::ns::capi::namespace_VectorTest_destroy> {
public:

  inline static somelib::ns::RenamedVectorTest new_();

  inline size_t len() const;

  inline somelib::diplomat::Optional<double> operator[](size_t idx) const;

  inline void push(double value);

};

} // namespace
#endif // SOMELIB_ns_RenamedVectorTest_D_HPP
