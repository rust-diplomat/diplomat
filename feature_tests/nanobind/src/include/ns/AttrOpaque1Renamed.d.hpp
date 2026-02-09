#ifndef SOMELIB_ns_AttrOpaque1Renamed_D_HPP
#define SOMELIB_ns_AttrOpaque1Renamed_D_HPP

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
namespace capi { struct Unnamespaced; }
class Unnamespaced;
} // namespace somelib
namespace somelib {
namespace ns {
class RenamedAttrEnum;
} // namespace ns
} // namespace somelib



namespace somelib::ns {
namespace capi {
    struct AttrOpaque1Renamed;
} // namespace capi
} // namespace

namespace somelib::ns {
/**
 * Some example docs
 */
class AttrOpaque1Renamed {
public:

  inline static void test_namespaced_callback(std::function<somelib::diplomat::result<std::monostate, std::monostate>()> _t);

  inline static int32_t mac_test();

  inline static int32_t hello();

  inline uint8_t method_renamed() const;

  inline uint8_t abirenamed() const;

  inline void use_unnamespaced(const somelib::Unnamespaced& _un) const;

  inline void use_namespaced(somelib::ns::RenamedAttrEnum _n) const;

    inline const somelib::ns::capi::AttrOpaque1Renamed* AsFFI() const;
    inline somelib::ns::capi::AttrOpaque1Renamed* AsFFI();
    inline static const somelib::ns::AttrOpaque1Renamed* FromFFI(const somelib::ns::capi::AttrOpaque1Renamed* ptr);
    inline static somelib::ns::AttrOpaque1Renamed* FromFFI(somelib::ns::capi::AttrOpaque1Renamed* ptr);
    inline static void operator delete(void* ptr);
private:
    AttrOpaque1Renamed() = delete;
    AttrOpaque1Renamed(const somelib::ns::AttrOpaque1Renamed&) = delete;
    AttrOpaque1Renamed(somelib::ns::AttrOpaque1Renamed&&) noexcept = delete;
    AttrOpaque1Renamed operator=(const somelib::ns::AttrOpaque1Renamed&) = delete;
    AttrOpaque1Renamed operator=(somelib::ns::AttrOpaque1Renamed&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_ns_AttrOpaque1Renamed_D_HPP
