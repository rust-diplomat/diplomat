namespace nanobind::detail {
    template<>
    struct type_caster<std::unique_ptr<somelib::ns::RenamedStringList>>
    {
        Py_ssize_t size;
        using Caster = list_caster<std::vector<std::string>, std::string>;
        static constexpr auto Name = Caster::Name;

        NB_INLINE bool can_cast() const noexcept { return true; }

        static handle from_cpp(std::unique_ptr<somelib::ns::RenamedStringList> value, rv_policy p, cleanup_list* cl) noexcept {
            somelib::ns::RenamedStringList* val = value.release();
            auto ptr = (somelib::diplomat::capi::DiplomatStringView*) val;
            std::string test = std::string(ptr->data, ptr->len);

            somelib::ns::capi::namespace_StringList_destroy((somelib::ns::capi::RenamedStringList*)val);

            std::vector<std::string> vec = {test};
            for (char c : test) {
                vec.push_back(std::string{c});
            }
            return Caster::from_cpp(vec, p, cl);
        }

        bool from_python(handle src, uint8_t flags, cleanup_list* cl) noexcept {
            return false;
        }
    };
}