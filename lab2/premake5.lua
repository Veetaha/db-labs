workspace "lab2"
    configurations { "Debug", "Release" }
    startproject "lab2"

project "lab2"
    language "C++"
    kind     "ConsoleApp"

    targetname "lab2.out"
    location   "build"
    targetdir  "build/bin"
    objdir     "build/bin-int"

    pchheader "pch.h"
    pchsource "src/pch.cpp"
    forceincludes { "%{prj.pchheader}" }

    files { "src/**.cpp" }

    includedirs {
        "include",
        "vendor/conan/modules/include",
        "vendor/submodules/*/include"
    }

    libdirs {
        "vendor/conan/modules/lib",
        "vendor/submodules/fmt/build",
        "vendor/submodules/spdlog/build"
    }

    links {
        "boost_system",
        "boost_thread",
        "pthread",
        "fmt",
        "spdlog"
    }

    buildoptions {
        "-std=gnu++2a", -- workaround: `cppdialect "gnu++2a"` is not allowed
        "-pedantic"
    }

    enablewarnings {
        "error",
        "all",
        "extra"
    }

    filter "configurations:Debug"
        runtime "Debug"
        defines { "DEBUG" }
        symbols "On"

    filter "configurations:Release"
        runtime  "Release"
        defines  { "NDEBUG" }
        optimize "On"
