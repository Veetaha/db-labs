#pragma once

#include "generate-getters.h"


#include <cli/cli.h>

namespace App {

    class Ui {
        cli::Cli m_Cli{ CreateRootMenu() };

    public:

        Ui();

        APP_GENERATE_LVAL_GETTERS(constexpr, GetCli(), m_Cli)

        static std::unique_ptr<cli::Menu> CreateRootMenu();

    };
}
