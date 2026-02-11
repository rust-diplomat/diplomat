#ifndef SOMELIB_ns_RenamedOpaqueZSTIterator_D_HPP
#define SOMELIB_ns_RenamedOpaqueZSTIterator_D_HPP

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
namespace capi { struct RenamedOpaqueZST; }
class RenamedOpaqueZST;
namespace capi { struct RenamedOpaqueZSTIterator; }
class RenamedOpaqueZSTIterator;
} // namespace ns
} // namespace somelib



namespace somelib::ns {
namespace capi {
    struct RenamedOpaqueZSTIterator;
} // namespace capi
} // namespace

namespace somelib::ns {
class RenamedOpaqueZSTIterator {
public:

  inline static std::unique_ptr<somelib::ns::RenamedOpaqueZSTIterator> ctor();

  inline std::unique_ptr<somelib::ns::RenamedOpaqueZSTIterator> next() const;

  inline std::unique_ptr<somelib::ns::RenamedOpaqueZSTIterator> operator[](size_t _idx) const;

  inline somelib::diplomat::result<std::string, std::unique_ptr<somelib::ns::RenamedOpaqueZST>> stringify() const;
  template<typename W>
  inline somelib::diplomat::result<std::monostate, std::unique_ptr<somelib::ns::RenamedOpaqueZST>> stringify_write(W& writeable_output) const;

    inline const somelib::ns::capi::RenamedOpaqueZSTIterator* AsFFI() const;
    inline somelib::ns::capi::RenamedOpaqueZSTIterator* AsFFI();
    inline static const somelib::ns::RenamedOpaqueZSTIterator* FromFFI(const somelib::ns::capi::RenamedOpaqueZSTIterator* ptr);
    inline static somelib::ns::RenamedOpaqueZSTIterator* FromFFI(somelib::ns::capi::RenamedOpaqueZSTIterator* ptr);
    inline static void operator delete(void* ptr);
private:
    RenamedOpaqueZSTIterator() = delete;
    RenamedOpaqueZSTIterator(const somelib::ns::RenamedOpaqueZSTIterator&) = delete;
    RenamedOpaqueZSTIterator(somelib::ns::RenamedOpaqueZSTIterator&&) noexcept = delete;
    RenamedOpaqueZSTIterator operator=(const somelib::ns::RenamedOpaqueZSTIterator&) = delete;
    RenamedOpaqueZSTIterator operator=(somelib::ns::RenamedOpaqueZSTIterator&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_ns_RenamedOpaqueZSTIterator_D_HPP
