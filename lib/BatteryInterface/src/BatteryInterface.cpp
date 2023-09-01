//
// Created by Ikken on 10/8/2023.
//

#include <Arduino.h>
#include "BatteryInterface.h"

BatteryInterface::BatteryInterface() {
    Wire.begin(SDA, SCL);
}

void BatteryInterface::write(uint8_t reg_addr, uint8_t data) {
    Wire.beginTransmission(IP5306I2C_ADDR);
    Wire.write(reg_addr);
    Wire.write(data);
    Wire.endTransmission();
}

uint8_t BatteryInterface::read(uint8_t reg_addr) {
    uint8_t data;
    Wire.beginTransmission(IP5306I2C_ADDR);
    Wire.write(reg_addr);
    Wire.endTransmission();
    Wire.requestFrom(IP5306I2C_ADDR, 1);
    while (Wire.available()) {
        data = Wire.read();
    }
    return data;
}

int8_t BatteryInterface::getBatteryCharge() {
    switch (Wire.read() & 0xF0) {
        case 0xE0:
            return 25;
        case 0xC0:
            return 50;
        case 0x80:
            return 75;
        case 0x00:
            return 100;
        default:
            return 0;
    }
}

