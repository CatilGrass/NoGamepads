#ifndef PLAYER_FFI_H
#define PLAYER_FFI_H

#include <stdbool.h>
#include <stdint.h>

typedef struct FfiAccount {
    char* id;
    char* player_hash;
} FfiAccount;

typedef struct FfiCustomize {
    char* nickname;
    int32_t color_hue;
    double color_saturation;
    double color_value;
} FfiCustomize;

typedef struct FfiPlayer {
    FfiAccount account;
    FfiCustomize* customize;
} FfiPlayer;

#ifdef __cplusplus
extern "C" {
#endif

FfiPlayer* player_register(const char* id, const char* password);
bool player_check(const FfiPlayer* player, const char* password);
void player_set_nickname(FfiPlayer* player, const char* nickname);
void player_set_hue(FfiPlayer* player, int hue);
void player_set_hsv(FfiPlayer* player, int hue, double saturation, double value);
void player_free(FfiPlayer* player);

#ifdef __cplusplus
}
#endif

#endif