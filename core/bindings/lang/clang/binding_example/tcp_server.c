#include <stdio.h>
#include <pthread.h>
#include "include/nogamepads_data.h"

void* server_thread(void* arg) {
    FfiTcpServerService *server = arg;
    tcp_server_listening_block_on(server);

    printf("Server stopped. \n");
    return NULL;
}

int main(void) {

    printf("//////////////////////////////////// \n");
    printf("///// NoGamepads C Binding ///////// \n");
    printf("///// Example: Start a server. ///// \n");
    printf("//////////////////////////////////// \n");
    printf("\n");

    FfiGameData *data = game_data_new();
    printf("Data created. \n");

    FfiGameRuntime *rt = game_data_build_runtime(data);
    printf("Runtime created. \n");

    FfiTcpServerService *server = tcp_server_build(rt);
    printf("Tcp services created. \n");

    tcp_server_bind_address_v4(server, 127, 0, 0, 1, 5989);
    printf("Address bind. \n");

    pthread_t thread;
    pthread_create(&thread, NULL, server_thread, server);

    printf("Server started. Press Enter to stop...\n");

    getchar();

    printf("Process stopped. \n");

    return 0;
}