// Auto generated by cbindgen 0.29.0

#ifndef NOGAMEPADS_DATA_H
#define NOGAMEPADS_DATA_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum FfiConnectionMessageTag {
  ConnectionJoin,
  ConnectionRequestGameInfos,
  ConnectionRequestLayoutConfigure,
  ConnectionRequestSkinPackage,
  ConnectionReady,
  ConnectionError,
} FfiConnectionMessageTag;

typedef enum FfiConnectionResponseMessageTag {
  GameInfosResponse,
  DenyResponse,
  FailResponse,
  OkResponse,
  WelcomeResponse,
  ErrorResponse,
} FfiConnectionResponseMessageTag;

typedef enum FfiControlMessageTag {
  CtrlMsg,
  CtrlPressed,
  CtrlReleased,
  CtrlAxis,
  CtrlDir,
  CtrlExit,
  CtrlError,
  CtrlEnd,
} FfiControlMessageTag;

typedef enum FfiExitReason {
  ExitReason,
  GameOverReason,
  ServerClosedReason,
  YouAreKickedReason,
  YouAreBannedReason,
  ErrorReason,
} FfiExitReason;

typedef enum FfiGameMessageTag {
  GameEventTrigger,
  GameMsg,
  GameLetExit,
  GameError,
  GameEnd,
} FfiGameMessageTag;

typedef enum FfiJoinFailedMessage {
  ContainIdenticalPlayer,
  PlayerBanned,
  GameLocked,
  UnknownError,
} FfiJoinFailedMessage;

typedef enum FfiServiceType {
  Unknown,
  TCPConnection,
  BlueTooth,
  USB,
} FfiServiceType;

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

typedef struct FfiKeyAndAxis {
  uint8_t key;
  double axis;
} FfiKeyAndAxis;

typedef struct FfiKeyAndDirection {
  uint8_t key;
  double x;
  double y;
} FfiKeyAndDirection;

typedef union FfiControlMessageUnion {
  char *message;
  uint8_t key;
  struct FfiKeyAndAxis key_and_axis;
  struct FfiKeyAndDirection key_and_direction;
} FfiControlMessageUnion;

typedef struct FfiControlMessage {
  enum FfiControlMessageTag tag;
  union FfiControlMessageUnion data;
} FfiControlMessage;

typedef union FfiGameMessageUnion {
  uint8_t key;
  char *message;
  enum FfiExitReason exit_reason;
} FfiGameMessageUnion;

typedef struct FfiGameMessage {
  enum FfiGameMessageTag tag;
  union FfiGameMessageUnion data;
} FfiGameMessage;

typedef union FfiConnectionMessageUnion {
  struct FfiPlayer player;
} FfiConnectionMessageUnion;

typedef struct FfiConnectionMessage {
  enum FfiConnectionMessageTag tag;
  union FfiConnectionMessageUnion data;
} FfiConnectionMessage;

typedef struct KeyValuePair {
  char *key;
  char *value;
} KeyValuePair;

typedef struct FfiGameInfo {
  struct KeyValuePair *data;
  uintptr_t len;
  uintptr_t cap;
} FfiGameInfo;

typedef union FfiConnectionResponseMessageUnion {
  struct FfiGameInfo game_info;
  enum FfiJoinFailedMessage failed_message;
} FfiConnectionResponseMessageUnion;

typedef struct FfiConnectionResponseMessage {
  enum FfiConnectionResponseMessageTag tag;
  union FfiConnectionResponseMessageUnion data;
} FfiConnectionResponseMessage;

typedef struct FfiControllerData {
  void *_0;
} FfiControllerData;

typedef struct FfiControllerRuntime {
  void *inner;
  void (*drop_fn)(void*);
} FfiControllerRuntime;

typedef struct FfiGameData {
  void *_0;
} FfiGameData;

typedef struct FfiGameRuntimeArchive {
  void *_0;
} FfiGameRuntimeArchive;

