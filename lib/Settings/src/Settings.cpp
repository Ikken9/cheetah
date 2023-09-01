//
// Created by Ikken on 30/8/2023.
//

#include "Settings.h"

Settings::Settings() {

}

struct Settings::SettingsMap {
    char deviceName[32];
    int operatingChannel;
};

void Settings::load_settings() {
    File settingsFile = SPIFFS.open("/settings.json", "r");
    if (!settingsFile) {
        Serial.println("Failed to open config file");
        return;
    }

    size_t size = settingsFile.size();
    std::unique_ptr<char[]> buf(new char[size]);
    settingsFile.readBytes(buf.get(), size);
    settingsFile.close();

    SettingsMap settings;
    deserializeJson((JsonDocument &) settings, buf.get());
}

void Settings::update_setting(const char *property, const char *value) {
    if (!SPIFFS.begin()) {
        Serial.println("Failed to mount SPIFFS");
        return;
    }

    File settingsFile = SPIFFS.open("/config.json", "r");
    if (!settingsFile) {
        Serial.println("Failed to open config file");
        return;
    }

    size_t size = settingsFile.size();
    std::unique_ptr<char[]> buf(new char[size]);
    settingsFile.readBytes(buf.get(), size);
    settingsFile.close();

    DynamicJsonDocument doc(1024);  // Adjust the buffer size as needed
    DeserializationError error = deserializeJson(doc, buf.get());

    if (error) {
        Serial.println("Failed to parse JSON");
        return;
    }

    const char* newPropertyValue = value;  // Replace with your new value
    doc[property] = newPropertyValue;

    File configFileWrite = SPIFFS.open("/config.json", "w");
    if (!configFileWrite) {
        Serial.println("Failed to open config file for writing");
        return;
    }

    serializeJson(doc, configFileWrite);
    configFileWrite.close();

    Serial.println("Property updated successfully");
}