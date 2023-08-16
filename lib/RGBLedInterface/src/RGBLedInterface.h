//
// Created by Ikken on 10/8/2023.
//

#ifndef ESP_TOOLS_RGBLEDINTERFACE_H
#define ESP_TOOLS_RGBLEDINTERFACE_H

#include <Arduino.h>
#include <Adafruit_NeoPixel.h>
#include <esp_log.h>

#define LED_PIN                     38

#define MODE_OFF                    0
#define MODE_SCAN                   1
#define MODE_SNIFF                  2
#define MODE_ATTACK                 3
#define MODE_FIRMWARE_UPGRADE       4
#define MODE_SUB1                   5

extern Adafruit_NeoPixel strip;

class RGBLedInterface {
public:
    RGBLedInterface();
    void setup();
    uint8_t get_mode();
    void set_mode(uint8_t mode);

private:
    uint8_t current_mode;
    void set_mode_off();
    void set_mode_scan();
    void set_mode_sniff();
    void set_mode_attack();
    void set_mode_firmware_upgrade();
    void set_mode_sub1();
};


#endif //ESP_TOOLS_RGBLEDINTERFACE_H
