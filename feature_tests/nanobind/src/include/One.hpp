#ifndef SOMELIB_One_HPP
#define SOMELIB_One_HPP

#include "One.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "Two.hpp"
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {
    extern "C" {

    somelib::capi::One* One_transitivity(const somelib::capi::One* hold, const somelib::capi::One* nohold);

    somelib::capi::One* One_cycle(const somelib::capi::Two* hold, const somelib::capi::One* nohold);

    somelib::capi::One* One_many_dependents(const somelib::capi::One* a, const somelib::capi::One* b, const somelib::capi::Two* c, const somelib::capi::Two* d, const somelib::capi::Two* nohold);

    somelib::capi::One* One_return_outlives_param(const somelib::capi::Two* hold, const somelib::capi::One* nohold);

    somelib::capi::One* One_diamond_top(const somelib::capi::One* top, const somelib::capi::One* left, const somelib::capi::One* right, const somelib::capi::One* bottom);

    somelib::capi::One* One_diamond_left(const somelib::capi::One* top, const somelib::capi::One* left, const somelib::capi::One* right, const somelib::capi::One* bottom);

    somelib::capi::One* One_diamond_right(const somelib::capi::One* top, const somelib::capi::One* left, const somelib::capi::One* right, const somelib::capi::One* bottom);

    somelib::capi::One* One_diamond_bottom(const somelib::capi::One* top, const somelib::capi::One* left, const somelib::capi::One* right, const somelib::capi::One* bottom);

    somelib::capi::One* One_diamond_and_nested_types(const somelib::capi::One* a, const somelib::capi::One* b, const somelib::capi::One* c, const somelib::capi::One* d, const somelib::capi::One* nohold);

    somelib::capi::One* One_implicit_bounds(const somelib::capi::One* explicit_hold, const somelib::capi::One* implicit_hold, const somelib::capi::One* nohold);

    somelib::capi::One* One_implicit_bounds_deep(const somelib::capi::One* explicit_, const somelib::capi::One* implicit_1, const somelib::capi::One* implicit_2, const somelib::capi::One* nohold);

    void One_destroy(One* self);

    } // extern "C"
} // namespace capi
} // namespace

inline somelib::One somelib::One::transitivity(const somelib::One& hold DIPLOMAT_LIFETIME_BOUND, const somelib::One& nohold) {
    auto result = somelib::capi::One_transitivity(hold.AsFFI(),
        nohold.AsFFI());
    return somelib::One::FromFFI(result);
}

inline somelib::One somelib::One::cycle(const somelib::Two& hold DIPLOMAT_LIFETIME_BOUND, const somelib::One& nohold) {
    auto result = somelib::capi::One_cycle(hold.AsFFI(),
        nohold.AsFFI());
    return somelib::One::FromFFI(result);
}

inline somelib::One somelib::One::many_dependents(const somelib::One& a DIPLOMAT_LIFETIME_BOUND, const somelib::One& b DIPLOMAT_LIFETIME_BOUND, const somelib::Two& c DIPLOMAT_LIFETIME_BOUND, const somelib::Two& d DIPLOMAT_LIFETIME_BOUND, const somelib::Two& nohold) {
    auto result = somelib::capi::One_many_dependents(a.AsFFI(),
        b.AsFFI(),
        c.AsFFI(),
        d.AsFFI(),
        nohold.AsFFI());
    return somelib::One::FromFFI(result);
}

inline somelib::One somelib::One::return_outlives_param(const somelib::Two& hold DIPLOMAT_LIFETIME_BOUND, const somelib::One& nohold) {
    auto result = somelib::capi::One_return_outlives_param(hold.AsFFI(),
        nohold.AsFFI());
    return somelib::One::FromFFI(result);
}

inline somelib::One somelib::One::diamond_top(const somelib::One& top DIPLOMAT_LIFETIME_BOUND, const somelib::One& left DIPLOMAT_LIFETIME_BOUND, const somelib::One& right DIPLOMAT_LIFETIME_BOUND, const somelib::One& bottom DIPLOMAT_LIFETIME_BOUND) {
    auto result = somelib::capi::One_diamond_top(top.AsFFI(),
        left.AsFFI(),
        right.AsFFI(),
        bottom.AsFFI());
    return somelib::One::FromFFI(result);
}

inline somelib::One somelib::One::diamond_left(const somelib::One& top, const somelib::One& left DIPLOMAT_LIFETIME_BOUND, const somelib::One& right, const somelib::One& bottom DIPLOMAT_LIFETIME_BOUND) {
    auto result = somelib::capi::One_diamond_left(top.AsFFI(),
        left.AsFFI(),
        right.AsFFI(),
        bottom.AsFFI());
    return somelib::One::FromFFI(result);
}

inline somelib::One somelib::One::diamond_right(const somelib::One& top, const somelib::One& left, const somelib::One& right DIPLOMAT_LIFETIME_BOUND, const somelib::One& bottom DIPLOMAT_LIFETIME_BOUND) {
    auto result = somelib::capi::One_diamond_right(top.AsFFI(),
        left.AsFFI(),
        right.AsFFI(),
        bottom.AsFFI());
    return somelib::One::FromFFI(result);
}

inline somelib::One somelib::One::diamond_bottom(const somelib::One& top, const somelib::One& left, const somelib::One& right, const somelib::One& bottom DIPLOMAT_LIFETIME_BOUND) {
    auto result = somelib::capi::One_diamond_bottom(top.AsFFI(),
        left.AsFFI(),
        right.AsFFI(),
        bottom.AsFFI());
    return somelib::One::FromFFI(result);
}

inline somelib::One somelib::One::diamond_and_nested_types(const somelib::One& a DIPLOMAT_LIFETIME_BOUND, const somelib::One& b DIPLOMAT_LIFETIME_BOUND, const somelib::One& c DIPLOMAT_LIFETIME_BOUND, const somelib::One& d DIPLOMAT_LIFETIME_BOUND, const somelib::One& nohold) {
    auto result = somelib::capi::One_diamond_and_nested_types(a.AsFFI(),
        b.AsFFI(),
        c.AsFFI(),
        d.AsFFI(),
        nohold.AsFFI());
    return somelib::One::FromFFI(result);
}

inline somelib::One somelib::One::implicit_bounds(const somelib::One& explicit_hold DIPLOMAT_LIFETIME_BOUND, const somelib::One& implicit_hold DIPLOMAT_LIFETIME_BOUND, const somelib::One& nohold) {
    auto result = somelib::capi::One_implicit_bounds(explicit_hold.AsFFI(),
        implicit_hold.AsFFI(),
        nohold.AsFFI());
    return somelib::One::FromFFI(result);
}

inline somelib::One somelib::One::implicit_bounds_deep(const somelib::One& explicit_ DIPLOMAT_LIFETIME_BOUND, const somelib::One& implicit_1 DIPLOMAT_LIFETIME_BOUND, const somelib::One& implicit_2 DIPLOMAT_LIFETIME_BOUND, const somelib::One& nohold) {
    auto result = somelib::capi::One_implicit_bounds_deep(explicit_.AsFFI(),
        implicit_1.AsFFI(),
        implicit_2.AsFFI(),
        nohold.AsFFI());
    return somelib::One::FromFFI(result);
}


#endif // SOMELIB_One_HPP
