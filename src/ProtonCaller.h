//
// Created by avery on 15/10/2020.
//

#ifndef PROTON_CALLER_PROTONCALLER_H
#define PROTON_CALLER_PROTONCALLER_H

#include <iostream>
#include <string>
#include <cstring>
#include <filesystem>
#include <unistd.h>

#define VERSION "1.3.0"
#define STEAM "STEAM_COMPAT_DATA_PATH"

class ProtonClass {
public:
    std::string _argv1;
    std::string _argv2;
    std::string _argv3;
    const char *common{};
    std::string proton;
    std::string program;
    std::string proton_path;
    bool custom{};

    void initPC() const {
        check();
        protonCall();
    }

    // main environment checks
    void check() const {
        if (custom){std::cout<<"custom mode does not check for Proton directory.\n";return;}
        namespace fs = std::filesystem;
        std::string prtn = "Proton ";
        std::string tmpPath = common + prtn + proton + "/";
        char chkPath[50];
        strcpy(chkPath, tmpPath.c_str());
        if(fs::is_directory(chkPath)){std::cout << proton << " exists.\n";return;}
        else {std::cout << proton << " does not exist.\n";exit(EXIT_FAILURE);}
    }

    // the execution of Proton itself
    void protonCall() const {
        std::string exec_ = proton_path + proton + "/proton";
        char cmd[100];
        char win[50];
        strcpy(win, program.c_str());
        strcpy(cmd, exec_.c_str());
        execl(cmd,"proton", "run", win, NULL);
    }
};

void Args(ProtonClass &ProtonObject, int argc, char *argv[]);

void setEnvironment(ProtonClass &ProtonObject);

const char* findCommon(const char *cCommon);

#endif //PROTON_CALLER_PROTONCALLER_H
