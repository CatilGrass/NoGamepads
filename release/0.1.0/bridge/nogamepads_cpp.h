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

struct GameProfileC {
  const char *game_name;
  const char *game_description;
  const char *organization;
  const char *version;
  const char *website;
  const char *email;
};

struct PlayerInfoC {
  const char *account_id;
  const char *account_hash;
  const char *customize_nickname;
  int32_t customize_color_hue;
  double customize_color_saturation;
  double customize_color_value;
};

struct PadClientC {
  const char *target_address;
  uint16_t target_port;
  PlayerInfoC bind_player;
  bool enable_console;
  bool quiet;
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

struct ControlMessageC {
  CtrlMsgCTag tag;
  CtrlMsgCUnion data;
};

union GameMsgCUnion {
  void *nul;
  KeyData key;
  LeaveReasonData reason;
};

struct GameMessageC {
  GameMsgCTag tag;
  GameMsgCUnion data;
};

struct PlayerList {
  PlayerInfoC *players;
  uintptr_t len;
  uintptr_t capacity;
};

extern "C" {

GameProfileC init_game_profile();

GameProfileC set_game_profile_name(GameProfileC game_profile, const char *value);

GameProfileC set_game_profile_description(GameProfileC game_profile, const char *value);

GameProfileC set_game_profile_organization(GameProfileC game_profile, const char *value);

GameProfileC set_game_profile_version(GameProfileC game_profile, const char *value);

GameProfileC set_game_profile_website(GameProfileC game_profile, const char *value);

GameProfileC set_game_profile_email(GameProfileC game_profile, const char *value);

PlayerInfoC init_player_info();

PlayerInfoC set_player_info_account(PlayerInfoC info, const char *id, const char *password);

PlayerInfoC set_player_info_color(PlayerInfoC info, int32_t h, double s, double v);

PlayerInfoC set_player_info_nickname(PlayerInfoC info, const char *nickname);

uint16_t get_default_port();

PadClientC init_client(const char *address);

PadClientC init_client_with_port(const char *address, uint16_t port);

PadClientC set_client_quiet(PadClientC client);

PadClientC enable_client_console(PadClientC client);

PadClientC bind_player_to_client(PadClientC client, PlayerInfoC info);

PadClient *complete(PadClientC client);

void connect_client_to_server(PadClient *client);

void exit_client_from_server(const PadClient *client);

void put_a_msg_to_server(const PadClient *client, ControlMessageC msg);

GameMessageC pop_a_msg_from_server(const PadClient *client);

GameMessageC pop_msg_from_server_or(const PadClient *client, GameMessageC or);

PadServer *init_server(const char *address);

PadServer *init_server_with_port(const char *address, uint16_t port);

void set_server_quiet(PadServer *server);

void enable_server_console(PadServer *server);

void bind_profile_to_server(PadServer *server, const GameProfileC *profile);

void start_server(PadServer *server);

void stop_server(PadServer *server);

void lock_game_on_server(PadServer *server);

void unlock_game_on_server(PadServer *server);

bool is_server_game_locked(PadServer *server);

void put_a_msg_to_player(PadServer *server, GameMessageC msg, const PlayerInfoC *player);

void put_msg_to_all_players(PadServer *server, GameMessageC msg);

ControlMessageC pop_a_msg_from_player(PadServer *server, const PlayerInfoC *player);

ControlMessageC pop_msg_from_player_or(PadServer *server,
                                       const PlayerInfoC *player,
                                       ControlMessageC or);

bool is_player_online(PadServer *server, const PlayerInfoC *player);

bool is_player_banned(PadServer *server, const PlayerInfoC *player);

void kick_player(PadServer *server, const PlayerInfoC *player);

void ban_player(PadServer *server, const PlayerInfoC *player);

void pardon_player(PadServer *server, const PlayerInfoC *player);

PlayerList list_online_players(PadServer *server);

PlayerList list_banned_players(PadServer *server);

}  // extern "C"

}  // namespace nogamepads
