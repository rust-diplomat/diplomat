extern "C" {
    void namespace_StringList_destroy(somelib::ns::capi::RenamedStringList* self);

    somelib::ns::capi::RenamedStringList* namespace_StringList_return_new(void);
}

namespace somelib::ns {
    // Returns std::vector<std::string> (nanobind already knows how to cast that) rather than the
    // opaque wrapper class itself: nanobind doesn't allow a bound class and a type_caster for the
    // exact same C++ type to coexist, so the class-caster route the old std::unique_ptr-keyed
    // caster used pre-rework isn't available anymore.
    std::vector<std::string> RenamedStringList::return_new() {
        somelib::ns::capi::RenamedStringList* self = namespace_StringList_return_new();

        auto ptr = (somelib::diplomat::capi::DiplomatStringView*) self;
        std::string test = std::string(ptr->data, ptr->len);

        namespace_StringList_destroy(self);

        std::vector<std::string> vec = {test};
        for (char c : test) {
            vec.push_back(std::string{c});
        }
        return vec;
    }
}
