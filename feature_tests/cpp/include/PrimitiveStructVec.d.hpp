#ifndef SOMELIB_PrimitiveStructVec_D_HPP
#define SOMELIB_PrimitiveStructVec_D_HPP

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
namespace capi { struct PrimitiveStructVec; }
class PrimitiveStructVec;
struct PrimitiveStruct;
} // namespace somelib
namespace somelib {
namespace ns {
struct RenamedStructWithAttrs;
} // namespace ns
} // namespace somelib



namespace somelib {
namespace capi {
    struct PrimitiveStructVec;
} // namespace capi
} // namespace

namespace somelib {
class PrimitiveStructVec {
public:

  inline static std::unique_ptr<somelib::PrimitiveStructVec> new_();

  inline void push(somelib::PrimitiveStruct value);

  inline size_t len() const;

  inline somelib::diplomat::span<const somelib::PrimitiveStruct> as_slice() const;

  inline somelib::diplomat::span<somelib::PrimitiveStruct> as_slice_mut();

  inline somelib::PrimitiveStruct get(size_t idx) const;

  inline static void take_slice_from_other_namespace(somelib::diplomat::span<const somelib::ns::RenamedStructWithAttrs> _sl);

    inline const somelib::capi::PrimitiveStructVec* AsFFI() const;
    inline somelib::capi::PrimitiveStructVec* AsFFI();
    inline static const somelib::PrimitiveStructVec* FromFFI(const somelib::capi::PrimitiveStructVec* ptr);
    inline static somelib::PrimitiveStructVec* FromFFI(somelib::capi::PrimitiveStructVec* ptr);
    inline static void operator delete(void* ptr);
private:
    PrimitiveStructVec() = delete;
    PrimitiveStructVec(const somelib::PrimitiveStructVec&) = delete;
    PrimitiveStructVec(somelib::PrimitiveStructVec&&) noexcept = delete;
    PrimitiveStructVec operator=(const somelib::PrimitiveStructVec&) = delete;
    PrimitiveStructVec operator=(somelib::PrimitiveStructVec&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_PrimitiveStructVec_D_HPP