typedef struct FfiGameRuntime {
  void *inner;
  void (*drop_fn)(void*);
} FfiGameRuntime;

typedef struct FfiControlEvent {
  struct FfiPlayer player;
  struct FfiControlMessage message;
} FfiControlEvent;

typedef struct FfiButtonStatus {
  bool found;
  bool pressed;
  bool released;
} FfiButtonStatus;

typedef struct FfiAxis {
  bool found;
  double axis;
} FfiAxis;

typedef struct FfiDirection {
  bool found;
  double x;
  double y;
} FfiDirection;

typedef struct FfiBooleanResult {
  bool found;
  bool result;
} FfiBooleanResult;

typedef struct FfiPlayerList {
  struct FfiPlayer *players;
  uintptr_t len;
  uintptr_t cap;
} FfiPlayerList;

typedef struct FfiTcpClientService {
  void *_0;
} FfiTcpClientService;

typedef struct FfiTcpServerService {
  void *_0;
} FfiTcpServerService;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void free_c_string(char *ptr);

/**
 * Register a player
 */
struct FfiPlayer *player_register(const char *id, const char *password);

/**
 * Register a player from hash
 */
struct FfiPlayer *player_from_hash(const char *hash);

/**
 * Get a hash from player
 */
const char *player_get_hash(struct FfiPlayer *player);

/**
 * Check if the player's password is correct
 */
bool player_check(const struct FfiPlayer *player, const char *password);

/**
 * Set the player's nickname
 */
void player_set_nickname(struct FfiPlayer *player, const char *nickname);

/**
 * Set the player's hue
 */
void player_set_hue(struct FfiPlayer *player, int hue);

/**
 * Set the player's HSV color
 */
void player_set_hsv(struct FfiPlayer *player, int hue, double saturation, double value);

/**
 * Free the player
 */
void free_player(struct FfiPlayer *player);

/**
 * Free ControlMessage
 */
void free_control_message(struct FfiControlMessage *msg);

/**
 * Free GameMessage
 */
void free_game_message(struct FfiGameMessage *msg);

/**
 * Free ExitReason
 */
void free_exit_reason(enum FfiExitReason *msg);

/**
 * Free ConnectionMessage
 */
void free_connection_message(struct FfiConnectionMessage *msg);

/**
 * Free ConnectionResponseMessage
 */
void free_connection_response_message(struct FfiConnectionResponseMessage *msg);

/**
 * Free JoinFailedMessage
 */
void free_join_failed_message(enum FfiJoinFailedMessage *msg);

void free_game_info(struct FfiGameInfo map);

/**
 * Create controller data
 */
struct FfiControllerData *controller_data_new(void);

/**
 * Bind player to controller
 */
void controller_data_bind_player(struct FfiControllerData *controller,
                                 struct FfiPlayer *ffi_player);

/**
 * Build runtime
 */
struct FfiControllerRuntime *controller_data_build_runtime(struct FfiControllerData *controller);

/**
 * Free ControllerData memory
 */
void free_controller_data(struct FfiControllerData *controller);

/**
 * Close runtime and exit game
 */
void controller_runtime_close(struct FfiControllerRuntime *runtime);

/**
 * Send control message
 */
void controller_runtime_send_message(struct FfiControllerRuntime *runtime,
                                     struct FfiControlMessage *control_message);

/**
 * Send text message
 */
void controller_runtime_send_text_message(struct FfiControllerRuntime *runtime,
                                          const char *message_ptr);

/**
 * Press a button
 */
void controller_runtime_press_a_button(struct FfiControllerRuntime *runtime, uint8_t key);

/**
 * Release a button
 */
void controller_runtime_release_a_button(struct FfiControllerRuntime *runtime, uint8_t key);

/**
 * Change axis value
 */
void controller_runtime_change_axis(struct FfiControllerRuntime *runtime, uint8_t key, double axis);

/**
 * Change direction value
 */
void controller_runtime_change_direction(struct FfiControllerRuntime *runtime,
                                         uint8_t key,
                                         double x,
                                         double y);

