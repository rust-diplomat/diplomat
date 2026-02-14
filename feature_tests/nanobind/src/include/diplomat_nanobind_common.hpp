#pragma once

#include <nanobind/nanobind.h>
#include <nanobind/operators.h>
#include <nanobind/stl/unique_ptr.h>
#include <nanobind/stl/string_view.h>
#include <nanobind/stl/string.h>
#include <nanobind/stl/optional.h>
#include <nanobind/stl/function.h>
#include <nanobind/stl/vector.h>
#include <nanobind/stl/bind_vector.h>
#include <nanobind/stl/detail/nb_list.h>
#include <nanobind/ndarray.h>
#include <../src/nb_internals.h>  // Required for shimming
#include "diplomat_runtime.hpp"

namespace nb = nanobind;
using namespace nb::literals;

namespace nanobind::detail
{
    // Nanobind does not ship with support for casting char32_t, which seems to be an oversight.
    // Remove this block when upstream support is added
    template <>
    struct type_caster<char32_t>
    {
        using Value = char32_t;
        Value value;
        Py_ssize_t size;
        static constexpr auto Name = const_name("str");
        template <typename T>
        using Cast = char32_t;

        bool from_python(handle src, uint8_t, cleanup_list *) noexcept
        {
            value = PyUnicode_ReadChar(src.ptr(), 0);
            if (!value)
            {
                PyErr_Clear();
                return false;
            }
            size = PyUnicode_GetLength(src.ptr());
            return true;
        }

        static handle from_cpp(const char32_t *value, rv_policy,
                               cleanup_list *) noexcept
        {
            if (value == nullptr)
            {
                PyObject *result = Py_None;
                Py_INCREF(result);
                return result;
            }
            size_t len = 0;
            const char32_t *str = value;
            while (*str != U'\0')
            {
                len++;
                str++;
            }
            return PyUnicode_DecodeUTF32(reinterpret_cast<const char *>(value), len * 4, nullptr, nullptr);
        }

        static handle from_cpp(char32_t value, rv_policy, cleanup_list *) noexcept
        {
            return PyUnicode_DecodeUTF32(reinterpret_cast<const char *>(&value), 4, nullptr, nullptr);
        }

        template <typename T_>
        NB_INLINE bool can_cast() const noexcept
        {
            return (value && size == 1);
        }

        explicit operator char32_t()
        {
            if (can_cast<char32_t>())
                return value;
            else
                throw next_overload();
        }
    };

    template <typename T>
    struct type_caster<std::reference_wrapper<T>>
    {
        using Value = std::reference_wrapper<T>;
        Value value;
        Py_ssize_t size;
        using Caster = make_caster<T>;
        static constexpr auto Name = Caster::Name;

        static handle from_cpp(std::reference_wrapper<T> value, rv_policy p, cleanup_list *cl) noexcept
        {
            return Caster::from_cpp(value.get(), p, cl);
        }

        NB_INLINE bool can_cast() const noexcept { return Caster::template can_cast<T>(); }
    };

    template <typename T, typename E>
    struct type_caster<somelib::diplomat::result<T, E>>
    {
        using U = std::conditional_t<std::is_reference_v<T>, std::reference_wrapper<std::remove_reference_t<T>>, T>;
        using V = std::conditional_t<std::is_reference_v<E>, std::reference_wrapper<std::remove_reference_t<E>>, E>;
        using Value = somelib::diplomat::result<T, E>;
        // Can't store result<T, E> directly since T& will create compiler errors.
        std::optional<U> ok_val;
        std::optional<V> err_val;
        bool is_ok;
        Py_ssize_t size;
        using Caster = make_caster<U>;
        using ErrCaster = make_caster<V>;
        static constexpr auto Name = const_name("result");

        static handle from_cpp(somelib::diplomat::result<T, E> value, rv_policy p, cleanup_list *cl) noexcept
        {
            if (value.is_ok()) {
                return Caster::from_cpp(forward_like_<U>(std::move(value).ok().value()), p, cl);
            }

            auto errorPyV = ErrCaster::from_cpp(forward_like_<V>(std::move(value).err().value()), p, cl);
            if (errorPyV.is_valid())
            {
                PyErr_SetObject(PyExc_Exception, errorPyV.ptr());
                // PyErr_SetObject takes ownership (https://github.com/python/cpython/blob/fa73fd473f00dd231f59e44798a3d00a46322658/Python/errors.c#L151)
                // but Nanobind expects Python to take ownership directly. So we decref after PyErr_SetObject takes ownership, to remove Nanobind's reference:
                Py_DECREF(errorPyV.ptr());
            }
            else
            {
                char error_msg[512];
                snprintf(error_msg, sizeof(error_msg), "Cannot convert unknown type %s to python exception.", typeid(E).name());
                PyErr_SetString(PyExc_Exception, error_msg);
            }

            return nullptr;
        }
        
