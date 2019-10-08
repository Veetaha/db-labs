#pragma once

#include <cstdlib>

namespace App::Env {
    
template <typename T>

    [[nodiscard]] T ReadVarOrAbort(const char* const varId) {
        const auto value { std::getenv(varId) };

        if (!value) {
            spdlog::critical("Environment variable \"{}\" is not defined", varId);
            std::abort();
        }

        try {   
            return boost::lexical_cast<T>(value);
        } catch (const boost::bad_lexical_cast& err) {
            spdlog::critical(
                "Failed while reading \"{}\" environment variable: {}", 
                varId, err.what()
            );
            std::abort();
        }
    }
}
