#include <iostream>
#include <string>
#include "ProtonCaller.h"

void mkdir(const char *pPath) {
    std::string _proton = "/proton";
    std::string dir = getenv("HOME") + _proton;
    std::filesystem::create_directory(dir);
    std::cout << "Add 'export " << pPath << "=$HOME/proton' to your profile and run again\n";
    exit(EXIT_FAILURE);
}

int main(int argc, char *argv[]) {
    ProtonClass ProtonObject;
    ProtonObject.common = "PC_COMMON";
    std::cout << "Proton Caller by Avery Murray version: " << VERSION << "\n";
    Args(ProtonObject, argc, argv);
    // check for compat data path
    if (getenv(STEAM) != nullptr) {
        std::cout<< STEAM << " located at: " << getenv(STEAM) << "\n";
    } else {
        std::cout << STEAM << "does not exist.\n";
    }
    setEnvironment(ProtonObject);
    ProtonObject.initPC();
    return 0;
}
