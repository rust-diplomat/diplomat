#ifndef SOMELIB_Float64Vec_HPP
#define SOMELIB_Float64Vec_HPP

#include "Float64Vec.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "ErrorEnum.hpp"
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {
    extern "C" {

    somelib::capi::Float64Vec* Float64Vec_new(somelib::diplomat::capi::DiplomatF64View v);

    somelib::capi::Float64Vec* Float64Vec_new_bool(somelib::diplomat::capi::DiplomatBoolView v);

    somelib::capi::Float64Vec* Float64Vec_new_i16(somelib::diplomat::capi::DiplomatI16View v);

    somelib::capi::Float64Vec* Float64Vec_new_u16(somelib::diplomat::capi::DiplomatU16View v);

    somelib::capi::Float64Vec* Float64Vec_new_isize(somelib::diplomat::capi::DiplomatIsizeView v);

    somelib::capi::Float64Vec* Float64Vec_new_usize(somelib::diplomat::capi::DiplomatUsizeView v);

    somelib::capi::Float64Vec* Float64Vec_new_f64_be_bytes(somelib::diplomat::capi::DiplomatU8View v);

    somelib::diplomat::capi::DiplomatF64View Float64Vec_as_slice(const somelib::capi::Float64Vec* self);

    void Float64Vec_fill_slice(const somelib::capi::Float64Vec* self, somelib::diplomat::capi::DiplomatF64ViewMut v);

    void Float64Vec_set_value(somelib::capi::Float64Vec* self, somelib::diplomat::capi::DiplomatF64View new_slice);

    void Float64Vec_to_string(const somelib::capi::Float64Vec* self, somelib::diplomat::capi::DiplomatWrite* write);

    somelib::diplomat::capi::DiplomatF64View Float64Vec_borrow(const somelib::capi::Float64Vec* self);

    typedef struct Float64Vec_get_result {union {double ok; }; bool is_ok;} Float64Vec_get_result;
    Float64Vec_get_result Float64Vec_get(const somelib::capi::Float64Vec* self, size_t i);

    void Float64Vec_write_to(const somelib::capi::Float64Vec* self, somelib::diplomat::capi::DiplomatWrite* write);

    void Float64Vec_write_multi(const somelib::capi::Float64Vec* self, somelib::diplomat::capi::DiplomatWrite* write);

    typedef struct Float64Vec_try_write_to_result {union { somelib::capi::ErrorEnum err;}; bool is_ok;} Float64Vec_try_write_to_result;
    Float64Vec_try_write_to_result Float64Vec_try_write_to(const somelib::capi::Float64Vec* self, bool should_succeed, somelib::diplomat::capi::DiplomatWrite* write);

    typedef struct Float64Vec_maybe_write_to_result { bool is_ok;} Float64Vec_maybe_write_to_result;
    Float64Vec_maybe_write_to_result Float64Vec_maybe_write_to(const somelib::capi::Float64Vec* self, bool should_succeed, somelib::diplomat::capi::DiplomatWrite* write);

    void Float64Vec_destroy(Float64Vec* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<somelib::Float64Vec> somelib::Float64Vec::new_(somelib::diplomat::span<const double> v) {
    auto result = somelib::capi::Float64Vec_new({v.data(), v.size()});
    return std::unique_ptr<somelib::Float64Vec>(somelib::Float64Vec::FromFFI(result));
}

inline std::unique_ptr<somelib::Float64Vec> somelib::Float64Vec::new_bool(somelib::diplomat::span<const bool> v) {
    auto result = somelib::capi::Float64Vec_new_bool({v.data(), v.size()});
    return std::unique_ptr<somelib::Float64Vec>(somelib::Float64Vec::FromFFI(result));
}

inline std::unique_ptr<somelib::Float64Vec> somelib::Float64Vec::new_i16(somelib::diplomat::span<const int16_t> v) {
    auto result = somelib::capi::Float64Vec_new_i16({v.data(), v.size()});
    return std::unique_ptr<somelib::Float64Vec>(somelib::Float64Vec::FromFFI(result));
}

inline std::unique_ptr<somelib::Float64Vec> somelib::Float64Vec::new_u16(somelib::diplomat::span<const uint16_t> v) {
    auto result = somelib::capi::Float64Vec_new_u16({v.data(), v.size()});
    return std::unique_ptr<somelib::Float64Vec>(somelib::Float64Vec::FromFFI(result));
}

inline std::unique_ptr<somelib::Float64Vec> somelib::Float64Vec::new_isize(somelib::diplomat::span<const intptr_t> v) {
    auto result = somelib::capi::Float64Vec_new_isize({v.data(), v.size()});
    return std::unique_ptr<somelib::Float64Vec>(somelib::Float64Vec::FromFFI(result));
}

inline std::unique_ptr<somelib::Float64Vec> somelib::Float64Vec::new_usize(somelib::diplomat::span<const size_t> v) {
    auto result = somelib::capi::Float64Vec_new_usize({v.data(), v.size()});
    return std::unique_ptr<somelib::Float64Vec>(somelib::Float64Vec::FromFFI(result));
}

inline std::unique_ptr<somelib::Float64Vec> somelib::Float64Vec::new_f64_be_bytes(somelib::diplomat::span<const uint8_t> v) {
    auto result = somelib::capi::Float64Vec_new_f64_be_bytes({v.data(), v.size()});
    return std::unique_ptr<somelib::Float64Vec>(somelib::Float64Vec::FromFFI(result));
}

inline somelib::diplomat::span<const double> somelib::Float64Vec::as_slice() const DIPLOMAT_LIFETIME_BOUND {
    auto result = somelib::capi::Float64Vec_as_slice(this->AsFFI());
    return somelib::diplomat::span<const double>(result.data, result.len);
}

inline void somelib::Float64Vec::fill_slice(somelib::diplomat::span<double> v) const {
    somelib::capi::Float64Vec_fill_slice(this->AsFFI(),
        {v.data(), v.size()});
}

inline void somelib::Float64Vec::set_value(somelib::diplomat::span<const double> new_slice) {
    somelib::capi::Float64Vec_set_value(this->AsFFI(),
        {new_slice.data(), new_slice.size()});
}

inline std::string somelib::Float64Vec::to_string() const {
    std::string output;
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteFromString(output);
    somelib::capi::Float64Vec_to_string(this->AsFFI(),
        &write);
    return output;
}
template<typename W>
inline void somelib::Float64Vec::to_string_write(W& writeable) const {
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteTrait<W>::Construct(writeable);
    somelib::capi::Float64Vec_to_string(this->AsFFI(),
        &write);
}

inline somelib::diplomat::span<const double> somelib::Float64Vec::borrow() const DIPLOMAT_LIFETIME_BOUND {
    auto result = somelib::capi::Float64Vec_borrow(this->AsFFI());
    return somelib::diplomat::span<const double>(result.data, result.len);
}

inline std::optional<double> somelib::Float64Vec::operator[](size_t i) const {
    auto result = somelib::capi::Float64Vec_get(this->AsFFI(),
        i);
    return result.is_ok ? std::optional<double>(result.ok) : std::nullopt;
}

inline std::vector<double> somelib::Float64Vec::write_to() const {
    std::vector<double> output;
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteFromVector(output);
    somelib::capi::Float64Vec_write_to(this->AsFFI(),
        &write);
    return output;
}
template<typename W>
inline void somelib::Float64Vec::write_to_write(W& writeable) const {
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteTrait<W>::Construct(writeable);
    somelib::capi::Float64Vec_write_to(this->AsFFI(),
        &write);
}

inline std::vector<double> somelib::Float64Vec::write_multi() const {
    std::vector<double> output;
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteFromVector(output);
    somelib::capi::Float64Vec_write_multi(this->AsFFI(),
        &write);
    return output;
}
template<typename W>
inline void somelib::Float64Vec::write_multi_write(W& writeable) const {
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteTrait<W>::Construct(writeable);
    somelib::capi::Float64Vec_write_multi(this->AsFFI(),
        &write);
}

inline somelib::diplomat::result<std::vector<double>, somelib::ErrorEnum> somelib::Float64Vec::try_write_to(bool should_succeed) const {
    std::vector<double> output;
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteFromVector(output);
    auto result = somelib::capi::Float64Vec_try_write_to(this->AsFFI(),
        should_succeed,
        &write);
    return result.is_ok ? somelib::diplomat::result<std::vector<double>, somelib::ErrorEnum>(somelib::diplomat::Ok<std::vector<double>>(std::move(output))) : somelib::diplomat::result<std::vector<double>, somelib::ErrorEnum>(somelib::diplomat::Err<somelib::ErrorEnum>(somelib::ErrorEnum::FromFFI(result.err)));
}
template<typename W>
inline somelib::diplomat::result<std::monostate, somelib::ErrorEnum> somelib::Float64Vec::try_write_to_write(bool should_succeed, W& writeable) const {
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = somelib::capi::Float64Vec_try_write_to(this->AsFFI(),
        should_succeed,
        &write);
    return result.is_ok ? somelib::diplomat::result<std::monostate, somelib::ErrorEnum>(somelib::diplomat::Ok<std::monostate>()) : somelib::diplomat::result<std::monostate, somelib::ErrorEnum>(somelib::diplomat::Err<somelib::ErrorEnum>(somelib::ErrorEnum::FromFFI(result.err)));
}

inline std::optional<std::vector<double>> somelib::Float64Vec::maybe_write_to(bool should_succeed) const {
    std::vector<double> output;
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteFromVector(output);
    auto result = somelib::capi::Float64Vec_maybe_write_to(this->AsFFI(),
        should_succeed,
        &write);
    return result.is_ok ? std::optional<std::vector<double>>(std::move(output)) : std::nullopt;
}
template<typename W>
inline std::optional<std::monostate> somelib::Float64Vec::maybe_write_to_write(bool should_succeed, W& writeable) const {
    somelib::diplomat::capi::DiplomatWrite write = somelib::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = somelib::capi::Float64Vec_maybe_write_to(this->AsFFI(),
        should_succeed,
        &write);
    return result.is_ok ? std::optional<std::monostate>() : std::nullopt;
}

inline const somelib::capi::Float64Vec* somelib::Float64Vec::AsFFI() const {
    return reinterpret_cast<const somelib::capi::Float64Vec*>(this);
}

inline somelib::capi::Float64Vec* somelib::Float64Vec::AsFFI() {
    return reinterpret_cast<somelib::capi::Float64Vec*>(this);
}

inline const somelib::Float64Vec* somelib::Float64Vec::FromFFI(const somelib::capi::Float64Vec* ptr) {
    return reinterpret_cast<const somelib::Float64Vec*>(ptr);
}

inline somelib::Float64Vec* somelib::Float64Vec::FromFFI(somelib::capi::Float64Vec* ptr) {
    return reinterpret_cast<somelib::Float64Vec*>(ptr);
}

inline void somelib::Float64Vec::operator delete(void* ptr) {
    somelib::capi::Float64Vec_destroy(reinterpret_cast<somelib::capi::Float64Vec*>(ptr));
}


#endif // SOMELIB_Float64Vec_HPP