/**
 * Pop a message from the queue
 */
struct FfiGameMessage *controller_runtime_pop(struct FfiControllerRuntime *runtime);

/**
 * Free runtime memory
 */
void free_controller_runtime(struct FfiControllerRuntime *runtime);

/**
 * Create game data
 */
struct FfiGameData *game_data_new(void);

/**
 * Add info
 */
struct FfiGameData *game_data_add_info(struct FfiGameData *data,
                                       const char *key,
                                       const char *value);

/**
 * Set name info
 */
struct FfiGameData *game_data_set_name_info(struct FfiGameData *data, const char *name);

/**
 * Set version info
 */
struct FfiGameData *game_data_set_version_info(struct FfiGameData *data, const char *version);

/**
 * Load data archive
 */
struct FfiGameData *game_data_load_archive(struct FfiGameData *data,
                                           struct FfiGameRuntimeArchive *archive);

/**
 * Build runtime by data
 */
struct FfiGameRuntime *game_data_build_runtime(struct FfiGameData *data);

/**
 * Free data
 */
void free_game_data(struct FfiGameData *data);

/**
 * Create game archive data
 */
struct FfiGameRuntimeArchive *game_archive_data_new(void);

/**
 * Add ban player
 */
struct FfiGameRuntimeArchive *game_archive_data_add_ban_player(struct FfiGameRuntimeArchive *data,
                                                               struct FfiPlayer *ffi_player);

/**
 * Free data
 */
void free_game_archive_data(struct FfiGameRuntimeArchive *data);

/**
 * Send a message to
 */
void game_runtime_send_message_to(struct FfiGameRuntime *runtime,
                                  const struct FfiPlayer *player,
                                  struct FfiGameMessage *message,
                                  enum FfiServiceType service_type);

/**
 * Send a text message
 */
void game_runtime_send_text_message(struct FfiGameRuntime *runtime,
                                    const struct FfiPlayer *player,
                                    enum FfiServiceType service_type,
                                    const char *text);

/**
 * Send a event message
 */
void game_runtime_send_event(struct FfiGameRuntime *runtime,
                             const struct FfiPlayer *player,
                             enum FfiServiceType service_type,
                             uint8_t key);

/**
 * Pop a control event
 */
struct FfiControlEvent *game_runtime_pop_control_event(struct FfiGameRuntime *runtime);

/**
 * Let player exit
 */
void game_runtime_let_exit(struct FfiGameRuntime *runtime,
                           const struct FfiPlayer *player,
                           enum FfiServiceType service_type,
                           enum FfiExitReason reason);

/**
 * Kick a player
 */
void game_runtime_kick_player(struct FfiGameRuntime *runtime,
                              const struct FfiPlayer *player,
                              enum FfiServiceType service_type);

/**
 * Ban a player (And kick)
 */
void game_runtime_ban_player(struct FfiGameRuntime *runtime,
                             const struct FfiPlayer *player,
                             enum FfiServiceType service_type);

/**
 * Pardon a player
 */
void game_runtime_pardon_player(struct FfiGameRuntime *runtime, const struct FfiPlayer *player);

/**
 * Close runtime
 */
void game_runtime_close(struct FfiGameRuntime *runtime);

/**
 * Lock game
 */
void game_runtime_lock(struct FfiGameRuntime *runtime);

/**
 * Unlock game
 */
void game_runtime_unlock(struct FfiGameRuntime *runtime);

/**
 * Get game lock status
 */
bool game_runtime_get_lock_status(struct FfiGameRuntime *runtime);

/**
 * Get button status of player
 */
struct FfiButtonStatus game_runtime_get_button_status(struct FfiGameRuntime *runtime,
                                                      const struct FfiPlayer *player,
                                                      uint8_t key);

/**
 * Get axis value of player
 */
struct FfiAxis game_runtime_get_axis(struct FfiGameRuntime *runtime,
                                     const struct FfiPlayer *player,
                                     uint8_t key);

