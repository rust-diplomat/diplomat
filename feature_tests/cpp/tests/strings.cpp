#include <iostream>
#include "../include/Opaque.hpp"
#include "../include/OptionString.hpp"
#include "../include/MyString.hpp"
#include "assert.hpp"

using namespace somelib;

int main(int argc, char* argv[]) {
    std::unique_ptr<Opaque> o = Opaque::from_str("hello world").ok().value();
    std::string output = o->get_debug_str();
    simple_assert_eq("simple string get",  output, "\"hello world\"");
    output = "prefix ";
    o->get_debug_str_write(output);
    simple_assert_eq("string write", output, "prefix \"hello world\"");


    std::unique_ptr<OptionString> os = OptionString::new_("hello world");
    output = os->write().ok().value();
    simple_assert_eq("simple string get with result", output, "hello world");
    output = "prefix ";
    os->write_write(output).ok().value();
    simple_assert_eq("string write with result", output, "prefix hello world");

    std::unique_ptr<MyString> s = MyString::new_owned("ABC");
    simple_assert_eq("Owned string pass in", s->get_str(), "ABC");
    s->set_str("123");
    simple_assert_eq("Owned string pass out", s->get_str(), "123");
}
