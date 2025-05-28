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

typedef struct GameProfileC {
  const char *game_name;
  const char *game_description;
  const char *organization;
  const char *version;
  const char *website;
  const char *email;
} GameProfileC;

typedef struct PlayerInfoC {
  const char *account_id;
  const char *account_hash;
  const char *customize_nickname;
  int32_t customize_color_hue;
  double customize_color_saturation;
  double customize_color_value;
} PlayerInfoC;

typedef struct PadClientC {
  const char *target_address;
  uint16_t target_port;
  struct PlayerInfoC bind_player;
  bool enable_console;
  bool quiet;
} PadClientC;

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

typedef struct ControlMessageC {
  enum CtrlMsgCTag tag;
  union CtrlMsgCUnion data;
} ControlMessageC;

typedef union GameMsgCUnion {
  void *nul;
  struct KeyData key;
  enum LeaveReasonData reason;
} GameMsgCUnion;

typedef struct GameMessageC {
  enum GameMsgCTag tag;
  union GameMsgCUnion data;
} GameMessageC;

typedef struct PlayerList {
  struct PlayerInfoC *players;
  uintptr_t len;
  uintptr_t capacity;
} PlayerList;

struct GameProfileC init_game_profile(void);

struct GameProfileC set_game_profile_name(struct GameProfileC game_profile, const char *value);

struct GameProfileC set_game_profile_description(struct GameProfileC game_profile,
                                                 const char *value);

struct GameProfileC set_game_profile_organization(struct GameProfileC game_profile,
                                                  const char *value);

struct GameProfileC set_game_profile_version(struct GameProfileC game_profile, const char *value);

struct GameProfileC set_game_profile_website(struct GameProfileC game_profile, const char *value);

struct GameProfileC set_game_profile_email(struct GameProfileC game_profile, const char *value);

struct PlayerInfoC init_player_info(void);

struct PlayerInfoC set_player_info_account(struct PlayerInfoC info,
                                           const char *id,
                                           const char *password);

struct PlayerInfoC set_player_info_color(struct PlayerInfoC info, int32_t h, double s, double v);

struct PlayerInfoC set_player_info_nickname(struct PlayerInfoC info, const char *nickname);

uint16_t get_default_port(void);

struct PadClientC init_client(const char *address);

struct PadClientC init_client_with_port(const char *address, uint16_t port);

struct PadClientC set_client_quiet(struct PadClientC client);

struct PadClientC enable_client_console(struct PadClientC client);

struct PadClientC bind_player_to_client(struct PadClientC client, struct PlayerInfoC info);

PadClient *complete(struct PadClientC client);

void connect_client_to_server(PadClient *client);

void exit_client_from_server(const PadClient *client);

void put_a_msg_to_server(const PadClient *client, struct ControlMessageC msg);

struct GameMessageC pop_a_msg_from_server(const PadClient *client);

struct GameMessageC pop_msg_from_server_or(const PadClient *client, struct GameMessageC or);

PadServer *init_server(const char *address);

PadServer *init_server_with_port(const char *address, uint16_t port);

void set_server_quiet(PadServer *server);

void enable_server_console(PadServer *server);

void bind_profile_to_server(PadServer *server, const struct GameProfileC *profile);

void start_server(PadServer *server);

void stop_server(PadServer *server);

void lock_game_on_server(PadServer *server);

void unlock_game_on_server(PadServer *server);

bool is_server_game_locked(PadServer *server);

void put_a_msg_to_player(PadServer *server,
                         struct GameMessageC msg,
                         const struct PlayerInfoC *player);

void put_msg_to_all_players(PadServer *server, struct GameMessageC msg);

struct ControlMessageC pop_a_msg_from_player(PadServer *server, const struct PlayerInfoC *player);

struct ControlMessageC pop_msg_from_player_or(PadServer *server,
                                              const struct PlayerInfoC *player,
                                              struct ControlMessageC or);

bool is_player_online(PadServer *server, const struct PlayerInfoC *player);

bool is_player_banned(PadServer *server, const struct PlayerInfoC *player);

void kick_player(PadServer *server, const struct PlayerInfoC *player);

void ban_player(PadServer *server, const struct PlayerInfoC *player);

void pardon_player(PadServer *server, const struct PlayerInfoC *player);

struct PlayerList list_online_players(PadServer *server);

struct PlayerList list_banned_players(PadServer *server);
