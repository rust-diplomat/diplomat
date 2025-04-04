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
#include "Bar.hpp"
#include "BorrowedFields.hpp"
#include "BorrowedFieldsReturning.hpp"
#include "BorrowedFieldsWithBounds.hpp"
#include "CallbackTestingStruct.hpp"
#include "CallbackWrapper.hpp"
#include "ContiguousEnum.hpp"
#include "CyclicStructA.hpp"
#include "CyclicStructB.hpp"
#include "CyclicStructC.hpp"
#include "DefaultEnum.hpp"
#include "ErrorEnum.hpp"
#include "ErrorStruct.hpp"
#include "Float64Vec.hpp"
#include "Foo.hpp"
#include "ImportedStruct.hpp"
#include "MyEnum.hpp"
#include "MyOpaqueEnum.hpp"
#include "MyString.hpp"
#include "MyStruct.hpp"
#include "MyStructContainingAnOption.hpp"
#include "MyZst.hpp"
#include "NestedBorrowedFields.hpp"
#include "One.hpp"
#include "Opaque.hpp"
#include "OpaqueMutexedString.hpp"
#include "OpaqueThin.hpp"
#include "OpaqueThinIter.hpp"
#include "OpaqueThinVec.hpp"
#include "OptionEnum.hpp"
#include "OptionInputStruct.hpp"
#include "OptionOpaque.hpp"
#include "OptionOpaqueChar.hpp"
#include "OptionString.hpp"
#include "OptionStruct.hpp"
#include "RefList.hpp"
#include "RefListParameter.hpp"
#include "ResultOpaque.hpp"
#include "StructArithmetic.hpp"
#include "Two.hpp"
#include "UnimportedEnum.hpp"
#include "Unnamespaced.hpp"
#include "Utf16Wrap.hpp"
#include "nested/ns/Nested.hpp"
#include "nested/ns2/Nested.hpp"
#include "ns/AttrOpaque1Renamed.hpp"
#include "ns/RenamedAttrEnum.hpp"
#include "ns/RenamedAttrOpaque2.hpp"
#include "ns/RenamedComparable.hpp"
#include "ns/RenamedMyIndexer.hpp"
#include "ns/RenamedMyIterable.hpp"
#include "ns/RenamedMyIterator.hpp"
#include "ns/RenamedOpaqueArithmetic.hpp"
#include "ns/RenamedOpaqueIterable.hpp"
#include "ns/RenamedOpaqueIterator.hpp"
#include "ns/RenamedStructWithAttrs.hpp"

namespace nb = nanobind;
using namespace nb::literals;

// Nanobind does not usually support custom deleters, so we're shimming some of the machinery to add that ability.
// On module init, the dummy type will have the normal nanobind inst_dealloc function in the tp_dealloc slot, so we
// pull it out, store it here, and then call it in the tp_dealloc function we are shimming in to all our types.
// Our custom tp_dealloc function will call the tp_free function instead of `delete`, allowing us effectively to override
// the delete operator.
// See https://nanobind.readthedocs.io/en/latest/lowlevel.html#customizing-type-creation and
// https://github.com/wjakob/nanobind/discussions/932
static void (*nb_tp_dealloc)(void *) = nullptr;

void diplomat_tp_dealloc(PyObject *self)
{
    using namespace nb::detail;
    PyTypeObject *tp = Py_TYPE(self);
    const type_data *t = nb_type_data(tp);

    nb_inst *inst = (nb_inst *)self;
    void *p = inst_ptr(inst);
    if (inst->destruct)
    {
        inst->destruct = false;
        check(t->flags & (uint32_t)type_flags::is_destructible,
              "nanobind::detail::inst_dealloc(\"%s\"): attempted to call "
              "the destructor of a non-destructible type!",
              t->name);
        if (t->flags & (uint32_t)type_flags::has_destruct)
            t->destruct(p);
    }
    if (inst->cpp_delete)
    {
        inst->cpp_delete = false;
        auto tp_free = (freefunc)(PyType_GetSlot(tp, Py_tp_free));
        (*tp_free)(p);
    }
    (*nb_tp_dealloc)(self);
}

struct _Dummy {};

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
            return ListCaster::from_cpp(src, policy, cleanup);
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

