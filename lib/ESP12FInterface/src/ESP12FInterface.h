//
// Created by Ikken on 10/8/2023.
//

#ifndef ESP_TOOLS_ESP12FINTERFACE_H
#define ESP_TOOLS_ESP12FINTERFACE_H

#include "Arduino.h"

#define TXD     17 //   ESP32[TXD]      --> ESP12F[RXD]
#define RXD     18 //   ESP32[RXD]      --> ESP12F[TXD]
#define RST     7 //    ESP32[GPIO7]    --> ESP12F[RST]
#define ZERO    9 //    ESP32[GPIO9]    --> ESP12F[GPIO0]

class ESP12FInterface {
public:
    ESP12FInterface();
    void setup();

    void begin();
    void communicate();
    void reboot();

    void toggle_program_mode();
    void toggle_run_mode();

};


#endif //ESP_TOOLS_ESP12FINTERFACE_H