        template <typename T_>
        using Cast = Value;
        operator Value() { 
            if (is_ok) {
                return somelib::diplomat::Ok<T>(forward_like_<U>(ok_val.value()));
            } else {
                return somelib::diplomat::Err<E>(forward_like_<V>(err_val.value()));
            }
        }

        bool from_python(handle src, uint8_t flags, cleanup_list* cleanup) noexcept  {
            uint8_t local_flags = flags_for_local_caster<U>(flags);

            // We raise an exception above, but I think it's okay just to check if our conversion succeeds:
            auto caster = make_caster<T>();
            if (caster.from_python(src, local_flags, cleanup)) {
                is_ok = true;
                if constexpr(std::is_reference_v<T>) {
                    ok_val = std::optional(std::reference_wrapper(caster.operator cast_t<T>()));
                } else {
                    ok_val = std::optional(caster.operator cast_t<T>());
                }
                return true;
            } else {
                auto err_caster = make_caster<E>();
                uint8_t err_local_flags = flags_for_local_caster<E>(flags);
                if (err_caster.from_python(src, err_local_flags, cleanup)) {
                    is_ok = false;
                    if constexpr(std::is_reference_v<E>) {
                        err_val = std::optional(std::reference_wrapper(err_caster.operator cast_t<E>()));
                    } else {
                        err_val = std::optional(err_caster.operator cast_t<E>());
                    }
                    return true;
                }
            }

            return false;
        }

        NB_INLINE bool can_cast() const noexcept { return Caster::template can_cast<U>(); }
    };

    template <typename T, std::size_t E>
    class type_caster<somelib::diplomat::span<T, E>> {
        // The type referenced by the span, with const removed.
        using value_type = std::remove_cv_t<T>;
        // Avoid pitfalls with std::vector<bool>
        using vector_value_type = std::conditional_t<std::is_same_v<bool, value_type>, uint8_t, value_type>;
        using ListCaster = list_caster<std::vector<vector_value_type>, value_type>;
        static_assert(sizeof(bool) == sizeof(uint8_t), "bool representation size is unexpected!");


    public:
        using Value = somelib::diplomat::span<T, E>;
        Value value = somelib::diplomat::span<T, E>();

        static constexpr auto Name = ListCaster::Name;

        template <typename T_>
        using Cast = Value;
        operator Value() { return value; }

        template <typename T_> static constexpr bool can_cast() { return true; }

        // Cast Python -> C++ (nb::cast call)
        bool from_python(handle src, uint8_t flags, cleanup_list* cleanup) noexcept {
            uint8_t local_flags = flags_for_local_caster<T>(flags);

            // First try to convert from ndarray for efficiency
            // Try to get a 1D contiguous array directly using type tags
            if constexpr (is_ndarray_scalar_v<T>) {
                auto caster = make_caster<nb::ndarray<T, ndim<1>>>();
                if (caster.from_python(src, local_flags, cleanup)) {
                    // Create a span from the array data
                    value = somelib::diplomat::span<T, E>(caster.value.data(), caster.value.shape(0));
                    return true;
                }
            }

            // C++ std::vector<bool> is not allowed, so we convert it to std::vector<uint8_t> for the compiler's sake.
            using U = std::conditional_t<!std::is_same_v<std::remove_cv_t<T>, bool>, std::vector<std::remove_cv_t<T>>, std::vector<uint8_t>>;


            // Are we a bound vector type? If so, we can pass over data directly.
            if (nb::inst_check(src) && nb::isinstance<U>(src)) {
                U* bound_vec = nb::inst_ptr<U>(src);
                value = somelib::diplomat::span<T, E>(reinterpret_cast<T*>(bound_vec->data()), bound_vec->size());
                return true;
            }

            // Attempt to convert a native sequence. We must convert all elements & store
            // them in a temporary object which will be cleaned up 
            if (std::is_const_v<T> &&
                (!std::is_pointer_v<T> || is_base_caster_v<make_caster<T>>)) {
                ListCaster caster;
                if (caster.from_python(src, local_flags, cleanup)) {
                    value = somelib::diplomat::span<T, E>(reinterpret_cast<T*>(caster.value.data()), caster.value.size());
                    // Move the owning std::vector into a capsule that will live for the duration of the function call.
                    // The address of the vector will change, the address of the region it references won't.
                    nb::capsule deleter(new std::vector<vector_value_type>(std::move(caster.value)), [](void* data) noexcept {
                        delete (std::vector<vector_value_type>*)data;
                        });
                    cleanup->append(deleter.release().ptr());

                    return true;
                }
            }

            return false; // Python type cannot be loaded into a span.
        }

