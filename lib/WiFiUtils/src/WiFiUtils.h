//
// Created by Ikken on 9/8/2023.
//

#ifndef ESP_TOOLS_WIFIUTILS_H
#define ESP_TOOLS_WIFIUTILS_H


class WiFiUtils {
public:
    WiFiUtils();
    void select_channel(uint8_t);

    void list_ap();
    void list_sta();
    void clear_ap();
    void clear_sta();

    void sigmon();
    void initialize_wifi();
    void shutdown_wifi();

};


#endif //ESP_TOOLS_WIFIUTILS_H
