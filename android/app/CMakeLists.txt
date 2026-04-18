# CMakeLists.txt stub
# Required by app/build.gradle externalNativeBuild block.
# This file does NOT compile any C/C++ code — its only job is to satisfy
# the Gradle build system's requirement for a CMake project, which is needed
# to trigger the -DANDROID_STL=c++_shared argument that bundles libc++_shared.so.
#
# The actual Rust compilation is handled by cargo-ndk in build_android.ps1.
# cargo-ndk outputs libvoidrift.so to android/app/src/main/jniLibs/arm64-v8a/
# Gradle picks it up from there during APK assembly.

cmake_minimum_required(VERSION 3.22.1)
project("voidrift_cmake_stub")

# Dummy library — exists only to satisfy the externalNativeBuild requirement.
# It will NOT be included in the final APK (excluded via packagingOptions in build.gradle).
add_library(dummy SHARED dummy.cpp)
