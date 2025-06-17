#include "include/nogamepads_data.h"
#include <stdio.h>
#include <stdlib.h>

#define USERNAME "juliet"
#define PASSWORD "12345678"
#define DISPLAY_NAME "IM JULIET"
#define SERVER_IP {127, 0, 0, 1}
#define SERVER_PORT 5989

int main(void) {

    // Create player
    FfiPlayer *player = player_register(USERNAME, PASSWORD);
    if (!player) {
        fprintf(stderr, "Failed to register player\n");
        return EXIT_FAILURE;
    }

    // Configure player attributes
    player_set_hsv(player, 120, 0.5, 0.9);
    player_set_nickname(player, DISPLAY_NAME);

    // Initialize controller
    FfiControllerData *controller = controller_data_new();
    if (!controller) {
        fprintf(stderr, "Failed to create controller\n");
        free_player(player);
        return EXIT_FAILURE;
    } else {
        controller_data_bind_player(controller, player);
    }

    // Create runtime
    FfiControllerRuntime *rt = controller_data_build_runtime(controller);
    if (!rt) {
        fprintf(stderr, "Failed to create runtime\n");
        free_player(player);
        free_controller_data(controller);
        return EXIT_FAILURE;
    }

    // Create TcpNetwork client
    FfiTcpClientService *client = tcp_client_build(rt);
    if (!client) {
        fprintf(stderr, "Failed to create client\n");
        free_controller_runtime(rt);
        free_controller_data(controller);
        free_player(player);
        return EXIT_FAILURE;
    } else {
        uint8_t ip[4] = SERVER_IP;
        tcp_client_bind_address_v4(client, ip[0], ip[1], ip[2], ip[3], SERVER_PORT);
    }

    enable_logger(0);

    // Connect to server and block
    tcp_client_connect(client);

    // Release memory
    free_controller_runtime(rt);
    free_controller_data(controller);
    free_player(player);

    return 0;
}