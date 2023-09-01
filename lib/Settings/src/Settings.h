//
// Created by Ikken on 30/8/2023.
//

#ifndef CHEETAH_SETTINGS_H
#define CHEETAH_SETTINGS_H

#include <ArduinoJson.h>
#include <SPIFFS.h>

class Settings {
public:
    Settings();

    struct SettingsMap;

    static void load_settings();

    static void update_setting(const char* property, const char* value);


};


#endif //CHEETAH_SETTINGS_H
