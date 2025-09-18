#ifndef ns_RenamedVectorTest_D_HPP
#define ns_RenamedVectorTest_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"

namespace ns {
namespace capi { struct RenamedVectorTest; }
class RenamedVectorTest;
} // namespace ns




namespace ns {
namespace capi {
    struct RenamedVectorTest;
} // namespace capi
} // namespace

namespace ns {
class RenamedVectorTest {
public:

  inline static std::unique_ptr<ns::RenamedVectorTest> new_();

  inline size_t len() const;

  inline std::optional<double> operator[](size_t idx) const;

  inline void push(double value);

    inline const ns::capi::RenamedVectorTest* AsFFI() const;
    inline ns::capi::RenamedVectorTest* AsFFI();
    inline static const ns::RenamedVectorTest* FromFFI(const ns::capi::RenamedVectorTest* ptr);
    inline static ns::RenamedVectorTest* FromFFI(ns::capi::RenamedVectorTest* ptr);
    inline static void operator delete(void* ptr);
private:
    RenamedVectorTest() = delete;
    RenamedVectorTest(const ns::RenamedVectorTest&) = delete;
    RenamedVectorTest(ns::RenamedVectorTest&&) noexcept = delete;
    RenamedVectorTest operator=(const ns::RenamedVectorTest&) = delete;
    RenamedVectorTest operator=(ns::RenamedVectorTest&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // ns_RenamedVectorTest_D_HPP
