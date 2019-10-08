#include "ui.h"

namespace App {

    Ui::Ui() {
        cli::SetColor();
        m_Cli.ExitAction([](std::ostream& out){ 
            out << "Exited console interface.\n"; 
        });
    }

    std::unique_ptr<cli::Menu> Ui::CreateRootMenu() {
        auto rootMenu { std::make_unique<cli::Menu>("lab2", "This is the database lab2 menu.") };

        rootMenu->Insert("Hello",
            [](std::ostream& os) {
                os << "hello";
            },
            "Description"
        );
        return rootMenu;
    }


}
