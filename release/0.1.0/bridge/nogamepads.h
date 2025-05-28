/* NoGamepads C Bindings. */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum CtrlMsgCTag {
  Msg,
  Pressed,
  Released,
  Axis,
  Dir,
  Exit,
  Err,
} CtrlMsgCTag;

typedef enum GameMsgCTag {
  SkinEventTrigger,
  DisableKey,
  EnableKey,
  Leave,
  Err,
} GameMsgCTag;

typedef enum LeaveReasonData {
  GameOver,
  ServerClosed,
  YouAreKicked,
  YouAreBanned,
} LeaveReasonData;

typedef struct KeyData {
  uint8_t key;
} KeyData;

typedef struct AxisData {
  uint8_t key;
  double ax;
} AxisData;

typedef struct DirData {
  uint8_t key;
  double x;
  double y;
} DirData;

typedef struct StrData {
  const char *str;
} StrData;

typedef union CtrlMsgCUnion {
  void *nul;
  struct KeyData key;
  struct AxisData axis;
  struct DirData dir;
  struct StrData str;
} CtrlMsgCUnion;

typedef struct CtrlMsgC {
  enum CtrlMsgCTag tag;
  union CtrlMsgCUnion data;
} CtrlMsgC;

typedef union GameMsgCUnion {
  void *nul;
  struct KeyData key;
  enum LeaveReasonData reason;
} GameMsgCUnion;

typedef struct GameMsgC {
  enum GameMsgCTag tag;
  union GameMsgCUnion data;
} GameMsgC;

typedef struct PlayerList {
  PlayerInfo *players;
  uintptr_t len;
  uintptr_t capacity;
} PlayerList;

GameProfile *init_game_profile(void);

void set_game_profile_name(GameProfile *game_profile, const char *value);

void set_game_profile_description(GameProfile *game_profile, const char *value);

void set_game_profile_organization(GameProfile *game_profile, const char *value);

void set_game_profile_version(GameProfile *game_profile, const char *value);

void set_game_profile_website(GameProfile *game_profile, const char *value);

void set_game_profile_email(GameProfile *game_profile, const char *value);

PlayerInfo *init_player_info(void);

void set_player_info_account(PlayerInfo *info, const char *id, const char *password);

void set_player_info_color(PlayerInfo *info, int32_t h, double s, double v);

void set_player_info_nickname(PlayerInfo *info, const char *nickname);

uint16_t get_default_port(void);

PadClient *init_client(const char *address);

PadClient *init_client_with_port(const char *address, uint16_t port);

void set_client_quiet(PadClient *client);

void enable_client_console(PadClient *client);

void bind_player_to_client(PadClient *client, PlayerInfo *info);

void connect_client_to_server(PadClient *client);

void exit_client_from_server(const PadClient *client);

void put_a_msg_to_server(const PadClient *client, struct CtrlMsgC msg);

struct GameMsgC pop_a_msg_from_server(const PadClient *client);

struct GameMsgC pop_msg_from_server_or(const PadClient *client, struct GameMsgC or);

PadServer *init_server(const char *address);

PadServer *init_server_with_port(const char *address, uint16_t port);

void set_server_quiet(PadServer *server);

void enable_server_console(PadServer *server);

void bind_profile_to_server(PadServer *server, GameProfile *profile);

void start_server(PadServer *server);

void stop_server(PadServer *server);

void lock_game_on_server(PadServer *server);

void unlock_game_on_server(PadServer *server);

bool is_server_game_locked(PadServer *server);

void put_a_msg_to_player(PadServer *server, struct GameMsgC msg, const PlayerInfo *player);

void put_msg_to_all_players(PadServer *server, struct GameMsgC msg);

struct CtrlMsgC pop_a_msg_from_player(PadServer *server, const PlayerInfo *player);

struct CtrlMsgC pop_msg_from_player_or(PadServer *server,
                                       const PlayerInfo *player,
                                       struct CtrlMsgC or);

bool is_player_online(PadServer *server, const PlayerInfo *player);

bool is_player_banned(PadServer *server, const PlayerInfo *player);

void kick_player(PadServer *server, const PlayerInfo *player);

void ban_player(PadServer *server, const PlayerInfo *player);

void pardon_player(PadServer *server, const PlayerInfo *player);

struct PlayerList list_online_players(PadServer *server);

struct PlayerList list_banned_players(PadServer *server);
