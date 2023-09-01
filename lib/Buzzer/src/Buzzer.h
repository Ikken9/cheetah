//
// Created by Ikken on 11/8/2023.
//

#ifndef CHEETAH_BUZZER_H
#define CHEETAH_BUZZER_H

#include "Arduino.h"

#define BUZZER_PIN          8



class Buzzer {
public:
    Buzzer();

    static void buzz();

    static void enable_buzzer();
    static void disable_buzzer();
};


#endif //CHEETAH_BUZZER_H