        // Cast C++ -> Python (when returning a span from a C++ function)
        static handle from_cpp(somelib::diplomat::span<T, E> src, rv_policy policy, cleanup_list* cleanup) {
            using Array = nb::ndarray<std::remove_cv_t<T>, nb::numpy, nb::ndim<1>, nb::f_contig>;
            if constexpr(is_ndarray_scalar_v<T>) {
                nb::object owner;
                if (cleanup->self()) {
                    owner = nb::borrow(cleanup->self());
                    policy = rv_policy::reference;
                }

                 object o = steal(type_caster<Array>::from_cpp(
                    Array((void* )src.data(), {src.size()}, owner),
                    policy, cleanup));

                return o.release();
            } else {
                return ListCaster::from_cpp(src, policy, cleanup);
            }
        }
    };

    template <>
    struct type_caster<somelib::diplomat::string_view_for_slice> {
        NB_TYPE_CASTER(somelib::diplomat::string_view_for_slice, const_name("str"))

        bool from_python(handle src, uint8_t, cleanup_list *) noexcept {
            Py_ssize_t size;
            const char *str = PyUnicode_AsUTF8AndSize(src.ptr(), &size);
            if (!str) {
                PyErr_Clear();
                return false;
            }
            value = somelib::diplomat::string_view_for_slice(str, (size_t) size);
            return true;
        }

        static handle from_cpp(somelib::diplomat::string_view_for_slice value, rv_policy, cleanup_list *) noexcept {
            return PyUnicode_FromStringAndSize(value.data(), value.size());
        }
    };
}

// Return the inner type from next()
// Next returns either a std::unique_ptr or std::optional.
// When T is optional, return inner<T>&&. When T is unique_ptr, just return it.
template<typename T>
struct next_inner_extractor {
    static T&& get(T&& v) { return std::move(v); }
};

template<typename T>
struct next_inner_extractor<std::optional<T>> {
    static T&& get(std::optional<T>&& v) { return std::move(v).value(); }
};

// These are defined in the root module.cpp file.
extern void (*nb_tp_dealloc)(void *);
void diplomat_tp_dealloc(PyObject *self);

// Templating for handling Opaque ZSTs.
// Nanobind cannot handle pointers of Rust ZSTs if they're wrapped behind a std::unique_ptr.
// By Rust's own standard, pointers to ZSTs are always the same address.
// Nanobind always assumes that any unique_ptr it sees contains an address that is *never* repeated.
// So Nanobind handles the first conversion from std::unique_ptr->Python just fine,
// but if a function returns a std::unique_ptr of the same address as one it has seen before, then Nanobind panics.
//
// The solution then (without modifying the rest of Diplomat to provide information on what is/isn't an opaque ZST)
// is to hijack functions which return std::unique_ptr and check if they have ZSTs. This chain starts with maybe_op_unwrap 
// and ends with maybe_alloc_zst.
template<typename Return>
inline std::unique_ptr<Return> maybe_alloc_zst(std::unique_ptr<Return> pointer) {
    // Are we at address 1? Then we're a ZST.
    // Per the Rust allocator (https://github.com/rust-lang/rust/blob/18d13b5332916ffca8eadb9106d54b5b434e9978/library/alloc/src/alloc.rs#L187)
    if ((void*)pointer.get() == (void*)0x1) {
        // We don't need to free or drop the pointer, the pointer we allocate below serves the same purpose.
        pointer.release();
        // We are a ZST, so C++ expects a 0-sized malloc (guaranteed to be a unique pointer):
        return std::unique_ptr<Return>((Return*)malloc(0));
    } else {
        return pointer;
    }
}


// Helper for determining if we have a unique_ptr type.
template <class T>
struct get_unique_ptr : std::false_type {};

template <class T>
struct get_unique_ptr<std::unique_ptr<T>> : std::true_type{};

