#include "ui.h"
#include "env.h"
#include "controller.h"


#include <cli/cli.h>
#include <cli/remotecli.h>


int main() {
    const auto port { App::Env::ReadVarOrAbort<short>("PORT") };

    boost::asio::io_context asio;

    App::Controller controller;

    auto server { controller.Listen(asio, port) };

    server.ExitAction([&asio](std::ostream& os){
        os << "Closing the server...\n";
        asio.stop();
    });

    spdlog::info("Server is listening on port {}", port);
    return asio.run();
}
