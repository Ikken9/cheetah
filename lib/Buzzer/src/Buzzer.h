//
// Created by Ikken on 11/8/2023.
//

#ifndef ESP_TOOLS_BUZZER_H
#define ESP_TOOLS_BUZZER_H


class Buzzer {
public:
    Buzzer();
    void setup();
    void buzz();
    void buzz_intermittent();

    void enable_buzzer();
    void disable_buzzer();
};


#endif //ESP_TOOLS_BUZZER_H
