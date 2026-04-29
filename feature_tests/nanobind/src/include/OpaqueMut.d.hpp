#ifndef SOMELIB_OpaqueMut_D_HPP
#define SOMELIB_OpaqueMut_D_HPP

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
namespace capi { struct OpaqueMut; }
class OpaqueMut;
} // namespace somelib



namespace somelib {
namespace capi {
    struct OpaqueMut;


    typedef struct DiplomatOpaqueMutView {
      const OpaqueMut** data;
      size_t len;
    } DiplomatOpaqueMutView;
} // namespace capi
} // namespace

namespace somelib {
class OpaqueMut {
public:

  inline static std::unique_ptr<somelib::OpaqueMut> new_();

    inline const somelib::capi::OpaqueMut* AsFFI() const;
    inline somelib::capi::OpaqueMut* AsFFI();
    inline static const somelib::OpaqueMut* FromFFI(const somelib::capi::OpaqueMut* ptr);
    inline static somelib::OpaqueMut* FromFFI(somelib::capi::OpaqueMut* ptr);
    inline static void operator delete(void* ptr);
private:
    OpaqueMut() = delete;
    OpaqueMut(const somelib::OpaqueMut&) = delete;
    OpaqueMut(somelib::OpaqueMut&&) noexcept = delete;
    OpaqueMut operator=(const somelib::OpaqueMut&) = delete;
    OpaqueMut operator=(somelib::OpaqueMut&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_OpaqueMut_D_HPP
