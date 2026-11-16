cmake_minimum_required(VERSION 3.20)
project(autoime_windows_probe CXX)

add_executable(rime_probe rime_probe.cpp)
target_compile_features(rime_probe PRIVATE cxx_std_17)
target_compile_definitions(rime_probe PRIVATE _CRT_SECURE_NO_WARNINGS)
target_link_libraries(rime_probe PRIVATE advapi32 version)

if(MSVC)
  target_compile_options(rime_probe PRIVATE /W4 /permissive- /utf-8 /EHsc)
endif()
