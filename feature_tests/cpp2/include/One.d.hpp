#ifndef One_D_HPP
#define One_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "One.d.h"
class Two;


class One {
public:

  inline static std::unique_ptr<One> transitivity(const One& hold, const One& nohold);

  inline static std::unique_ptr<One> cycle(const Two& hold, const One& nohold);

  inline static std::unique_ptr<One> many_dependents(const One& a, const One& b, const Two& c, const Two& d, const Two& nohold);

  inline static std::unique_ptr<One> return_outlives_param(const Two& hold, const One& nohold);

  inline static std::unique_ptr<One> diamond_top(const One& top, const One& left, const One& right, const One& bottom);

  inline static std::unique_ptr<One> diamond_left(const One& top, const One& left, const One& right, const One& bottom);

  inline static std::unique_ptr<One> diamond_right(const One& top, const One& left, const One& right, const One& bottom);

  inline static std::unique_ptr<One> diamond_bottom(const One& top, const One& left, const One& right, const One& bottom);

  inline static std::unique_ptr<One> diamond_and_nested_types(const One& a, const One& b, const One& c, const One& d, const One& nohold);

  inline static std::unique_ptr<One> implicit_bounds(const One& explicit_hold, const One& implicit_hold, const One& nohold);

  inline static std::unique_ptr<One> implicit_bounds_deep(const One& explicit_, const One& implicit_1, const One& implicit_2, const One& nohold);

  inline const capi::One* AsFFI() const;
  inline capi::One* AsFFI();
  inline static const One* FromFFI(const capi::One* ptr);
  inline static One* FromFFI(capi::One* ptr);
  inline ~One();
private:
  One() = delete;
};


#endif // One_D_HPP
