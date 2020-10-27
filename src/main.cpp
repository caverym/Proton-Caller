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
    ProtonObject.steam = "STEAM_COMPAT_DATA_PATH";
    ProtonObject.common = "PC_COMMON";
    std::cout << "Proton Caller by Avery Murray version: " << VERSION << "\n";

    // check for compat data path
    if (getenv(ProtonObject.steam) != nullptr) {
        std::cout<< ProtonObject.steam << " located at: " << getenv(ProtonObject.steam) << "\n";
    } else {mkdir(ProtonObject.steam);}

    Args(ProtonObject, argc, argv);
    setEnvironment(ProtonObject);
    ProtonObject.initPC();
    return 0;
}
