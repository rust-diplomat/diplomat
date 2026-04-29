#ifndef SOMELIB_ns_RenamedMixinTest_D_HPP
#define SOMELIB_ns_RenamedMixinTest_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"


namespace somelib::ns {
namespace capi {
    struct RenamedMixinTest;


    typedef struct DiplomatRenamedMixinTestView {
      const RenamedMixinTest** data;
      size_t len;
    } DiplomatRenamedMixinTestView;
} // namespace capi
} // namespace

namespace somelib::ns {
class RenamedMixinTest {
public:

  inline static std::string hello();
  template<typename W>
  inline static void hello_write(W& writeable_output);

    inline const somelib::ns::capi::RenamedMixinTest* AsFFI() const;
    inline somelib::ns::capi::RenamedMixinTest* AsFFI();
    inline static const somelib::ns::RenamedMixinTest* FromFFI(const somelib::ns::capi::RenamedMixinTest* ptr);
    inline static somelib::ns::RenamedMixinTest* FromFFI(somelib::ns::capi::RenamedMixinTest* ptr);
    inline static void operator delete(void* ptr);
private:
    RenamedMixinTest() = delete;
    RenamedMixinTest(const somelib::ns::RenamedMixinTest&) = delete;
    RenamedMixinTest(somelib::ns::RenamedMixinTest&&) noexcept = delete;
    RenamedMixinTest operator=(const somelib::ns::RenamedMixinTest&) = delete;
    RenamedMixinTest operator=(somelib::ns::RenamedMixinTest&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_ns_RenamedMixinTest_D_HPP
