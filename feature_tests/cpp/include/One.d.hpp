#ifndef SOMELIB_One_D_HPP
#define SOMELIB_One_D_HPP

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
namespace capi { struct One; }
class One;
namespace capi { struct Two; }
class Two;
} // namespace somelib



namespace somelib {
namespace capi {
    struct One;


    typedef struct DiplomatOneView {
      const One** data;
      size_t len;
    } DiplomatOneView;
} // namespace capi
} // namespace

namespace somelib {
class One {
public:

  inline static std::unique_ptr<somelib::One> transitivity(const somelib::One& hold DIPLOMAT_LIFETIME_BOUND, const somelib::One& nohold);

  inline static std::unique_ptr<somelib::One> cycle(const somelib::Two& hold DIPLOMAT_LIFETIME_BOUND, const somelib::One& nohold);

  inline static std::unique_ptr<somelib::One> many_dependents(const somelib::One& a DIPLOMAT_LIFETIME_BOUND, const somelib::One& b DIPLOMAT_LIFETIME_BOUND, const somelib::Two& c DIPLOMAT_LIFETIME_BOUND, const somelib::Two& d DIPLOMAT_LIFETIME_BOUND, const somelib::Two& nohold);

  inline static std::unique_ptr<somelib::One> return_outlives_param(const somelib::Two& hold DIPLOMAT_LIFETIME_BOUND, const somelib::One& nohold);

  inline static std::unique_ptr<somelib::One> diamond_top(const somelib::One& top DIPLOMAT_LIFETIME_BOUND, const somelib::One& left DIPLOMAT_LIFETIME_BOUND, const somelib::One& right DIPLOMAT_LIFETIME_BOUND, const somelib::One& bottom DIPLOMAT_LIFETIME_BOUND);

  inline static std::unique_ptr<somelib::One> diamond_left(const somelib::One& top, const somelib::One& left DIPLOMAT_LIFETIME_BOUND, const somelib::One& right, const somelib::One& bottom DIPLOMAT_LIFETIME_BOUND);

  inline static std::unique_ptr<somelib::One> diamond_right(const somelib::One& top, const somelib::One& left, const somelib::One& right DIPLOMAT_LIFETIME_BOUND, const somelib::One& bottom DIPLOMAT_LIFETIME_BOUND);

  inline static std::unique_ptr<somelib::One> diamond_bottom(const somelib::One& top, const somelib::One& left, const somelib::One& right, const somelib::One& bottom DIPLOMAT_LIFETIME_BOUND);

  inline static std::unique_ptr<somelib::One> diamond_and_nested_types(const somelib::One& a DIPLOMAT_LIFETIME_BOUND, const somelib::One& b DIPLOMAT_LIFETIME_BOUND, const somelib::One& c DIPLOMAT_LIFETIME_BOUND, const somelib::One& d DIPLOMAT_LIFETIME_BOUND, const somelib::One& nohold);

  inline static std::unique_ptr<somelib::One> implicit_bounds(const somelib::One& explicit_hold DIPLOMAT_LIFETIME_BOUND, const somelib::One& implicit_hold DIPLOMAT_LIFETIME_BOUND, const somelib::One& nohold);

  inline static std::unique_ptr<somelib::One> implicit_bounds_deep(const somelib::One& explicit_ DIPLOMAT_LIFETIME_BOUND, const somelib::One& implicit_1 DIPLOMAT_LIFETIME_BOUND, const somelib::One& implicit_2 DIPLOMAT_LIFETIME_BOUND, const somelib::One& nohold);

    inline const somelib::capi::One* AsFFI() const;
    inline somelib::capi::One* AsFFI();
    inline static const somelib::One* FromFFI(const somelib::capi::One* ptr);
    inline static somelib::One* FromFFI(somelib::capi::One* ptr);
    inline static void operator delete(void* ptr);
private:
    One() = delete;
    One(const somelib::One&) = delete;
    One(somelib::One&&) noexcept = delete;
    One operator=(const somelib::One&) = delete;
    One operator=(somelib::One&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_One_D_HPP
