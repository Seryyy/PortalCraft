plugins {
    id("fabric-loom") version "1.6.1"
    id("java")
}

group = "com.seryyy"
version = "0.1.0"
java.toolchain.languageVersion.set(JavaLanguageVersion.of(21))

repositories {
    mavenCentral()
    maven { url = uri("https://maven.fabricmc.net") }
    maven { url = uri("https://maven.minecraftforge.net") }
}

dependencies {
    minecraft "com.mojang:minecraft:1.21.1"
    mappings "net.fabricmc:yarn:1.21.1+build.1@v1"
    modImplementation "net.fabricmc:fabric-loader:0.16.9"
    modImplementation "net.fabricmc.fabric-api:fabric-api:0.108.0+1.21.1"
    modImplementation "com.google.code.gson:gson:2.10.1"
    implementation "org.jetbrains.kotlin:kotlin-stdlib-jdk8:1.9.24"
}

loom {
    mappingsVersion.set("1.21.1+build.1")
    remapJar.set(true)
}

tasks.withType<JavaCompile> {
    options.encoding = "UTF-8"
}

jar {
    manifest {
        attributes(
            "Fabric-Mod-ID" to "portalcraft",
            "Fabric-Mod-Name" to "PortalCraft",
            "Fabric-Mod-Version" to version,
            "Fabric-Mod-Authors" to "Seryyy",
            "Fabric-Mod-Description" to "Minecraft side of PortalCraft - syncs blocks/inventory with Portal 2",
            "Fabric-Mod-Entrypoints" to "{\"main\":[\"com.seryyy.portalcraft.PortalCraftMod\"]}"
        )
    }
}