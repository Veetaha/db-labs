#include <cli/cliasyncsession.h>
#include <cli/cli.h>
#include <spdlog/spdlog.h>

namespace Cli = cli;
using namespace std;

int main() {
        // setup cli

    auto rootMenu { std::make_unique<Cli::Menu>("cli") };
    
    rootMenu->Insert(
        "hello", 
        [](std::ostream& out, std::string name){ 
            
        },
        "Print hello world" 
    )


    Cli::Cli cli{ std::move(rootMenu) };
    // global exit action
    cli.ExitAction([](auto& out){ 
        out << "Goodbye and thanks for all the fish.\n"; 
    });


    boost::asio::io_context ioCtx;

    Cli::CliAsyncSession session{ioCtx, cli};

    session.ExitAction(
        [&ioCtx](auto& out){
            out << "Closing App...\n";
            ioCtx.stop();
        }
    );

    ioCtx.run();

    return 0;
}
