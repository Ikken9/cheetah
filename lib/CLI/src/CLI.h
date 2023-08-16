//
// Created by Ikken on 12/8/2023.
//

#ifndef ESP_TOOLS_CLI_H
#define ESP_TOOLS_CLI_H

#include "CC1101Interface.h"
#include "Deauther.h"
#include "Scanner.h"
#include "Sniffer.h"

// General
const char PROGMEM CMD_REBOOT[] = "reboot";
const char PROGMEM CMD_STOP[] = "stop";
const char PROGMEM CMD_SHOW_SETTINGS[] = "scfg";
const char PROGMEM CMD_SHOW_NETWORK_STATUS[] = "netstat";

// Discovery
const char PROGMEM CMD_SCAN_AP[] = "scanap";
const char PROGMEM CMD_SCAN_STA[] = "scansta";
const char PROGMEM CMD_SNIFF_BEACON[] = "sbeacon";
const char PROGMEM CMD_SNIFF_PMKID[] = "spmkid";

// Offensive
const char PROGMEM CMD_ATTACK_BEACON[] = "attbeacon";
const char PROGMEM CMD_ATTACK_DEAUTH[] = "attdeauth";
const char PROGMEM CMD_ATTACK_PROBE[] = "attprobe";

// Misc
const char PROGMEM CMD_CONNECT[] = "connect";

const char PROGMEM CMD_SELECT_CHANNEL[] = "sc";

const char PROGMEM CMD_LIST_AP[] = "listap";
const char PROGMEM CMD_LIST_STA[] = "liststa";
const char PROGMEM CMD_CLEAR_AP[] = "clearap";
const char PROGMEM CMD_CLEAR_STA[] = "clearsta";

const char PROGMEM CMD_SIGNAL_MONITOR[] = "sigmon";


class CLI {
public:
    CLI();

    // General
    void reboot();
    void stop();
    void show_settings();
    void show_network_status();

    // Discovery
    void scan_ap();
    void scan_sta();
    void sniff_beacon();
    void sniff_pmkid();

    // Offensive
    void attack_beacon();
    void attack_deauth();
    void attack_probe();

    // Misc
    void connect();
    void select_channel(uint8_t channel);

    void list_ap();
    void list_sta();
    void clear_ap();
    void clear_sta();

    void signal_monitor();

};


#endif //ESP_TOOLS_CLI_H