/**
 * Get direction value of player
 */
struct FfiDirection game_runtime_get_direction(struct FfiGameRuntime *runtime,
                                               const struct FfiPlayer *player,
                                               uint8_t key);

/**
 * Get service type of player
 */
enum FfiServiceType game_runtime_get_service_type(struct FfiGameRuntime *runtime,
                                                  const struct FfiPlayer *player);

/**
 * Is player banned
 */
struct FfiBooleanResult game_runtime_is_player_banned(struct FfiGameRuntime *runtime,
                                                      const struct FfiPlayer *player);

/**
 * Is player online
 */
struct FfiBooleanResult game_runtime_is_player_online(struct FfiGameRuntime *runtime,
                                                      const struct FfiPlayer *player);

/**
 * Get online list
 */
struct FfiPlayerList game_runtime_get_online_list(struct FfiGameRuntime *runtime);

/**
 * Get banned list
 */
struct FfiPlayerList game_runtime_get_banned_list(struct FfiGameRuntime *runtime);

/**
 * Free game runtime
 */
void free_game_runtime(struct FfiGameRuntime *runtime);

/**
 * Free control event
 */
void free_control_event(struct FfiControlEvent *event);

/**
 * Free player list
 */
void free_player_list(struct FfiPlayerList list);

/**
 * Free service type tag
 */
void free_ffi_service_type(enum FfiServiceType *ptr);

/**
 * Build tcp client
 */
struct FfiTcpClientService *tcp_client_build(struct FfiControllerRuntime *runtime);

/**
 * Bind ipv4 address
 */
void tcp_client_bind_ipv4(struct FfiTcpClientService *service,
                          uint8_t a0,
                          uint8_t a1,
                          uint8_t a2,
                          uint8_t a3);

/**
 * Bind ipv6 address
 */
bool tcp_client_bind_ipv6(struct FfiTcpClientService *service, const char *ip_str);

/**
 * Bind port
 */
void tcp_client_bind_port(struct FfiTcpClientService *service, uint16_t port);

/**
 * Bind address with ipv4
 */
void tcp_client_bind_address_v4(struct FfiTcpClientService *service,
                                uint8_t a0,
                                uint8_t a1,
                                uint8_t a2,
                                uint8_t a3,
                                uint16_t port);

/**
 * Bind address with ipv6
 */
bool tcp_client_bind_address_v6(struct FfiTcpClientService *service,
                                const char *ip_str,
                                uint16_t port);

/**
 * Connect
 */
void tcp_client_connect(struct FfiTcpClientService *service);

/**
 * Free tcp client
 */
void free_tcp_client(struct FfiTcpClientService *service);

/**
 * Build tcp server
 */
struct FfiTcpServerService *tcp_server_build(struct FfiGameRuntime *runtime);

/**
 * Bind ipv4 address
 */
void tcp_server_bind_ipv4(struct FfiTcpServerService *service,
                          uint8_t a0,
                          uint8_t a1,
                          uint8_t a2,
                          uint8_t a3);

/**
 * Bind ipv6 address
 */
bool tcp_server_bind_ipv6(struct FfiTcpServerService *service, const char *ip_str);

/**
 * Bind port
 */
void tcp_server_bind_port(struct FfiTcpServerService *service, uint16_t port);

/**
 * Bind address with ipv4
 */
void tcp_server_bind_address_v4(struct FfiTcpServerService *service,
                                uint8_t a0,
                                uint8_t a1,
                                uint8_t a2,
                                uint8_t a3,
                                uint16_t port);

/**
 * Bind address with ipv6
 */
bool tcp_server_bind_address_v6(struct FfiTcpServerService *service,
                                const char *ip_str,
                                uint16_t port);

/**
 * Start listening
 */
void tcp_server_listening_block_on(struct FfiTcpServerService *service);

/**
 * Free tcp server
 */
void free_tcp_server(struct FfiTcpServerService *service);

void enable_logger(uint8_t level);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* NOGAMEPADS_DATA_H */
