//
// Created by Ikken on 10/8/2023.
//

#ifndef CHEETAH_BATTERYINTERFACE_H
#define CHEETAH_BATTERYINTERFACE_H

#include <cstring>
#include <Wire.h>

#define IP5306I2C_ADDR      0x75
#define SDA                 42
#define SCL                 41

class BatteryInterface {
public:
    BatteryInterface();

    static uint8_t read(uint8_t reg_addr);
    static int8_t getBatteryCharge();

private:
    static void write(uint8_t reg_addr, uint8_t data);
};


#endif //CHEETAH_BATTERYINTERFACE_H
