plugins {
    kotlin("jvm") version "1.9.0"
    application
    `maven-publish`
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
    testImplementation("org.junit.jupiter:junit-jupiter:5.9.2")
    testRuntimeOnly("org.junit.platform:junit-platform-launcher")
    testImplementation("org.mockito:mockito-core:5.8.0")
    testImplementation("org.mockito:mockito-junit-jupiter:5.8.0")
    testImplementation("org.mockito.kotlin:mockito-kotlin:5.2.1")
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