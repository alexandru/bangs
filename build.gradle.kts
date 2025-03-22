import org.jetbrains.kotlin.gradle.targets.js.webpack.KotlinWebpackConfig

plugins {
    alias(libs.plugins.kotlinMultiplatform) 
}

group = "org.alexn.bangs"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

kotlin {
    js(IR) {
        binaries.executable()
        browser {
            outputModuleName = "bangs"

            commonWebpackConfig {
                devServer = (devServer ?: KotlinWebpackConfig.DevServer()).apply {
                    static = (static ?: mutableListOf()).apply {
                        add(project.rootDir.path)
                    }
                }
                cssSupport {
                    enabled.set(false)
                }
            }

            testTask {
                useKarma {
                    useChromeHeadless()
                }
            }
        }
    }

    sourceSets {
        val jsMain by getting {
            dependencies {
                implementation(kotlin("stdlib-js"))
            }
        }
        val jsTest by getting {
            dependencies {
                implementation(kotlin("test-js"))
            }
        }
    }
}

tasks.register("generateBangs") {
    doLast {
        val generatedDir = File(
            project.layout.projectDirectory.asFile,
            "src/jsMain/kotlin"
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
