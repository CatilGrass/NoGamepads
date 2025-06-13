#include <stdio.h>
#include "nogamepads_data.h"

int main(void) {
    const FfiPlayer *player = player_register("MyAccount", "Password123456");
    printf(player->account.id);
}