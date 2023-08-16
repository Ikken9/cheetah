//
// Created by Ikken on 9/8/2023.
//

#ifndef ESP_TOOLS_DEAUTHER_H
#define ESP_TOOLS_DEAUTHER_H


#include <esp_wifi.h>
#include <esp_log.h>
#include <cstring>
#include <WiFi.h>

class Deauther {
public:
    Deauther();

    void deauth(const char *targetBSSID, int count);

private:
    void send_deauth_packet(const uint8_t *targetMAC);
};

#endif //ESP_TOOLS_DEAUTHER_H
