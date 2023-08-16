//
// Created by Ikken on 9/8/2023.
//

#include <WiFi.h>
#include <esp_wifi.h>
#include <esp_log.h>
#include <cstring>

#ifndef ESP_TOOLS_SCAN_H
#define ESP_TOOLS_SCAN_H


class Scanner {
public:
    Scanner();
    void scan_ap();
    void scan_sta();

private:
    static void wifi_scan_callback(void *arg, esp_event_base_t event_base, int32_t event_id, void *event_data);

    static void wifi_initialize();

};

#endif //ESP_TOOLS_SCAN_H