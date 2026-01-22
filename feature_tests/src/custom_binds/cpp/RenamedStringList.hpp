extern "C" {
    void namespace_StringList_destroy(somelib::ns::capi::RenamedStringList* self);

    somelib::ns::capi::RenamedStringList* namespace_StringList_return_new(void);
}

namespace somelib::ns {
    std::vector<std::string> RenamedStringList::return_new() {
        somelib::ns::capi::RenamedStringList* self = namespace_StringList_return_new();

        auto ptr = (somelib::diplomat::capi::DiplomatStringView*) self;
        std::string arr = std::string(ptr->data, ptr->len);

        namespace_StringList_destroy(self);

        return std::vector<std::string>({arr});
    }
}