# Kotlin Backend
The kotlin backend uses JNA to wrap the ABI types from diplomat into an ergonomic library for use in the JVM.
The backend ensures safe sharing and disposal of memory, but it is still experimental so bugs may exist.

To run the kotlin backend you need to provide some configuration
```sh
diplomat-tool -e {PATH_TO_LIB.RS} -c {CONFIG_FILE} --config {CONFIG_OVERRIDE_1} --config {CONFIG_OVERRIDE_2} kotlin {OUTPUT_PATH}
```
The configuration consists of three parts
* `lib-name` - the name of the library. This can be different to the rust library name, but native access will 
  look for a file with the name `lib{lib-name}.so` (linux) or `lib{lib-name}.dylib` (macos) or `{lib-name}.dll` (windows)
  If you include this in `src/main/resources` then the library should be packaged automatically when you publish 
  the kotlin library. With the appropriate directory structure you can include binaries for multple operating 
  systems and architectures.
* `kotlin.domain` - this is the group of the package, e.g. `io.apache`.
* `kotlin.use_finalizers_not_cleaners` - An optional binary value. By default kotlin uses the Cleaner API from 
  Java 9 onwards for cleaning up the native memory alloated by the diplomat library. It can, however, be 
  configured to use the finalizer api from Java 8 and prior.
* `kotlin.scaffold` - an optional binary value. If it is set to `true`, `diplomat-tool` will scaffold a project
  for you with configuration corresponding to your group and library name.

Diplomat always create the following structure in the `OUTPUT_PATH`
```
├── src 
│   └── main
│       └── kotlin
│           └──{group-tld}
│               └──{group-name}
│                   └──{lib-name} 
│                       ├── GeneratedFile.kt
│                       ├── ...
│                       ├── Lib.kt
│                       ├── ...
```
The `Lib.kt` file includes some utility methods that the library uses, as well as a dedicated cleaner. 
If you choose `scaffold` then it will also generate a couple of config files
```
├── src 
│   └── ...
├── build.gradle.kts
├── setttings.gradle.kts
```
## Examples
The best way to learn to use the kotlin backend is first understand diplomat generally by reading this [book](../SUMMARY.md).
Then look at the `example` and `feature_tests` directories in the diplomat project. Then you can look at the tests
for how to use them
* Feature tests: [rust source](https://github.com/rust-diplomat/diplomat/tree/main/feature_tests/src/), [kotlin usage](https://github.com/rust-diplomat/diplomat/tree/main/feature_tests/kotlin/somelib/src/test/kotlin/dev/diplomattest/somelib)
* Example: [rust source](https://github.com/rust-diplomat/diplomat/tree/main/example/src/), [kotlin usage](https://github.com/rust-diplomat/diplomat/tree/main/example/kotlin/somelib/src/test/kotlin/dev/diplomattest/somelib)
### Callbacks
The kotlin backend supports passing kotlin functions and closures as callbacks to native code. To do this you need to add the 
enable the `jvm-callback-support` feature when you add  the diplomat runtime to your project. For examples on how to use them
look in the `feature_tests` directory of the diplomat project. This is a bit conceptually different to the rest of diplomat
so here some specific examples
* [rust code](https://github.com/rust-diplomat/diplomat/blob/main/feature_tests/src/callbacks.rs)
* [generated kotlin code](https://github.com/rust-diplomat/diplomat/blob/main/feature_tests/kotlin/somelib/src/main/kotlin/dev/diplomattest/somelib/CallbackWrapper.kt)
* [example usage](https://github.com/rust-diplomat/diplomat/blob/main/feature_tests/kotlin/somelib/src/test/kotlin/dev/diplomattest/somelib/CallbackWrapperTest.kt)
