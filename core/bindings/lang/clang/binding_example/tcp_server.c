#include "include/nogamepads_data.h"
#include <stdio.h>
#include <stdlib.h>

#define GAME_NAME "My Hero"
#define GAME_VERSION "0.1.0"
#define SERVER_IP {127, 0, 0, 1}
#define SERVER_PORT 5989

int main(void) {

    // Game data initialization
    FfiGameData *data = game_data_new();
    if (!data) {
        fprintf(stderr, "Failed to create game data\n");
        return EXIT_FAILURE;
    }

    // Configure game information
    game_data_set_name_info(data, GAME_NAME);
    game_data_set_version_info(data, GAME_VERSION);

    // Create runtime
    FfiGameRuntime *rt = game_data_build_runtime(data);
    if (!rt) {
        fprintf(stderr, "Failed to create runtime\n");
        free_game_data(data);
        return EXIT_FAILURE;
    }

    // Start the server
    FfiTcpServerService *server = tcp_server_build(rt);
    if (!server) {
        fprintf(stderr, "Failed to create server\n");
        free_game_runtime(rt);
        free_game_data(data);
        return EXIT_FAILURE;
    }

    // Bind address
    uint8_t ip[4] = SERVER_IP;
    tcp_server_bind_address_v4(server, ip[0], ip[1], ip[2], ip[3], SERVER_PORT);

    printf("Server started at %d.%d.%d.%d:%d\n", ip[0], ip[1], ip[2], ip[3], SERVER_PORT);

    enable_logger(2);

    // Start server listening
    tcp_server_listening_block_on(server);

    printf("Server closed\n");

    // Release memory
    free_game_runtime(rt);
    free_game_data(data);
    return 0;
}