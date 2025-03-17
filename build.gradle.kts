import org.jetbrains.kotlin.gradle.targets.js.webpack.KotlinWebpackConfig

plugins {
    alias(libs.plugins.kotlinMultiplatform) 
}

group = "me.user"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

kotlin {
    wasmJs {
        binaries.executable()
        browser {
            commonWebpackConfig {
                devServer = (devServer ?: KotlinWebpackConfig.DevServer()).apply {
                    static = (static ?: mutableListOf()).apply {
                        add(project.rootDir.path)
                    }
                }
            }
        }
    }

    sourceSets {
        val commonMain by getting
        val commonTest by getting {
            dependencies {
                implementation(libs.kotlin.test)
            }
        }
        val wasmJsMain by getting {
            dependencies {
                implementation(libs.kotlinx.browser)
            }
        }
        val wasmJsTest by getting
    }
}

tasks.named("compileKotlinWasmJs") {
    dependsOn(tasks.named("generateBangs"))
}

tasks.register("generateBangs") {
    doLast {
        val generatedDir = File(
            project.layout.projectDirectory.asFile,
            "src/wasmJsMain/kotlin"
        )
        val outputFile = File(generatedDir, "data.kt")

        if (!generatedDir.exists()) {
            generatedDir.mkdirs()
        }

        val oneDayInMillis = 24 * 60 * 60 * 1000L
        val fileExistsAndIsRecent = outputFile.exists() &&
                (System.currentTimeMillis() - outputFile.lastModified() < oneDayInMillis)

        if (!fileExistsAndIsRecent) {
            exec {
                commandLine("node", "./scripts/gen-bangs.js")
                standardOutput = outputFile.outputStream()
            }
        }
    }
}
