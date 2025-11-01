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
} // namespace capi
} // namespace

namespace somelib::ns {
class RenamedVectorTest {
public:

  inline static std::unique_ptr<somelib::ns::RenamedVectorTest> new_();

  inline size_t len() const;

  inline std::optional<double> operator[](size_t idx) const;

  inline void push(double value);

    inline const somelib::ns::capi::RenamedVectorTest* AsFFI() const;
    inline somelib::ns::capi::RenamedVectorTest* AsFFI();
    inline static const somelib::ns::RenamedVectorTest* FromFFI(const somelib::ns::capi::RenamedVectorTest* ptr);
    inline static somelib::ns::RenamedVectorTest* FromFFI(somelib::ns::capi::RenamedVectorTest* ptr);
    inline static void operator delete(void* ptr);
private:
    RenamedVectorTest() = delete;
    RenamedVectorTest(const somelib::ns::RenamedVectorTest&) = delete;
    RenamedVectorTest(somelib::ns::RenamedVectorTest&&) noexcept = delete;
    RenamedVectorTest operator=(const somelib::ns::RenamedVectorTest&) = delete;
    RenamedVectorTest operator=(somelib::ns::RenamedVectorTest&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_ns_RenamedVectorTest_D_HPP
