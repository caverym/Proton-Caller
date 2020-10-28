//
// Created by avery on 27/10/2020.
//

#ifndef PROTON_CALLER_PROTONARGUMENTS_H
#define PROTON_CALLER_PROTONARGUMENTS_H

#include "ProtonCaller.h"
#define COMMON "PC_COMMON"

void Args(ProtonCaller &proObj, int argc, char *argv[]);

void setEnvironment(ProtonCaller &proObj);

const char* findCommon();

#endif //PROTON_CALLER_PROTONARGUMENTS_H