// Given a Return type, map that inner type with maybe_alloc_zst (by unwrapping and re-wrapping the Return type).
// Default implementation for plain unique_ptr types.
template<typename Return>
inline typename std::enable_if_t<get_unique_ptr<Return>::value, Return>
map_inner(Return to_wrap) {
    return maybe_alloc_zst(std::move(to_wrap));
}

template <class Ty>
struct get_diplomat_result : std::false_type {};

template <class T, class E>
struct get_diplomat_result<somelib::diplomat::result<T, E>> : std::true_type {
    typedef somelib::diplomat::Ok<T> ok;
    typedef somelib::diplomat::Err<E> err;

    typedef T success; 
    typedef E error;
};

template<typename Return, typename... Args>
inline typename std::enable_if_t<get_diplomat_result<Return>::value, Return>
map_inner(Return to_wrap) {
    using Result = get_diplomat_result<Return>;
    // Either the success type, the error type, or both are unique_ptrs.
    if (to_wrap.is_ok()) {
        if constexpr(get_unique_ptr<typename Result::success>::value) {
            return typename Result::ok(maybe_alloc_zst(std::move(to_wrap).ok().value()));
        } else {
            return to_wrap;
        }
    } else {
        if constexpr(get_unique_ptr<typename Result::error>::value) {
            return typename Result::err(maybe_alloc_zst(std::move(to_wrap).err().value()));
        } else {
            return to_wrap;
        }
    }
}

// Helper for taking a function of a signature and returning a new function which 
// potentially modifies the returned type (if it's a ZST opaque).
template<typename Func>
struct maybe_op_unwrapper {};

// Static function:
template<typename Return, typename... Args>
struct maybe_op_unwrapper<Return(*)(Args...)> {
    typedef std::function<Return(Args...)> result;

    // Get the output of the old function. Call map_inner to unwrap and then re-wrap the inner unique_ptrs.
    static Return bound_map(Return (*f)(Args...), Args... args) {
        auto out = f(args...);
        return map_inner<Return>(std::move(out));
    }

    // Return a std::function for nanobind to parse. This new function will be bound with the old function to call.
    static std::function<Return(Args...)> get_bound_mapper(Return (*f)(Args...)) {
        std::function<Return(Args...)> out = std::bind_front(bound_map, f);
        return out;
    }
};

// Class member function:
template<typename Return, class Class, typename... Args>
struct maybe_op_unwrapper<Return(Class::*)(Args...)> {
    typedef std::function<Return(Class*, Args...)> result;

    static Return bound_map(Return (Class::*f)(Args...), Class* c, Args... args) {
        auto out = (c->*f)(args...);
        return map_inner<Return>(std::move(out));
    }
    
    static std::function<Return(Class*, Args...)> get_bound_mapper(Return (Class::*f)(Args...)) {
        std::function<Return(Class*, Args...)> out = std::bind_front(maybe_op_unwrapper<Return(Class::*)(Args...)>::bound_map, f);
        return out;
    }
};

// Const member function:
template<typename Return, class Class, typename... Args>
struct maybe_op_unwrapper<Return(Class::*)(Args...) const> {
    typedef std::function<Return(Class*, Args...)> result;

    static Return bound_map(Return (Class::*f)(Args...) const, const Class* c, Args... args) {
        auto out = (c->*f)(args...);
        return map_inner<Return>(std::move(out));
    }
    
    static std::function<Return(Class*, Args...)> get_bound_mapper(Return (Class::*f)(Args...) const) {
        std::function<Return(Class*, Args...)> out = std::bind_front(maybe_op_unwrapper<Return(Class::*)(Args...) const>::bound_map, f);
        return out;
    }
};

// Given a function that is guaranteed to at some point return a std::unique_ptr (either bare, through optional, or through result).
// Returns a new function that unwraps and then re-wraps that opaque, applying a filter to reallocate any non-unique opaque ZST pointers.
// See maybe_alloc_zst for explanation as to why we call this function.
// Check to see if we're unwrapping a lambda first:
template<typename Func>
typename std::enable_if_t<nanobind::detail::is_lambda_v<std::remove_reference_t<Func>>, typename maybe_op_unwrapper<decltype(&Func::operator())>::result> maybe_op_unwrap(Func&& f) {
    return maybe_op_unwrapper<decltype(&Func::operator())>::get_bound_mapper((nanobind::detail::forward_t<decltype(&Func::operator())>)f);
}

// Then the general case:
template<typename Func>
maybe_op_unwrapper<Func>::result maybe_op_unwrap(Func&& f) {
    return maybe_op_unwrapper<Func>::get_bound_mapper((nanobind::detail::forward_t<Func>)f);
}