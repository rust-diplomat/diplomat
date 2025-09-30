#ifndef SOMELIB_OpaqueThinVec_D_HPP
#define SOMELIB_OpaqueThinVec_D_HPP

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
namespace capi { struct OpaqueThin; }
class OpaqueThin;
namespace capi { struct OpaqueThinIter; }
class OpaqueThinIter;
namespace capi { struct OpaqueThinVec; }
class OpaqueThinVec;
} // namespace somelib



namespace somelib {
namespace capi {
    struct OpaqueThinVec;
} // namespace capi
} // namespace

namespace somelib {
class OpaqueThinVec {
public:

  inline static std::unique_ptr<somelib::OpaqueThinVec> create(somelib::diplomat::span<const int32_t> a, somelib::diplomat::span<const float> b, std::string_view c);

  inline std::unique_ptr<somelib::OpaqueThinIter> iter() const;
  inline somelib::diplomat::next_to_iter_helper<somelib::OpaqueThinIter> begin() const;
  inline std::nullopt_t end() const { return std::nullopt; }

  inline size_t __len__() const;

  inline const somelib::OpaqueThin* operator[](size_t idx) const;

  inline const somelib::OpaqueThin* first() const;

    inline const somelib::capi::OpaqueThinVec* AsFFI() const;
    inline somelib::capi::OpaqueThinVec* AsFFI();
    inline static const somelib::OpaqueThinVec* FromFFI(const somelib::capi::OpaqueThinVec* ptr);
    inline static somelib::OpaqueThinVec* FromFFI(somelib::capi::OpaqueThinVec* ptr);
    inline static void operator delete(void* ptr);
private:
    OpaqueThinVec() = delete;
    OpaqueThinVec(const somelib::OpaqueThinVec&) = delete;
    OpaqueThinVec(somelib::OpaqueThinVec&&) noexcept = delete;
    OpaqueThinVec operator=(const somelib::OpaqueThinVec&) = delete;
    OpaqueThinVec operator=(somelib::OpaqueThinVec&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_OpaqueThinVec_D_HPP
