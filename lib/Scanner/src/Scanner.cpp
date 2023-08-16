//
// Created by Ikken on 9/8/2023.
//

#include "Scanner.h"

static const char *TAG = "WiFiScanner";

Scanner::Scanner() {};

void Scanner::scan_ap() {
    ESP_LOGI(TAG, "Scanning for Wi-Fi networks...");
    int32_t scan_status;
    scan_status = esp_wifi_scan_start(nullptr, true);
    if (scan_status != ESP_OK) {
        ESP_LOGE(TAG, "Scan start failed: %d", scan_status);
        return;
    }
}

void Scanner::scan_sta() {

}

void Scanner::wifi_scan_callback(void *arg, esp_event_base_t event_base, int32_t event_id, void *event_data) {
    if (event_id == WIFI_EVENT_SCAN_DONE) {
        uint16_t ap_count;
        esp_wifi_scan_get_ap_num(&ap_count);
        if (ap_count == 0) {
            ESP_LOGI(TAG, "No networks found");
            return;
        }

        auto *ap_list = (wifi_ap_record_t *)malloc(sizeof(wifi_ap_record_t) * ap_count);
        if (!ap_list) {
            ESP_LOGE(TAG, "Memory allocation failed");
            return;
        }

        esp_wifi_scan_get_ap_records(&ap_count, ap_list);
        for (int i = 0; i < ap_count; i++) {
            ESP_LOGI(TAG, "SSID: %s, RSSI: %d dBm", ap_list[i].ssid, ap_list[i].rssi);
            // You can access other fields like BSSID, channel, encryption type, etc. from ap_list[i]
        }

        free(ap_list);
    }
}

void Scanner::wifi_initialize() {
    tcpip_adapter_init();
    ESP_ERROR_CHECK(esp_event_loop_create_default());

    wifi_init_config_t cfg = WIFI_INIT_CONFIG_DEFAULT();
    ESP_ERROR_CHECK(esp_wifi_init(&cfg));
    ESP_ERROR_CHECK(esp_wifi_set_storage(WIFI_STORAGE_RAM));
    ESP_ERROR_CHECK(esp_wifi_set_mode(WIFI_MODE_STA));
    ESP_ERROR_CHECK(esp_wifi_start());

    ESP_ERROR_CHECK(esp_event_handler_register(WIFI_EVENT, ESP_EVENT_ANY_ID,
                                               &wifi_scan_callback, nullptr));
    ESP_ERROR_CHECK(esp_wifi_scan_start(nullptr, true));
}

