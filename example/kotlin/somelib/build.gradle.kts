plugins {
    kotlin("jvm") version "1.9.0"
    application
    `maven-publish`
    id("me.champeau.jmh") version "0.7.2"
}



group = "dev.diplomattest"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

// declare a "configuration" named "someConfiguration"
val someConfiguration by configurations.creating

dependencies {
    implementation("net.java.dev.jna:jna:5.14.0")
    testImplementation(kotlin("test"))
}
publishing {
    publications {
        create<MavenPublication>("maven") {
            groupId = "dev.diplomattest"
            artifactId = "somelib"
            version = "1.0-SNAPSHOT"

            from(components["java"])
        }
    }
}







tasks.test {
    useJUnitPlatform()
}

kotlin {
    jvmToolchain(17)
}

application {
}
jmh {
    iterations = 2 // Number of measurement iterations to do.
    batchSize =
        1 // Batch size: number of benchmark method calls per operation. (some benchmark modes can ignore this setting)
    fork = 2
    warmupBatchSize = 2 // Warmup batch size: number of benchmark method calls per operation.
    warmupIterations = 1 // Number of warmup iterations to do.
    timeOnIteration = "2s"
    warmup = "1s"
    benchmarkMode = listOf("avgt")
    timeUnit = "ns"
}