/* NoGamepads C++ Bindings. */

/* Generated with cbindgen:0.29.0 */

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

namespace nogamepads {

enum class CtrlMsgCTag {
  Msg,
  Pressed,
  Released,
  Axis,
  Dir,
  Exit,
  Err,
};

enum class GameMsgCTag {
  SkinEventTrigger,
  DisableKey,
  EnableKey,
  Leave,
  Err,
};

enum class LeaveReasonData {
  GameOver,
  ServerClosed,
  YouAreKicked,
  YouAreBanned,
};

struct KeyData {
  uint8_t key;
};

struct AxisData {
  uint8_t key;
  double ax;
};

struct DirData {
  uint8_t key;
  double x;
  double y;
};

struct StrData {
  const char *str;
};

union CtrlMsgCUnion {
  void *nul;
  KeyData key;
  AxisData axis;
  DirData dir;
  StrData str;
};

struct CtrlMsgC {
  CtrlMsgCTag tag;
  CtrlMsgCUnion data;
};

union GameMsgCUnion {
  void *nul;
  KeyData key;
  LeaveReasonData reason;
};

struct GameMsgC {
  GameMsgCTag tag;
  GameMsgCUnion data;
};

struct PlayerList {
  PlayerInfo *players;
  uintptr_t len;
  uintptr_t capacity;
};

extern "C" {

GameProfile *init_game_profile();

void set_game_profile_name(GameProfile *game_profile, const char *value);

void set_game_profile_description(GameProfile *game_profile, const char *value);

void set_game_profile_organization(GameProfile *game_profile, const char *value);

void set_game_profile_version(GameProfile *game_profile, const char *value);

void set_game_profile_website(GameProfile *game_profile, const char *value);

void set_game_profile_email(GameProfile *game_profile, const char *value);

PlayerInfo *init_player_info();

void set_player_info_account(PlayerInfo *info, const char *id, const char *password);

void set_player_info_color(PlayerInfo *info, int32_t h, double s, double v);

void set_player_info_nickname(PlayerInfo *info, const char *nickname);

uint16_t get_default_port();

PadClient *init_client(const char *address);

PadClient *init_client_with_port(const char *address, uint16_t port);

void set_client_quiet(PadClient *client);

void enable_client_console(PadClient *client);

void bind_player_to_client(PadClient *client, PlayerInfo *info);

void connect_client_to_server(PadClient *client);

void exit_client_from_server(const PadClient *client);

void put_a_msg_to_server(const PadClient *client, CtrlMsgC msg);

GameMsgC pop_a_msg_from_server(const PadClient *client);

GameMsgC pop_msg_from_server_or(const PadClient *client, GameMsgC or);

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

void put_a_msg_to_player(PadServer *server, GameMsgC msg, const PlayerInfo *player);

void put_msg_to_all_players(PadServer *server, GameMsgC msg);

CtrlMsgC pop_a_msg_from_player(PadServer *server, const PlayerInfo *player);

CtrlMsgC pop_msg_from_player_or(PadServer *server, const PlayerInfo *player, CtrlMsgC or);

bool is_player_online(PadServer *server, const PlayerInfo *player);

bool is_player_banned(PadServer *server, const PlayerInfo *player);

void kick_player(PadServer *server, const PlayerInfo *player);

void ban_player(PadServer *server, const PlayerInfo *player);

void pardon_player(PadServer *server, const PlayerInfo *player);

PlayerList list_online_players(PadServer *server);

PlayerList list_banned_players(PadServer *server);

}  // extern "C"

}  // namespace nogamepads
