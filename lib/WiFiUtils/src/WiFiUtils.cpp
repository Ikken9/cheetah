//
// Created by Ikken on 9/8/2023.
//

#include <WiFi.h>
#include "esp_wifi.h"
#include "WiFiUtils.h"


WiFiUtils::WiFiUtils() {}

void WiFiUtils::sigmon() {

}

void WiFiUtils::select_channel(uint8_t channel) {

}

void WiFiUtils::list_ap() {

}

void WiFiUtils::list_sta() {

}

void WiFiUtils::clear_ap() {

}

void WiFiUtils::clear_sta() {

}

void WiFiUtils::initialize_wifi() {

}

void WiFiUtils::shutdown_wifi() {
    WiFi.disconnect();
    WiFiClass::mode(WIFI_OFF);

    esp_wifi_set_mode(WIFI_MODE_NULL);
    esp_wifi_stop();
    esp_wifi_restore();
    esp_wifi_deinit();
}
