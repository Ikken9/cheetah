//
// Created by Ikken on 11/8/2023.
//

#include "Buzzer.h"

Buzzer::Buzzer() {}

void Buzzer::buzz() {
    digitalWrite(BUZZER_PIN, HIGH);
    delay(500);
    digitalWrite(BUZZER_PIN, LOW);
    delay(500);
}

void Buzzer::enable_buzzer() {
    pinMode(BUZZER_PIN, OUTPUT);
}

void Buzzer::disable_buzzer() {
    pinMode(BUZZER_PIN, INPUT);
}