//
// Created by avery on 27/10/2020.
//

#include "ProtonCaller.h"
#include "ProtonArguments.h"

int main(int argc, char *argv[]) {
    std::cout << "Proton Caller by Avery Murray version: " << VERSION << ".\n";
    ProtonCaller proObj;
    Args(proObj, argc, argv);
    setEnvironment(proObj);
    proObj.check();
    proObj.protonCall();
    return 0;
}
