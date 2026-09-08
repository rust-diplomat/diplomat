#include "diplomat_nanobind_common.hpp"


#include "FFIError.hpp"
#include "FallibleTesterTrait.hpp"

namespace somelib {
void add_FallibleTesterTrait_binding(nb::module_ mod) {
    class FallibleTesterTrait_PyImpl : public somelib::FallibleTesterTrait {
        protected:
        nb::handle self;
        
        virtual somelib::diplomat::result<std::monostate, somelib::FFIError> test_void_trait_fn() override {
            // We grab the pointer to the method inside to avoid refcount issues:
            nb::object func = nb::object(PyObject_GetAttrString(self.ptr(), "test_void_trait_fn"), nb::detail::steal_t());
    
            // nb::handle throws std::exception on failure:
            nb::object out;
            try {
                out = func();
            } catch (...) {
                return diplomat::Err(somelib::FFIError(somelib::FFIError::Value::FFI));
            }
            nb::detail::make_caster<somelib::diplomat::result<std::monostate, somelib::FFIError>> caster;
            if (caster.from_python(out, 0, nullptr)) {
                if (caster.ok_val.has_value()) {
                    return diplomat::Ok(caster.ok_val.value());
                } else if (caster.err_val.has_value()) {
                    return diplomat::Err(caster.err_val.value());
                } else {
                    return diplomat::Err(somelib::FFIError(somelib::FFIError::Value::FFI));
                }
            } else {
                return diplomat::Err(somelib::FFIError(somelib::FFIError::Value::FFI));
            }
        }
        virtual somelib::diplomat::result<uint32_t, somelib::FFIError> test_result_output(uint32_t x) override {
            // We grab the pointer to the method inside to avoid refcount issues:
            nb::object func = nb::object(PyObject_GetAttrString(self.ptr(), "test_result_output"), nb::detail::steal_t());
    
            // nb::handle throws std::exception on failure:
            nb::object out;
            try {
                out = func(x);
            } catch (...) {
                return diplomat::Err(somelib::FFIError(somelib::FFIError::Value::FFI));
            }
            nb::detail::make_caster<somelib::diplomat::result<uint32_t, somelib::FFIError>> caster;
            if (caster.from_python(out, 0, nullptr)) {
                if (caster.ok_val.has_value()) {
                    return diplomat::Ok(caster.ok_val.value());
                } else if (caster.err_val.has_value()) {
                    return diplomat::Err(caster.err_val.value());
                } else {
                    return diplomat::Err(somelib::FFIError(somelib::FFIError::Value::FFI));
                }
            } else {
                return diplomat::Err(somelib::FFIError(somelib::FFIError::Value::FFI));
            }
        }
    
        public:
        FallibleTesterTrait_PyImpl(nb::handle self) : self(self) {}
    
        // Allocate as Nanobind would a unique_ptr.
        static PyObject* new_(PyObject* cls, PyObject*, PyObject*) {
            // Main thing is to note that we use Python's allocator:
            auto o = nb::detail::inst_new_ext((PyTypeObject*)cls, malloc(sizeof(FallibleTesterTrait_PyImpl)));
            if (!o) {
                return nullptr;
            }
    
            auto inst = (nb::detail::nb_inst*) o;
            // Then, because we're returning from C++, we tell Python to take ownership:
            inst->destruct = true;
            inst->cpp_delete = true;
            return o;
        }
    
        static inline void invalidMethod(PyObject* self, const char* method_name) {
            PyTypeObject* tp = Py_TYPE(self);
            PyObject* tp_name = PyObject_GetAttrString((PyObject*)tp, "__name__");
            PyErr_Format(PyExc_AttributeError, "%U does not implement: '%s'", tp_name, method_name);
            Py_DECREF(tp_name);
        }
    
        // Get handles to the Python methods.
        static int init(PyObject* self, PyObject*, PyObject*) {
            auto inst = (nb::detail::nb_inst*) self;
            // Our methods are already defined (per the stub definitions below), so we just check if they're Nanobind methods, and throw if so.
            PyObject* test_void_trait_fn_method = PyObject_GetAttrString(self, "test_void_trait_fn");
            if (!test_void_trait_fn_method || Py_TYPE(test_void_trait_fn_method) == nb::detail::internals->nb_method || 
                Py_TYPE(test_void_trait_fn_method) == nb::detail::internals->nb_func ||
                Py_TYPE(test_void_trait_fn_method) == nb::detail::internals->nb_bound_method) {
                Py_DECREF(test_void_trait_fn_method);
                invalidMethod(self, "test_void_trait_fn");
                return -1;
            }
            Py_DECREF(test_void_trait_fn_method);
            PyObject* test_result_output_method = PyObject_GetAttrString(self, "test_result_output");
            if (!test_result_output_method || Py_TYPE(test_result_output_method) == nb::detail::internals->nb_method || 
                Py_TYPE(test_result_output_method) == nb::detail::internals->nb_func ||
                Py_TYPE(test_result_output_method) == nb::detail::internals->nb_bound_method) {
                Py_DECREF(test_result_output_method);
                invalidMethod(self, "test_result_output");
                return -1;
            }
            Py_DECREF(test_result_output_method);
    
            auto ptr = nb::detail::inst_ptr(inst);
            // Construct the object in place.
            new (ptr) FallibleTesterTrait_PyImpl(nb::handle(self));
    
            // We've now constructed, so we set to ready:
            inst->state = nb::detail::nb_inst::state_ready;
            return 0;
        }
    };
    
    nb::class_<somelib::FallibleTesterTrait> base(mod, "FallibleTesterTrait_PyBase", nb::sig("from abc import abstractmethod\nclass FallibleTesterTrait_PyBase()"));
    base.def("__init__", [](){
        PyErr_SetString(PyExc_TypeError, "Do not extend FallibleTesterTrait_PyBase, please extend FallibleTesterTrait.");
    });
    // Allocate slots for both __new__ and __init__, Nanobind does not allow you to bind both without using slots.
    PyType_Slot FallibleTesterTrait_PyImpl_slots[] = {
        {Py_tp_init, (void*) FallibleTesterTrait_PyImpl::init},
        {Py_tp_new, (void *) FallibleTesterTrait_PyImpl::new_},
        {0, nullptr}
    };
    nb::class_<FallibleTesterTrait_PyImpl, somelib::FallibleTesterTrait> tr(mod, "FallibleTesterTrait", nb::type_slots(FallibleTesterTrait_PyImpl_slots) );
    // Define abstract methods, for signature information.
    
    tr.def("test_void_trait_fn", [](){
        PyErr_SetString(PyExc_TypeError, "Abstract method test_void_trait_fn called. Please implement.");
    }, nb::sig("@abstractmethod\ndef test_void_trait_fn() -> None"));
    
    tr.def("test_result_output", [](){
        PyErr_SetString(PyExc_TypeError, "Abstract method test_result_output called. Please implement.");
    }, nb::sig("@abstractmethod\ndef test_result_output( x : int ) -> int"));
}

} 