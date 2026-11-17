cmake_minimum_required(VERSION 3.20)
project(autoime_windows_ime CXX)

add_library(autoime_ime SHARED
  dllmain.cpp
  classfactory.cpp
  textservice.cpp
  key_router.cpp
  diag.cpp
  edit_model.cpp
  deployment.cpp
  rime_engine.cpp
  candidate_window.cpp
  register.cpp
  autoime.rc
  autoime_ime.def
)
target_compile_features(autoime_ime PRIVATE cxx_std_17)
target_compile_definitions(autoime_ime PRIVATE _CRT_SECURE_NO_WARNINGS
                                            UNICODE _UNICODE)
target_link_libraries(autoime_ime PRIVATE uuid advapi32 user32)
set_target_properties(autoime_ime PROPERTIES OUTPUT_NAME "autoime_ime")

add_executable(verify_registered verify_registered.cpp)
target_compile_features(verify_registered PRIVATE cxx_std_17)
target_compile_definitions(verify_registered PRIVATE _CRT_SECURE_NO_WARNINGS)
target_link_libraries(verify_registered PRIVATE uuid ole32 user32 version)

# PLAN-070 T-01 automated fixture: replays the TSF callback shapes against the
# production KeyRouter and checks the diagnostics contract.
add_executable(autoime_fixture
  tests/key_router_test.cpp
  key_router.cpp
  diag.cpp
  edit_model.cpp
)
target_compile_features(autoime_fixture PRIVATE cxx_std_17)
target_compile_definitions(autoime_fixture PRIVATE _CRT_SECURE_NO_WARNINGS
                                                UNICODE _UNICODE)
target_link_libraries(autoime_fixture PRIVATE user32)

# PLAN-070 T-04 deployment CLI: synchronous deploy + schema validation with
# rollback/fallback, first-deploy timing and key-to-candidate latency.
add_executable(autoime_deploy
  tools/deploy_main.cpp
  deployment.cpp
  diag.cpp
  edit_model.cpp
  rime_engine.cpp
)
target_compile_features(autoime_deploy PRIVATE cxx_std_17)
target_compile_definitions(autoime_deploy PRIVATE _CRT_SECURE_NO_WARNINGS
                                               UNICODE _UNICODE)
target_link_libraries(autoime_deploy PRIVATE advapi32 user32)

if(MSVC)
  target_compile_options(autoime_ime PRIVATE /W4 /permissive- /EHsc)
  target_compile_options(verify_registered PRIVATE /W4 /permissive- /EHsc /utf-8)
  target_compile_options(autoime_fixture PRIVATE /W4 /permissive- /EHsc /utf-8)
  target_compile_options(autoime_deploy PRIVATE /W4 /permissive- /EHsc /utf-8)
endif()
