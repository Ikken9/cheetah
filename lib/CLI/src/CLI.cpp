//
// Created by Ikken on 12/8/2023.
//

#include "CLI.h"
#include "RGBLedInterface.h"
#include "WiFiUtils.h"
#include "Buzzer.h"

extern RGBLedInterface RGBLedInterface;
extern WiFiUtils WiFiUtils;
extern Buzzer Buzzer;

CLI::CLI() {}

void CLI::reboot() {

}

void CLI::stop() {

}

void CLI::show_settings() {

}

void CLI::show_network_status() {

}

void CLI::scan_ap() {
    RGBLedInterface.set_mode(MODE_SCAN);
}

void CLI::scan_sta() {
    RGBLedInterface.set_mode(MODE_SCAN);
}

void CLI::sniff_beacon() {
    RGBLedInterface.set_mode(MODE_SNIFF);
}

void CLI::sniff_pmkid() {
    RGBLedInterface.set_mode(MODE_SNIFF);
}

void CLI::attack_beacon() {
    RGBLedInterface.set_mode(MODE_ATTACK);
}

void CLI::attack_deauth() {
    RGBLedInterface.set_mode(MODE_ATTACK);
}

void CLI::attack_probe() {
    RGBLedInterface.set_mode(MODE_ATTACK);
}

void CLI::connect() {

}

void CLI::select_channel(uint8_t channel) {
    WiFiUtils.select_channel(channel);
}

void CLI::list_ap() {
    WiFiUtils.list_ap();
}

void CLI::list_sta() {
    WiFiUtils.list_sta();
}

void CLI::clear_ap() {
    WiFiUtils.clear_ap();
}

void CLI::clear_sta() {
    WiFiUtils.clear_sta();
}

void CLI::signal_monitor() {
    WiFiUtils.sigmon();
}
