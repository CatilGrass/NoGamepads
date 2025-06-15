#include <stdio.h>
#include "include/nogamepads_data.h"

int main(void) {

    printf("//////////////////////////////////////// \n");
    printf("///// NoGamepads C Binding ///////////// \n");
    printf("///// Example: Connect to a server ///// \n");
    printf("//////////////////////////////////////// \n");
    printf("\n");

    FfiPlayer *player = player_register("juliet", "12345678");
    player_set_hsv(player, 120, 0.5, 0.9);
    player_set_nickname(player, "IM JULIET");
    printf("Player created. \n");

    FfiControllerData *controller = controller_data_new();
    controller_data_bind_player(controller, player);
    printf("Controller data created. \n");

    FfiControllerRuntime *rt = controller_data_build_runtime(controller);
    printf("Runtime created. \n");

    FfiTcpClientService *client = tcp_client_build(rt);
    printf("Tcp client built. \n");

    tcp_client_bind_address_v4(client, 127, 0, 0, 1, 5989);
    printf("Address bind. \n");

    printf("Connecting \n");
    tcp_client_connect(client);
    printf("Disconnected \n");

    return 0;
}