NB_MODULE(somelib, somelib_mod)
{
	{
		nb::class_<_Dummy> dummy(somelib_mod, "__dummy__");
		nb_tp_dealloc = (void (*)(void *))nb::type_get_slot(dummy, Py_tp_dealloc);
	}

    nb::class_<std::monostate>(somelib_mod, "monostate")
		.def("__repr__", [](const std::monostate &)
			 { return ""; })
		.def("__str__", [](const std::monostate &)
			 { return ""; });
    
    nb::module_ ns_mod = somelib_mod.def_submodule("ns");
    
    nb::class_<ns::RenamedStructWithAttrs>(ns_mod, "RenamedStructWithAttrs")
        .def_rw("a", &ns::RenamedStructWithAttrs::a)
        .def_rw("b", &ns::RenamedStructWithAttrs::b)
    	.def_prop_ro("c", &ns::RenamedStructWithAttrs::c)
    	.def("__init__", [](ns::RenamedStructWithAttrs* self, bool a, uint32_t b){ auto tmp = ns::RenamedStructWithAttrs::new_fallible(a, b);
    				if(tmp.is_ok()) {
    					*self = std::move(tmp).ok().value();
    				} else {
    					nb::cast(tmp); // This will raise a python error with the contents of the error type
    				}}, "a"_a, "b"_a);
    
    nb::class_<CallbackTestingStruct>(somelib_mod, "CallbackTestingStruct")
        .def(nb::init<>())
        .def(nb::init<int32_t, int32_t>(), "x"_a.none(),  "y"_a.none())
        .def_rw("x", &CallbackTestingStruct::x)
        .def_rw("y", &CallbackTestingStruct::y);
    
    nb::class_<CallbackWrapper>(somelib_mod, "CallbackWrapper")
        .def(nb::init<>())
        .def(nb::init<bool>(), "cant_be_empty"_a.none())
        .def_rw("cant_be_empty", &CallbackWrapper::cant_be_empty)
    	.def_static("test_cb_with_struct", &CallbackWrapper::test_cb_with_struct, "f"_a)
    	.def_static("test_multi_arg_callback", &CallbackWrapper::test_multi_arg_callback, "f"_a, "x"_a)
    	.def_static("test_multiple_cb_args", &CallbackWrapper::test_multiple_cb_args, "f"_a, "g"_a)
    	.def_static("test_no_args", &CallbackWrapper::test_no_args, "h"_a)
    	.def_static("test_str_cb_arg", &CallbackWrapper::test_str_cb_arg, "f"_a);
    
    nb::class_<ImportedStruct>(somelib_mod, "ImportedStruct")
        .def(nb::init<>())
        .def(nb::init<UnimportedEnum, uint8_t>(), "foo"_a.none(),  "count"_a.none())
        .def_rw("foo", &ImportedStruct::foo)
        .def_rw("count", &ImportedStruct::count);
    
    nb::class_<BorrowedFields>(somelib_mod, "BorrowedFields")
        .def(nb::init<>())
        .def(nb::init<std::u16string_view, std::string_view, std::string_view>(), "a"_a.none(),  "b"_a.none(),  "c"_a.none())
        .def_rw("a", &BorrowedFields::a)
        .def_rw("b", &BorrowedFields::b)
        .def_rw("c", &BorrowedFields::c)
    	.def_static("from_bar_and_strings", &BorrowedFields::from_bar_and_strings, "bar"_a, "dstr16"_a, "utf8_str"_a, nb::keep_alive<0, 1>(), nb::keep_alive<0, 2>(), nb::keep_alive<0, 3>());
    
    nb::class_<BorrowedFieldsReturning>(somelib_mod, "BorrowedFieldsReturning")
        .def(nb::init<>())
        .def(nb::init<std::string_view>(), "bytes"_a.none())
        .def_rw("bytes", &BorrowedFieldsReturning::bytes);
    
    nb::class_<BorrowedFieldsWithBounds>(somelib_mod, "BorrowedFieldsWithBounds")
        .def(nb::init<>())
        .def(nb::init<std::u16string_view, std::string_view, std::string_view>(), "field_a"_a.none(),  "field_b"_a.none(),  "field_c"_a.none())
        .def_rw("field_a", &BorrowedFieldsWithBounds::field_a)
        .def_rw("field_b", &BorrowedFieldsWithBounds::field_b)
        .def_rw("field_c", &BorrowedFieldsWithBounds::field_c)
    	.def_static("from_foo_and_strings", &BorrowedFieldsWithBounds::from_foo_and_strings, "foo"_a, "dstr16_x"_a, "utf8_str_z"_a, nb::keep_alive<0, 1>(), nb::keep_alive<0, 2>(), nb::keep_alive<0, 3>());
    
    nb::class_<NestedBorrowedFields>(somelib_mod, "NestedBorrowedFields")
        .def(nb::init<>())
        .def(nb::init<BorrowedFields, BorrowedFieldsWithBounds, BorrowedFieldsWithBounds>(), "fields"_a.none(),  "bounds"_a.none(),  "bounds2"_a.none())
        .def_rw("fields", &NestedBorrowedFields::fields)
        .def_rw("bounds", &NestedBorrowedFields::bounds)
        .def_rw("bounds2", &NestedBorrowedFields::bounds2)
    	.def_static("from_bar_and_foo_and_strings", &NestedBorrowedFields::from_bar_and_foo_and_strings, "bar"_a, "foo"_a, "dstr16_x"_a, "dstr16_z"_a, "utf8_str_y"_a, "utf8_str_z"_a, nb::keep_alive<0, 1>(), nb::keep_alive<0, 2>(), nb::keep_alive<0, 3>(), nb::keep_alive<0, 4>(), nb::keep_alive<0, 5>(), nb::keep_alive<0, 6>());
    
    nb::class_<OptionInputStruct>(somelib_mod, "OptionInputStruct")
        .def(nb::init<>())
        .def(nb::init<std::optional<uint8_t>, std::optional<char32_t>, std::optional<OptionEnum>>(), "a"_a.none(),  "b"_a.none(),  "c"_a.none())
        .def_rw("a", &OptionInputStruct::a)
        .def_rw("b", &OptionInputStruct::b)
        .def_rw("c", &OptionInputStruct::c);
    
    nb::class_<ErrorStruct>(somelib_mod, "ErrorStruct")
        .def(nb::init<>())
        .def(nb::init<int32_t, int32_t>(), "i"_a.none(),  "j"_a.none())
        .def_rw("i", &ErrorStruct::i)
        .def_rw("j", &ErrorStruct::j);
    
    nb::class_<CyclicStructA>(somelib_mod, "CyclicStructA")
        .def(nb::init<>())
        .def(nb::init<CyclicStructB>(), "a"_a.none())
        .def_rw("a", &CyclicStructA::a)
    	.def("cyclic_out", &CyclicStructA::cyclic_out)
    	.def("double_cyclic_out", &CyclicStructA::double_cyclic_out, "cyclic_struct_a"_a)
    	.def_static("get_b", &CyclicStructA::get_b)
    	.def_prop_ro("getter_out", &CyclicStructA::getter_out);
    
    nb::class_<CyclicStructB>(somelib_mod, "CyclicStructB")
        .def(nb::init<>())
        .def(nb::init<uint8_t>(), "field"_a.none())
        .def_rw("field", &CyclicStructB::field)
    	.def_static("get_a", &CyclicStructB::get_a)
    	.def_static("get_a_option", &CyclicStructB::get_a_option);
    
    nb::class_<CyclicStructC>(somelib_mod, "CyclicStructC")
        .def(nb::init<>())
        .def(nb::init<CyclicStructA>(), "a"_a.none())
        .def_rw("a", &CyclicStructC::a)
    	.def("cyclic_out", &CyclicStructC::cyclic_out)
    	.def_static("takes_nested_parameters", &CyclicStructC::takes_nested_parameters, "c"_a);
    
    nb::class_<MyStruct>(somelib_mod, "MyStruct")
        .def_rw("a", &MyStruct::a)
        .def_rw("b", &MyStruct::b)
        .def_rw("c", &MyStruct::c)
        .def_rw("d", &MyStruct::d)
        .def_rw("e", &MyStruct::e)
        .def_rw("f", &MyStruct::f)
        .def_rw("g", &MyStruct::g)
    	.def_static("fails_zst_result", &MyStruct::fails_zst_result)
    	.def("into_a", &MyStruct::into_a)
    	.def("__init__", [](MyStruct* self){ *self = MyStruct::new_(); })
    	.def_static("returns_zst_result", &MyStruct::returns_zst_result);
    
    nb::class_<MyStructContainingAnOption>(somelib_mod, "MyStructContainingAnOption")
        .def_rw("a", &MyStructContainingAnOption::a)
        .def_rw("b", &MyStructContainingAnOption::b)
    	.def_static("filled", &MyStructContainingAnOption::filled)
    	.def("__init__", [](MyStructContainingAnOption* self){ *self = MyStructContainingAnOption::new_(); });
    
    nb::class_<MyZst>(somelib_mod, "MyZst")
        .def(nb::init<>());
    
    nb::class_<StructArithmetic>(somelib_mod, "StructArithmetic")
        .def_rw("x", &StructArithmetic::x)
        .def_rw("y", &StructArithmetic::y)
    	.def(nb::self + nb::self)
    	.def(nb::self / nb::self)
    	.def(nb::self * nb::self)
    	.def("__init__", [](StructArithmetic* self, int32_t x, int32_t y){ *self = StructArithmetic::new_(x, y); }, "x"_a, "y"_a)
    	.def(nb::self - nb::self);
    
    nb::class_<OptionStruct>(somelib_mod, "OptionStruct")
        .def(nb::init<>())
        .def(nb::init<std::unique_ptr<OptionOpaque>, std::unique_ptr<OptionOpaqueChar>, uint32_t, std::unique_ptr<OptionOpaque>>(), "a"_a,  "b"_a,  "c"_a.none(),  "d"_a)
        .def_prop_rw("a", 
            [](const OptionStruct& self) { return self.a.get(); },
            [](OptionStruct& self, std::unique_ptr<OptionOpaque>&& v) { self.a = std::move(v); }
        )
        .def_prop_rw("b", 
            [](const OptionStruct& self) { return self.b.get(); },
            [](OptionStruct& self, std::unique_ptr<OptionOpaqueChar>&& v) { self.b = std::move(v); }
        )
        .def_rw("c", &OptionStruct::c)
        .def_prop_rw("d", 
            [](const OptionStruct& self) { return self.d.get(); },
            [](OptionStruct& self, std::unique_ptr<OptionOpaque>&& v) { self.d = std::move(v); }
        );
    
    PyType_Slot ns_AttrOpaque1Renamed_slots[] = {
        {Py_tp_free, (void *)ns::AttrOpaque1Renamed::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<ns::AttrOpaque1Renamed>(ns_mod, "AttrOpaque1Renamed", nb::type_slots(ns_AttrOpaque1Renamed_slots))
    	.def_prop_ro("abirenamed", &ns::AttrOpaque1Renamed::abirenamed)
    	.def_prop_ro("method", &ns::AttrOpaque1Renamed::method_renamed)
    	.def(nb::new_(&ns::AttrOpaque1Renamed::totally_not_new))
    	.def("use_namespaced", &ns::AttrOpaque1Renamed::use_namespaced, "_n"_a)
    	.def("use_unnamespaced", &ns::AttrOpaque1Renamed::use_unnamespaced, "_un"_a);
    
    PyType_Slot ns_RenamedAttrOpaque2_slots[] = {
        {Py_tp_free, (void *)ns::RenamedAttrOpaque2::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<ns::RenamedAttrOpaque2>(ns_mod, "RenamedAttrOpaque2", nb::type_slots(ns_RenamedAttrOpaque2_slots));
    
    PyType_Slot ns_RenamedComparable_slots[] = {
        {Py_tp_free, (void *)ns::RenamedComparable::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<ns::RenamedComparable>(ns_mod, "RenamedComparable", nb::type_slots(ns_RenamedComparable_slots))
    	.def(nb::self == nb::self)
    		.def(nb::self != nb::self)
    		.def(nb::self <= nb::self)
    		.def(nb::self >= nb::self)
    		.def(nb::self < nb::self)
    		.def(nb::self > nb::self)
    	.def_static("new", &ns::RenamedComparable::new_, "int"_a);
    
    PyType_Slot ns_RenamedMyIndexer_slots[] = {
        {Py_tp_free, (void *)ns::RenamedMyIndexer::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<ns::RenamedMyIndexer>(ns_mod, "RenamedMyIndexer", nb::type_slots(ns_RenamedMyIndexer_slots))
    	.def("__getitem__", &ns::RenamedMyIndexer::operator[], "i"_a, nb::keep_alive<0, 1>());
    
    PyType_Slot ns_RenamedMyIterable_slots[] = {
        {Py_tp_free, (void *)ns::RenamedMyIterable::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<ns::RenamedMyIterable>(ns_mod, "RenamedMyIterable", nb::type_slots(ns_RenamedMyIterable_slots))
    	.def("__len__", &ns::RenamedMyIterable::__len__)
    	.def("__iter__", &ns::RenamedMyIterable::iter, nb::keep_alive<0, 1>())
    	.def(nb::new_(&ns::RenamedMyIterable::new_), "x"_a);
    
    PyType_Slot ns_RenamedMyIterator_slots[] = {
        {Py_tp_free, (void *)ns::RenamedMyIterator::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<ns::RenamedMyIterator>(ns_mod, "RenamedMyIterator", nb::type_slots(ns_RenamedMyIterator_slots))
    	.def("__next__", [](ns::RenamedMyIterator& self){
    			auto next = self.next();
    			if (!next) {
    				throw nb::stop_iteration();
    			}
    			return next_inner_extractor<decltype(next)>::get(std::move(next));
    		})
    		.def("__iter__", [](nb::handle self) { return self; });
    nb::module_ nested_mod = somelib_mod.def_submodule("nested");
    
    PyType_Slot nested_ns_Nested_slots[] = {
        {Py_tp_free, (void *)nested::ns::Nested::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<nested::ns::Nested>(ns_mod, "Nested", nb::type_slots(nested_ns_Nested_slots));
    nb::module_ ns2_mod = somelib_mod.def_submodule("ns2");
    
    PyType_Slot nested_ns2_Nested_slots[] = {
        {Py_tp_free, (void *)nested::ns2::Nested::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<nested::ns2::Nested>(ns2_mod, "Nested", nb::type_slots(nested_ns2_Nested_slots));
    
    PyType_Slot ns_RenamedOpaqueArithmetic_slots[] = {
        {Py_tp_free, (void *)ns::RenamedOpaqueArithmetic::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<ns::RenamedOpaqueArithmetic>(ns_mod, "RenamedOpaqueArithmetic", nb::type_slots(ns_RenamedOpaqueArithmetic_slots))
    	.def(nb::self + nb::self)
    	.def(nb::self += nb::self, nb::rv_policy::none)
    	.def(nb::self / nb::self)
    	.def(nb::self /= nb::self, nb::rv_policy::none)
    	.def_static("make", &ns::RenamedOpaqueArithmetic::make, "x"_a, "y"_a)
    	.def(nb::self * nb::self)
    	.def(nb::self *= nb::self, nb::rv_policy::none)
    	.def(nb::self - nb::self)
    	.def(nb::self -= nb::self, nb::rv_policy::none)
    	.def("x", &ns::RenamedOpaqueArithmetic::x)
    	.def("y", &ns::RenamedOpaqueArithmetic::y);
    
    PyType_Slot ns_RenamedOpaqueIterable_slots[] = {
        {Py_tp_free, (void *)ns::RenamedOpaqueIterable::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<ns::RenamedOpaqueIterable>(ns_mod, "RenamedOpaqueIterable", nb::type_slots(ns_RenamedOpaqueIterable_slots))
    	.def("__iter__", &ns::RenamedOpaqueIterable::iter, nb::keep_alive<0, 1>());
    
    PyType_Slot ns_RenamedOpaqueIterator_slots[] = {
        {Py_tp_free, (void *)ns::RenamedOpaqueIterator::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<ns::RenamedOpaqueIterator>(ns_mod, "RenamedOpaqueIterator", nb::type_slots(ns_RenamedOpaqueIterator_slots))
    	.def("__next__", [](ns::RenamedOpaqueIterator& self){
    			auto next = self.next();
    			if (!next) {
    				throw nb::stop_iteration();
    			}
    			return next_inner_extractor<decltype(next)>::get(std::move(next));
    		})
    		.def("__iter__", [](nb::handle self) { return self; });
    
    PyType_Slot Unnamespaced_slots[] = {
        {Py_tp_free, (void *)Unnamespaced::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<Unnamespaced>(somelib_mod, "Unnamespaced", nb::type_slots(Unnamespaced_slots))
    	.def_static("make", &Unnamespaced::make, "_e"_a ) // unsupported special method NamedConstructor(None)
    	.def("use_namespaced", &Unnamespaced::use_namespaced, "_n"_a);
    
    PyType_Slot Bar_slots[] = {
        {Py_tp_free, (void *)Bar::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<Bar>(somelib_mod, "Bar", nb::type_slots(Bar_slots))
    	.def_prop_ro("foo", &Bar::foo);
    
    PyType_Slot Foo_slots[] = {
        {Py_tp_free, (void *)Foo::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<Foo>(somelib_mod, "Foo", nb::type_slots(Foo_slots))
    	.def("as_returning", &Foo::as_returning, nb::keep_alive<0, 1>())
    	.def_prop_ro("bar", &Foo::get_bar)
    	.def_static("extract_from_bounds", &Foo::extract_from_bounds, "bounds"_a, "another_string"_a, nb::keep_alive<0, 1>(), nb::keep_alive<0, 2>() ) // unsupported special method NamedConstructor(None)
    	.def_static("extract_from_fields", &Foo::extract_from_fields, "fields"_a, nb::keep_alive<0, 1>() ) // unsupported special method NamedConstructor(None)
    	.def(nb::new_(&Foo::new_), "x"_a, nb::keep_alive<1, 2>())
    	.def_static("new_static", &Foo::new_static, "x"_a ) // unsupported special method NamedConstructor(Some("static"))
    ;
    
    PyType_Slot One_slots[] = {
        {Py_tp_free, (void *)One::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<One>(somelib_mod, "One", nb::type_slots(One_slots))
    	.def_static("cycle", &One::cycle, "hold"_a, "nohold"_a, nb::keep_alive<0, 1>() ) // unsupported special method NamedConstructor(None)
    	.def_static("diamond_and_nested_types", &One::diamond_and_nested_types, "a"_a, "b"_a, "c"_a, "d"_a, "nohold"_a, nb::keep_alive<0, 1>(), nb::keep_alive<0, 2>(), nb::keep_alive<0, 3>(), nb::keep_alive<0, 4>() ) // unsupported special method NamedConstructor(None)
    	.def_static("diamond_bottom", &One::diamond_bottom, "top"_a, "left"_a, "right"_a, "bottom"_a, nb::keep_alive<0, 4>() ) // unsupported special method NamedConstructor(None)
    	.def_static("diamond_left", &One::diamond_left, "top"_a, "left"_a, "right"_a, "bottom"_a, nb::keep_alive<0, 2>(), nb::keep_alive<0, 4>() ) // unsupported special method NamedConstructor(None)
    	.def_static("diamond_right", &One::diamond_right, "top"_a, "left"_a, "right"_a, "bottom"_a, nb::keep_alive<0, 3>(), nb::keep_alive<0, 4>() ) // unsupported special method NamedConstructor(None)
    	.def_static("diamond_top", &One::diamond_top, "top"_a, "left"_a, "right"_a, "bottom"_a, nb::keep_alive<0, 1>(), nb::keep_alive<0, 2>(), nb::keep_alive<0, 3>(), nb::keep_alive<0, 4>() ) // unsupported special method NamedConstructor(None)
    	.def_static("implicit_bounds", &One::implicit_bounds, "explicit_hold"_a, "implicit_hold"_a, "nohold"_a, nb::keep_alive<0, 1>(), nb::keep_alive<0, 2>() ) // unsupported special method NamedConstructor(None)
    	.def_static("implicit_bounds_deep", &One::implicit_bounds_deep, "explicit_"_a, "implicit_1"_a, "implicit_2"_a, "nohold"_a, nb::keep_alive<0, 1>(), nb::keep_alive<0, 2>(), nb::keep_alive<0, 3>() ) // unsupported special method NamedConstructor(None)
    	.def_static("many_dependents", &One::many_dependents, "a"_a, "b"_a, "c"_a, "d"_a, "nohold"_a, nb::keep_alive<0, 1>(), nb::keep_alive<0, 2>(), nb::keep_alive<0, 3>(), nb::keep_alive<0, 4>() ) // unsupported special method NamedConstructor(None)
    	.def_static("return_outlives_param", &One::return_outlives_param, "hold"_a, "nohold"_a, nb::keep_alive<0, 1>() ) // unsupported special method NamedConstructor(None)
    	.def_static("transitivity", &One::transitivity, "hold"_a, "nohold"_a, nb::keep_alive<0, 1>() ) // unsupported special method NamedConstructor(None)
    ;
    
    PyType_Slot OpaqueThin_slots[] = {
        {Py_tp_free, (void *)OpaqueThin::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<OpaqueThin>(somelib_mod, "OpaqueThin", nb::type_slots(OpaqueThin_slots))
    	.def_prop_ro("a", &OpaqueThin::a)
    	.def_prop_ro("b", &OpaqueThin::b);
    
    PyType_Slot OpaqueThinIter_slots[] = {
        {Py_tp_free, (void *)OpaqueThinIter::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<OpaqueThinIter>(somelib_mod, "OpaqueThinIter", nb::type_slots(OpaqueThinIter_slots))
    	.def("__next__", [](OpaqueThinIter& self){
    			auto next = self.next();
    			if (!next) {
    				throw nb::stop_iteration();
    			}
    			return next_inner_extractor<decltype(next)>::get(std::move(next));
    		}, nb::keep_alive<0, 1>(), nb::rv_policy::reference)
    		.def("__iter__", [](nb::handle self) { return self; });
    
    PyType_Slot OpaqueThinVec_slots[] = {
        {Py_tp_free, (void *)OpaqueThinVec::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<OpaqueThinVec>(somelib_mod, "OpaqueThinVec", nb::type_slots(OpaqueThinVec_slots))
    	.def("__len__", &OpaqueThinVec::__len__)
    	.def(nb::new_(&OpaqueThinVec::create), "a"_a, "b"_a)
    	.def_prop_ro("first", &OpaqueThinVec::first)
    	.def("__getitem__", &OpaqueThinVec::operator[], "idx"_a, nb::keep_alive<0, 1>(), nb::rv_policy::reference)
    	.def("__iter__", &OpaqueThinVec::iter, nb::keep_alive<0, 1>());
    
    PyType_Slot Two_slots[] = {
        {Py_tp_free, (void *)Two::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<Two>(somelib_mod, "Two", nb::type_slots(Two_slots));
    
    PyType_Slot OptionOpaque_slots[] = {
        {Py_tp_free, (void *)OptionOpaque::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<OptionOpaque>(somelib_mod, "OptionOpaque", nb::type_slots(OptionOpaque_slots))
    	.def_static("accepts_option_enum", &OptionOpaque::accepts_option_enum, "arg"_a= nb::none(), "sentinel"_a)
    	.def_static("accepts_option_input_struct", &OptionOpaque::accepts_option_input_struct, "arg"_a= nb::none(), "sentinel"_a)
    	.def_static("accepts_option_primitive", &OptionOpaque::accepts_option_primitive, "arg"_a= nb::none(), "sentinel"_a)
    	.def_static("accepts_option_str", &OptionOpaque::accepts_option_str, "arg"_a= nb::none(), "sentinel"_a)
    	.def_static("accepts_option_str_slice", &OptionOpaque::accepts_option_str_slice, "arg"_a= nb::none(), "sentinel"_a)
    	.def_static("accepts_option_u8", &OptionOpaque::accepts_option_u8, "arg"_a= nb::none(), "sentinel"_a)
    	.def("assert_integer", &OptionOpaque::assert_integer, "i"_a)
    	.def_static("new", &OptionOpaque::new_, "i"_a)
    	.def_static("new_none", &OptionOpaque::new_none)
    	.def_static("new_struct", &OptionOpaque::new_struct)
    	.def_static("new_struct_nones", &OptionOpaque::new_struct_nones)
    	.def("option_i32", &OptionOpaque::option_i32)
    	.def("option_isize", &OptionOpaque::option_isize)
    	.def_static("option_opaque_argument", &OptionOpaque::option_opaque_argument, "arg"_a= nb::none())
    	.def("option_u32", &OptionOpaque::option_u32)
    	.def("option_usize", &OptionOpaque::option_usize)
    	.def_static("returns", &OptionOpaque::returns)
    	.def("returns_none_self", &OptionOpaque::returns_none_self, nb::keep_alive<0, 1>(), nb::rv_policy::reference)
    	.def_static("returns_option_input_struct", &OptionOpaque::returns_option_input_struct)
    	.def("returns_some_self", &OptionOpaque::returns_some_self, nb::keep_alive<0, 1>(), nb::rv_policy::reference);
    
    PyType_Slot OptionOpaqueChar_slots[] = {
        {Py_tp_free, (void *)OptionOpaqueChar::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<OptionOpaqueChar>(somelib_mod, "OptionOpaqueChar", nb::type_slots(OptionOpaqueChar_slots))
    	.def("assert_char", &OptionOpaqueChar::assert_char, "ch"_a);
    
    PyType_Slot OptionString_slots[] = {
        {Py_tp_free, (void *)OptionString::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<OptionString>(somelib_mod, "OptionString", nb::type_slots(OptionString_slots))
    	.def("borrow", &OptionString::borrow, nb::keep_alive<0, 1>())
    	.def_static("new", &OptionString::new_, "diplomat_str"_a)
    	.def("write", &OptionString::write);
    
    PyType_Slot ResultOpaque_slots[] = {
        {Py_tp_free, (void *)ResultOpaque::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<ResultOpaque>(somelib_mod, "ResultOpaque", nb::type_slots(ResultOpaque_slots))
    	.def("assert_integer", &ResultOpaque::assert_integer, "i"_a)
    	.def(nb::new_(&ResultOpaque::new_), "i"_a)
    	.def_static("new_failing_bar", &ResultOpaque::new_failing_bar)
    	.def_static("new_failing_foo", &ResultOpaque::new_failing_foo)
    	.def_static("new_failing_struct", &ResultOpaque::new_failing_struct, "i"_a)
    	.def_static("new_failing_unit", &ResultOpaque::new_failing_unit)
    	.def_static("new_in_enum_err", &ResultOpaque::new_in_enum_err, "i"_a)
    	.def_static("new_in_err", &ResultOpaque::new_in_err, "i"_a)
    	.def_static("new_int", &ResultOpaque::new_int, "i"_a)
    	.def("takes_str", &ResultOpaque::takes_str, "_v"_a, nb::keep_alive<0, 1>(), nb::rv_policy::reference);
    
    PyType_Slot RefList_slots[] = {
        {Py_tp_free, (void *)RefList::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<RefList>(somelib_mod, "RefList", nb::type_slots(RefList_slots))
    	.def_static("node", &RefList::node, "data"_a, nb::keep_alive<0, 1>() ) // unsupported special method NamedConstructor(None)
    ;
    
    PyType_Slot RefListParameter_slots[] = {
        {Py_tp_free, (void *)RefListParameter::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<RefListParameter>(somelib_mod, "RefListParameter", nb::type_slots(RefListParameter_slots));
    
    PyType_Slot Float64Vec_slots[] = {
        {Py_tp_free, (void *)Float64Vec::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<Float64Vec>(somelib_mod, "Float64Vec", nb::type_slots(Float64Vec_slots))
    	.def_prop_ro("asSlice", &Float64Vec::as_slice)
    	.def("borrow", &Float64Vec::borrow, nb::keep_alive<0, 1>())
    	.def("fill_slice", &Float64Vec::fill_slice, "v"_a)
    	.def("__getitem__", &Float64Vec::operator[], "i"_a)
    	.def_static("new", &Float64Vec::new_, "v"_a)
    	.def_static("new_bool", &Float64Vec::new_bool, "v"_a ) // unsupported special method NamedConstructor(Some("bool"))
    	.def_static("new_f64_be_bytes", &Float64Vec::new_f64_be_bytes, "v"_a ) // unsupported special method NamedConstructor(Some("f64BeBytes"))
    	.def_static("new_i16", &Float64Vec::new_i16, "v"_a ) // unsupported special method NamedConstructor(Some("i16"))
    	.def_static("new_isize", &Float64Vec::new_isize, "v"_a ) // unsupported special method NamedConstructor(Some("isize"))
    	.def_static("new_u16", &Float64Vec::new_u16, "v"_a ) // unsupported special method NamedConstructor(Some("u16"))
    	.def_static("new_usize", &Float64Vec::new_usize, "v"_a ) // unsupported special method NamedConstructor(Some("usize"))
    	.def("set_value", &Float64Vec::set_value, "new_slice"_a)
    	.def("__str__", &Float64Vec::to_string);
    
    PyType_Slot MyString_slots[] = {
        {Py_tp_free, (void *)MyString::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<MyString>(somelib_mod, "MyString", nb::type_slots(MyString_slots))
    	.def("borrow", &MyString::borrow, nb::keep_alive<0, 1>())
    	.def_static("get_static_str", &MyString::get_static_str)
    	.def(nb::new_(&MyString::new_), "v"_a)
    	.def_static("new_from_first", &MyString::new_from_first, "v"_a)
    	.def_static("new_owned", &MyString::new_owned, "v"_a)
    	.def_static("new_unsafe", &MyString::new_unsafe, "v"_a ) // unsupported special method NamedConstructor(Some("unsafe"))
    	.def_prop_rw("str", &MyString::get_str, &MyString::set_str)
    	.def_static("string_transform", &MyString::string_transform, "foo"_a);
    
    PyType_Slot MyOpaqueEnum_slots[] = {
        {Py_tp_free, (void *)MyOpaqueEnum::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<MyOpaqueEnum>(somelib_mod, "MyOpaqueEnum", nb::type_slots(MyOpaqueEnum_slots))
    	.def_static("new", &MyOpaqueEnum::new_)
    	.def("to_string", &MyOpaqueEnum::to_string);
    
    PyType_Slot Opaque_slots[] = {
        {Py_tp_free, (void *)Opaque::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<Opaque>(somelib_mod, "Opaque", nb::type_slots(Opaque_slots))
    	.def("assert_struct", &Opaque::assert_struct, "s"_a)
    	.def_static("cmp", &Opaque::cmp)
    	.def_static("from_str", &Opaque::from_str, "input"_a)
    	.def("get_debug_str", &Opaque::get_debug_str)
    	.def(nb::new_(&Opaque::new_))
    	.def_static("returns_imported", &Opaque::returns_imported)
    	.def_static("returns_usize", &Opaque::returns_usize)
    	.def_static("try_from_utf8", &Opaque::try_from_utf8, "input"_a);
    
    PyType_Slot OpaqueMutexedString_slots[] = {
        {Py_tp_free, (void *)OpaqueMutexedString::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<OpaqueMutexedString>(somelib_mod, "OpaqueMutexedString", nb::type_slots(OpaqueMutexedString_slots))
    	.def("borrow", &OpaqueMutexedString::borrow, nb::keep_alive<0, 1>(), nb::rv_policy::reference)
    	.def_static("borrow_other", &OpaqueMutexedString::borrow_other, "other"_a, nb::keep_alive<0, 1>(), nb::rv_policy::reference)
    	.def("borrow_self_or_other", &OpaqueMutexedString::borrow_self_or_other, "other"_a, nb::keep_alive<0, 1>(), nb::keep_alive<0, 2>(), nb::rv_policy::reference)
    	.def("change", &OpaqueMutexedString::change, "number"_a)
    	.def("dummy_str", &OpaqueMutexedString::dummy_str, nb::keep_alive<0, 1>())
    	.def_static("from_usize", &OpaqueMutexedString::from_usize, "number"_a)
    	.def("get_len_and_add", &OpaqueMutexedString::get_len_and_add, "other"_a)
    	.def("to_unsigned_from_unsigned", &OpaqueMutexedString::to_unsigned_from_unsigned, "input"_a)
    	.def("wrapper", &OpaqueMutexedString::wrapper);
    
    PyType_Slot Utf16Wrap_slots[] = {
        {Py_tp_free, (void *)Utf16Wrap::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<Utf16Wrap>(somelib_mod, "Utf16Wrap", nb::type_slots(Utf16Wrap_slots))
    	.def("borrow_cont", &Utf16Wrap::borrow_cont, nb::keep_alive<0, 1>())
    	.def(nb::new_(&Utf16Wrap::from_utf16), "input"_a)
    	.def("get_debug_str", &Utf16Wrap::get_debug_str);
    {
    	nb::class_<ns::RenamedAttrEnum> e_class(ns_mod, "RenamedAttrEnum");
    
    	nb::enum_<ns::RenamedAttrEnum::Value>(e_class, "RenamedAttrEnum")
    		.value("A", ns::RenamedAttrEnum::A)
    		.value("B", ns::RenamedAttrEnum::B)
    		.value("Renamed", ns::RenamedAttrEnum::Renamed)
    		.export_values();
    
    	e_class
    		.def(nb::init_implicit<ns::RenamedAttrEnum::Value>())
    		.def(nb::self == ns::RenamedAttrEnum::Value())
    		.def("__repr__", [](const ns::RenamedAttrEnum& self){
    			return nb::str(nb::cast(ns::RenamedAttrEnum::Value(self)));
    		});
    }
    
    {
    	nb::class_<UnimportedEnum> e_class(somelib_mod, "UnimportedEnum");
    
    	nb::enum_<UnimportedEnum::Value>(e_class, "UnimportedEnum")
    		.value("A", UnimportedEnum::A)
    		.value("B", UnimportedEnum::B)
    		.value("C", UnimportedEnum::C)
    		.export_values();
    
    	e_class
    		.def(nb::init_implicit<UnimportedEnum::Value>())
    		.def(nb::self == UnimportedEnum::Value())
    		.def("__repr__", [](const UnimportedEnum& self){
    			return nb::str(nb::cast(UnimportedEnum::Value(self)));
    		});
    }
    
    {
    	nb::class_<OptionEnum> e_class(somelib_mod, "OptionEnum");
    
    	nb::enum_<OptionEnum::Value>(e_class, "OptionEnum")
    		.value("Foo", OptionEnum::Foo)
    		.value("Bar", OptionEnum::Bar)
    		.export_values();
    
    	e_class
    		.def(nb::init_implicit<OptionEnum::Value>())
    		.def(nb::self == OptionEnum::Value())
    		.def("__repr__", [](const OptionEnum& self){
    			return nb::str(nb::cast(OptionEnum::Value(self)));
    		});
    }
    
    {
    	nb::class_<ErrorEnum> e_class(somelib_mod, "ErrorEnum");
    
    	nb::enum_<ErrorEnum::Value>(e_class, "ErrorEnum")
    		.value("Foo", ErrorEnum::Foo)
    		.value("Bar", ErrorEnum::Bar)
    		.export_values();
    
    	e_class
    		.def(nb::init_implicit<ErrorEnum::Value>())
    		.def(nb::self == ErrorEnum::Value())
    		.def("__repr__", [](const ErrorEnum& self){
    			return nb::str(nb::cast(ErrorEnum::Value(self)));
    		});
    }
    
    {
    	nb::class_<ContiguousEnum> e_class(somelib_mod, "ContiguousEnum");
    
    	nb::enum_<ContiguousEnum::Value>(e_class, "ContiguousEnum")
    		.value("C", ContiguousEnum::C)
    		.value("D", ContiguousEnum::D)
    		.value("E", ContiguousEnum::E)
    		.value("F", ContiguousEnum::F)
    		.export_values();
    
    	e_class
    		.def(nb::init_implicit<ContiguousEnum::Value>())
    		.def(nb::self == ContiguousEnum::Value())
    		.def("__repr__", [](const ContiguousEnum& self){
    			return nb::str(nb::cast(ContiguousEnum::Value(self)));
    		});
    }
    
    {
    	nb::class_<DefaultEnum> e_class(somelib_mod, "DefaultEnum");
    
    	nb::enum_<DefaultEnum::Value>(e_class, "DefaultEnum")
    		.value("A", DefaultEnum::A)
    		.value("B", DefaultEnum::B)
    		.export_values();
    
    	e_class
    		.def(nb::init_implicit<DefaultEnum::Value>())
    		.def(nb::self == DefaultEnum::Value())
    		.def("__repr__", [](const DefaultEnum& self){
    			return nb::str(nb::cast(DefaultEnum::Value(self)));
    		});
    }
    
    {
    	nb::class_<MyEnum> e_class(somelib_mod, "MyEnum");
    
    	nb::enum_<MyEnum::Value>(e_class, "MyEnum")
    		.value("A", MyEnum::A)
    		.value("B", MyEnum::B)
    		.value("C", MyEnum::C)
    		.value("D", MyEnum::D)
    		.value("E", MyEnum::E)
    		.value("F", MyEnum::F)
    		.export_values();
    
    	e_class
    		.def(nb::init_implicit<MyEnum::Value>())
    		.def(nb::self == MyEnum::Value())
    		.def("__repr__", [](const MyEnum& self){
    			return nb::str(nb::cast(MyEnum::Value(self)));
    		});
    }
}