// 自动生成的头文件 - 请勿手动编辑

#ifndef NOGAMEPADS_H
#define NOGAMEPADS_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct FfiAccount {
  char *id;
  char *player_hash;
} FfiAccount;

typedef struct FfiCustomize {
  char *nickname;
  int color_hue;
  double color_saturation;
  double color_value;
} FfiCustomize;

typedef struct FfiPlayer {
  struct FfiAccount account;
  struct FfiCustomize *customize;
} FfiPlayer;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

ngp_ struct FfiPlayer *player_register(const char *id, const char *password);

ngp_ bool player_check(const struct FfiPlayer *player, const char *password);

ngp_ void player_set_nickname(struct FfiPlayer *player, const char *nickname);

ngp_ void player_set_hue(struct FfiPlayer *player, int hue);

ngp_ void player_set_hsv(struct FfiPlayer *player, int hue, double saturation, double value);

ngp_ void player_free(struct FfiPlayer *player);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* NOGAMEPADS_H */
