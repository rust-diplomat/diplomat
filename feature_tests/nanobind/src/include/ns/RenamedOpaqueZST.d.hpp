#ifndef SOMELIB_ns_RenamedOpaqueZST_D_HPP
#define SOMELIB_ns_RenamedOpaqueZST_D_HPP

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
namespace capi { struct RenamedOpaqueZST; }
class RenamedOpaqueZST;
namespace capi { struct RenamedOpaqueZSTIterator; }
class RenamedOpaqueZSTIterator;
} // namespace ns
} // namespace somelib



namespace somelib::ns {
namespace capi {
    struct RenamedOpaqueZST;
    extern "C" {
    void namespace_OpaqueZST_destroy(RenamedOpaqueZST* self);
    }
} // namespace capi
} // namespace

namespace somelib::ns {
class RenamedOpaqueZST;
using RenamedOpaqueZSTRef = somelib::diplomat::Ref<RenamedOpaqueZST, const somelib::ns::capi::RenamedOpaqueZST>;
using RenamedOpaqueZSTRefMut = somelib::diplomat::Ref<RenamedOpaqueZST, somelib::ns::capi::RenamedOpaqueZST>;

/**
 * Tests for https://github.com/rust-diplomat/diplomat/issues/1050.
 * C++ generates unique_ptrs for Opaque ZSTs, and Nanobind
 * expects every unique_ptr it converts to wrap a unique pointer type. It errors otherwise.
 * This is not the case, as in Rust pointers to ZSTs are always the same address.
 */
class RenamedOpaqueZST : public somelib::diplomat::OpaquePointer<RenamedOpaqueZST, somelib::ns::capi::RenamedOpaqueZST, somelib::ns::capi::namespace_OpaqueZST_destroy> {
public:

  inline static somelib::ns::RenamedOpaqueZST ctor();

  inline static somelib::ns::RenamedOpaqueZST make();

  inline static std::string out_string();
  template<typename W>
  inline static void out_string_write(W& writeable_output);

  inline somelib::ns::RenamedOpaqueZST member() const;

  inline somelib::ns::RenamedOpaqueZST mut_member();

  inline somelib::ns::RenamedOpaqueZST operator+(const somelib::ns::RenamedOpaqueZST& _o) const;

  inline somelib::ns::RenamedOpaqueZST operator-(const somelib::ns::RenamedOpaqueZST& _o) const;

  inline somelib::ns::RenamedOpaqueZST operator*(const somelib::ns::RenamedOpaqueZST& _o) const;

  inline somelib::ns::RenamedOpaqueZST operator/(const somelib::ns::RenamedOpaqueZST& _o) const;

  inline static somelib::diplomat::result<somelib::ns::RenamedOpaqueZST, std::monostate> success_zst(bool return_success);

  inline static somelib::diplomat::result<std::monostate, somelib::ns::RenamedOpaqueZST> fail_zst(bool return_success);

  inline static somelib::diplomat::result<somelib::ns::RenamedOpaqueZST, somelib::ns::RenamedOpaqueZST> success_fail_zst(bool return_success);

  inline static somelib::diplomat::Optional<somelib::ns::RenamedOpaqueZST> optional_zst(bool is_some);

  inline static somelib::ns::RenamedOpaqueZST static_getter();

  inline static void static_setter(const somelib::ns::RenamedOpaqueZST& _a);

  inline somelib::ns::RenamedOpaqueZST getter() const;

  inline void setter(const somelib::ns::RenamedOpaqueZST& _a) const;

  inline somelib::ns::RenamedOpaqueZSTIterator iter() const;
  inline somelib::diplomat::next_to_iter_helper<somelib::ns::RenamedOpaqueZSTIterator> begin() const;
  inline std::nullopt_t end() const { return std::nullopt; }

  inline somelib::diplomat::Optional<somelib::ns::RenamedOpaqueZST> operator[](size_t _idx) const;

};

} // namespace
#endif // SOMELIB_ns_RenamedOpaqueZST_D_HPP
