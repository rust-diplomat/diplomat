#pragma once

#include <nanobind/nanobind.h>
#include <nanobind/operators.h>
#include <nanobind/stl/unique_ptr.h>
#include <nanobind/stl/string_view.h>
#include <nanobind/stl/string.h>
#include <nanobind/stl/optional.h>
#include <nanobind/stl/function.h>
#include <nanobind/stl/vector.h>
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

    template <typename T, typename E>
	struct type_caster<diplomat::result<T, E>>
	{
		using Value = diplomat::result<T, E>;
		Value value;
		Py_ssize_t size;
		using Caster = make_caster<T>;
		static constexpr auto Name = Caster::Name;

		static handle from_cpp(diplomat::result<T, E> value, rv_policy p, cleanup_list *cl) noexcept
		{
			if (value.is_ok()) {
				return Caster::from_cpp(forward_like_<T>(std::move(value).ok().value()), p, cl);
			}

			auto errorPyV = nb::cast(std::move(std::move(value).err().value()));
			if (errorPyV.is_valid())
			{
				PyErr_SetString(PyExc_Exception, nb::str(errorPyV).c_str());
			}
			else
			{
				char error_msg[512];
				snprintf(error_msg, sizeof(error_msg), "Cannot convert unknown type %s to string for python error.", typeid(E).name());
				PyErr_SetString(PyExc_Exception, error_msg);
			}

            return nullptr;
		}

		NB_INLINE bool can_cast() const noexcept { return Caster::template can_cast<T>(); }
	};

    template <typename T, std::size_t E>
    class type_caster<diplomat::span<T, E>> {
        // The type referenced by the span, with const removed.
        using value_type = std::remove_cv_t<T>;
        // Avoid pitfalls with std::vector<bool>
        using vector_value_type = std::conditional_t<std::is_same_v<bool, value_type>, uint8_t, value_type>;
        using ListCaster = list_caster<std::vector<vector_value_type>, value_type>;
        static_assert(sizeof(bool) == sizeof(uint8_t), "bool representation size is unexpected!");


    public:
        using Value = diplomat::span<T, E>;
        Value value = diplomat::span<T, E>();

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
                    value = diplomat::span<T, E>(caster.value.data(), caster.value.shape(0));
                    return true;
                }
            }

            // Attempt to convert a native sequence. We must convert all elements & store
            // them in a temporary object which will be cleaned up 
            if (std::is_const_v<T> &&
                (!std::is_pointer_v<T> || is_base_caster_v<make_caster<T>>)) {
                ListCaster caster;
                if (caster.from_python(src, local_flags, cleanup)) {
                    value = diplomat::span<T, E>(reinterpret_cast<T*>(caster.value.data()), caster.value.size());
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
        static handle from_cpp(diplomat::span<T, E> src, rv_policy policy, cleanup_list* cleanup) {
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