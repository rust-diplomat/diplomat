#ifndef OpaqueThinVec_D_HPP
#define OpaqueThinVec_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "diplomat_runtime.hpp"

namespace diplomat::capi { struct OpaqueThin; }
class OpaqueThin;
namespace diplomat::capi { struct OpaqueThinIter; }
class OpaqueThinIter;


namespace diplomat {
namespace capi {
    struct OpaqueThinVec;

} // namespace capi
} // namespace

class OpaqueThinVec {
public:

  inline static std::unique_ptr<OpaqueThinVec> create(diplomat::span<const int32_t> a, diplomat::span<const float> b);

  inline std::unique_ptr<OpaqueThinIter> iter() const;
  inline diplomat::next_to_iter_helper<OpaqueThinIter> begin() const;
  inline std::nullopt_t end() const { return std::nullopt; }

  inline size_t __len__() const;

  inline const OpaqueThin* operator[](size_t idx) const;

  inline const OpaqueThin* first() const;

  inline const diplomat::capi::OpaqueThinVec* AsFFI() const;
  inline diplomat::capi::OpaqueThinVec* AsFFI();
  inline static const OpaqueThinVec* FromFFI(const diplomat::capi::OpaqueThinVec* ptr);
  inline static OpaqueThinVec* FromFFI(diplomat::capi::OpaqueThinVec* ptr);
  inline static void operator delete(void* ptr);
private:
  OpaqueThinVec() = delete;
  OpaqueThinVec(const OpaqueThinVec&) = delete;
  OpaqueThinVec(OpaqueThinVec&&) noexcept = delete;
  OpaqueThinVec operator=(const OpaqueThinVec&) = delete;
  OpaqueThinVec operator=(OpaqueThinVec&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // OpaqueThinVec_D_HPP
