#ifndef SOMELIB_OpaqueThin_D_HPP
#define SOMELIB_OpaqueThin_D_HPP

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
    struct OpaqueThin;
} // namespace capi
} // namespace

namespace somelib {
class OpaqueThin {
public:

  inline int32_t a() const;

  inline float b() const;

  inline std::string c() const;
  template<typename W>
  inline void c_write(W& writeable_output) const;

    inline const somelib::capi::OpaqueThin* AsFFI() const;
    inline somelib::capi::OpaqueThin* AsFFI();
    inline static const somelib::OpaqueThin* FromFFI(const somelib::capi::OpaqueThin* ptr);
    inline static somelib::OpaqueThin* FromFFI(somelib::capi::OpaqueThin* ptr);
    inline static void operator delete(void* ptr);
private:
    OpaqueThin() = delete;
    OpaqueThin(const somelib::OpaqueThin&) = delete;
    OpaqueThin(somelib::OpaqueThin&&) noexcept = delete;
    OpaqueThin operator=(const somelib::OpaqueThin&) = delete;
    OpaqueThin operator=(somelib::OpaqueThin&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_OpaqueThin_D_HPP
