#pragma once

#include "ui.h"

#include <cli/cli.h>
#include <cli/remotecli.h>

namespace App {

    class Controller {
        Ui ui;

    public:

        Controller() {}


        inline auto Listen(boost::asio::io_context& asio, short port) {
            return cli::CliTelnetServer{ asio, port, ui.GetCli() };
        }

    };


}
