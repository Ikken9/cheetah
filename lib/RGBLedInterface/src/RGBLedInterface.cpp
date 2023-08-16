//
// Created by Ikken on 10/8/2023.
//

#include "RGBLedInterface.h"

static const char *TAG = "RGBLedInterface";

RGBLedInterface::RGBLedInterface() {
    current_mode = MODE_OFF;
}

void RGBLedInterface::setup() {
    if (current_mode != MODE_OFF) {
        strip.begin();
        set_mode_off();
    }
}

uint8_t RGBLedInterface::get_mode() {
    return this->current_mode;
}

void RGBLedInterface::set_mode(uint8_t mode) {
    this->current_mode = mode;
    switch (mode) {
        case 0:
            set_mode_off();
        case 1:
            set_mode_scan();
        case 2:
            set_mode_sniff();
        case 3:
            set_mode_attack();
        case 4:
            set_mode_firmware_upgrade();
        case 5:
            set_mode_sub1();
        default:
            ESP_LOGE(TAG, "Unable to set mode");
    }
}

void RGBLedInterface::set_mode_off() {
    this->current_mode = MODE_OFF;
    strip.setBrightness(0);
    strip.setPixelColor(0, Adafruit_NeoPixel::Color(0,0,0));
    strip.show();
}

void RGBLedInterface::set_mode_scan() {
    this->current_mode = MODE_SCAN;
    strip.setBrightness(50);
    strip.setPixelColor(0, Adafruit_NeoPixel::Color(255,255,0));
    strip.show();
}

void RGBLedInterface::set_mode_sniff() {
    this->current_mode = MODE_SNIFF;
    strip.setBrightness(50);
    strip.setPixelColor(0, Adafruit_NeoPixel::Color(255,0,255));
    strip.show();
}

void RGBLedInterface::set_mode_attack() {
    this->current_mode = MODE_ATTACK;
    strip.setBrightness(50);
    strip.setPixelColor(0, Adafruit_NeoPixel::Color(255,0,0));
    strip.show();
}

void RGBLedInterface::set_mode_firmware_upgrade() {
    this->current_mode = MODE_FIRMWARE_UPGRADE;
    strip.setBrightness(50);
    strip.setPixelColor(0, Adafruit_NeoPixel::Color(0,0,255));
    strip.show();
}

void RGBLedInterface::set_mode_sub1() {
    this->current_mode = MODE_SUB1;
    strip.setBrightness(50);
    strip.setPixelColor(0, Adafruit_NeoPixel::Color(0,255,0));
    strip.show();
}