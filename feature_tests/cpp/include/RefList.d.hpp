#ifndef SOMELIB_RefList_D_HPP
#define SOMELIB_RefList_D_HPP

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
namespace capi { struct RefList; }
class RefList;
namespace capi { struct RefListParameter; }
class RefListParameter;
} // namespace somelib



namespace somelib {
namespace capi {
    struct RefList;
} // namespace capi
} // namespace

namespace somelib {
class RefList {
public:

  inline static std::unique_ptr<somelib::RefList> node(const somelib::RefListParameter& data);

    inline const somelib::capi::RefList* AsFFI() const;
    inline somelib::capi::RefList* AsFFI();
    inline static const somelib::RefList* FromFFI(const somelib::capi::RefList* ptr);
    inline static somelib::RefList* FromFFI(somelib::capi::RefList* ptr);
    inline static void operator delete(void* ptr);
private:
    RefList() = delete;
    RefList(const somelib::RefList&) = delete;
    RefList(somelib::RefList&&) noexcept = delete;
    RefList operator=(const somelib::RefList&) = delete;
    RefList operator=(somelib::RefList&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_RefList_D_HPP
