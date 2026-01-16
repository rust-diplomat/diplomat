#ifndef SOMELIB_ns_RenamedStringList_D_HPP
#define SOMELIB_ns_RenamedStringList_D_HPP

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
namespace capi { struct RenamedStringList; }
class RenamedStringList;
} // namespace ns
} // namespace somelib



namespace somelib::ns {
namespace capi {
    struct RenamedStringList;
} // namespace capi
} // namespace

namespace somelib::ns {
/**
 * Testing support for List[str] in Nanobind
 */
class RenamedStringList {
public:

  inline static std::unique_ptr<somelib::ns::RenamedStringList> return_new();

    inline const somelib::ns::capi::RenamedStringList* AsFFI() const;
    inline somelib::ns::capi::RenamedStringList* AsFFI();
    inline static const somelib::ns::RenamedStringList* FromFFI(const somelib::ns::capi::RenamedStringList* ptr);
    inline static somelib::ns::RenamedStringList* FromFFI(somelib::ns::capi::RenamedStringList* ptr);
    inline static void operator delete(void* ptr);
private:
    RenamedStringList() = delete;
    RenamedStringList(const somelib::ns::RenamedStringList&) = delete;
    RenamedStringList(somelib::ns::RenamedStringList&&) noexcept = delete;
    RenamedStringList operator=(const somelib::ns::RenamedStringList&) = delete;
    RenamedStringList operator=(somelib::ns::RenamedStringList&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_ns_RenamedStringList_D_HPP
