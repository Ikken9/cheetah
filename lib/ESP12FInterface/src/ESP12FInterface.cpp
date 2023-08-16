//
// Created by Ikken on 10/8/2023.
//

#include "ESP12FInterface.h"
#include <HardwareSerial.h>

HardwareSerial HardwareSerial(1);

ESP12FInterface::ESP12FInterface() {}

void ESP12FInterface::setup() {
    pinMode(RST, OUTPUT);
    pinMode(ZERO, OUTPUT);
}

void ESP12FInterface::begin() {
    Serial.begin(115200); // Serial Monitor
    HardwareSerial.begin(9600, SERIAL_8N1, RXD, TXD);
    delay(100);
    digitalWrite(ZERO, HIGH);
    delay(100);
    digitalWrite(RST, LOW);
    delay(100);
    digitalWrite(RST, HIGH);
    delay(100);
    digitalWrite(ZERO, HIGH);
}

void ESP12FInterface::reboot() {
    digitalWrite(RST, LOW);
    delay(100);
    digitalWrite(RST, HIGH);
    delay(100);
}

void ESP12FInterface::toggle_run_mode() {
    Serial.end();
    digitalWrite(ZERO, HIGH);
    delay(100);
    digitalWrite(RST, LOW);
    delay(100);
    digitalWrite(RST, HIGH);
    delay(100);
    digitalWrite(ZERO, HIGH);
    Serial.begin(115200);
}

void ESP12FInterface::toggle_program_mode() {
    Serial.end();
    digitalWrite(ZERO, LOW);
    delay(100);
    digitalWrite(RST, LOW);
    delay(100);
    digitalWrite(RST, HIGH);
    delay(100);
    digitalWrite(ZERO, HIGH);
    Serial.begin(57600);
}

void ESP12FInterface::communicate() {
    while (Serial.available()) {
        Serial.print((char)HardwareSerial.read());
    }

    if (Serial.available()) {
        HardwareSerial.write((uint8_t)Serial.read());
    }
}
