use axum::{body::Body, extract::State, http::{header, HeaderMap, Request, StatusCode}, response::{IntoResponse, Response}, Json};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, io::{Read, Write}, net::{IpAddr, UdpSocket}, path::{Path, PathBuf}, sync::Arc, time::{SystemTime, UNIX_EPOCH}};
use uuid::Uuid;

use crate::{app_api, file_api, https_native, protocol_services, service_adapter, state::AppState, system_info, utils, webdav_server};

const BODY_LIMIT: usize = 16 * 1024 * 1024;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ShareEntry { id: String, name: String, path: String, owner: String, readonly: bool, created_unix: u64, expire_unix: Option<u64> }
#[derive(Debug, Clone, Serialize, Deserialize)]
struct StoragePool { id: String, name: String, path: String, backend: String, enabled: bool, bridged_to: Option<String> }
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SambaShare { id: String, name: String, path: String, users: Vec<String>, readonly: bool, enabled: bool }
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SambaUser { username: String, enabled: bool }
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ServiceConfig { name: String, enabled: bool, port: Option<u16>, upnp: bool, options: serde_json::Value }
#[derive(Debug, Clone, Serialize, Deserialize)]
struct WebDavClient { uuid: String, client_ip: String, username: String, allowed: bool, last_connection_unix: u64 }
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ScheduledJob { id: String, name: String, command: String, schedule: String, enabled: bool, created_unix: u64, logs: Vec<String> }
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ModuleInfo { name: String, group: String, icon_path: String, start_dir: String, version: String, desc: String }

pub async fn handle(State(state): State<Arc<AppState>>, headers: HeaderMap, req: Request<Body>) -> Response {
    let path = req.uri().path().to_string();
    let (params, json_body) = utils::request_params(req, BODY_LIMIT).await;
    match path.as_str() {
        "/alpnas/auth/admin" => admin_auth(&state, &headers),
        "/system/users/list" => users_list(&state),
        "/system/users/userinfo" => user_info(&state, &headers, &params),
        "/system/users/interfaceinfo" => interface_info(&state, &headers),
        "/system/users/profilepic" => profile_pic(),
        "/system/users/editUser" => edit_user(&state, &params),
        "/system/users/removeUser" | "/system/auth/unregister" => remove_user(&state, &params),
        "/system/auth/csvimport" => csv_import(&state, &params),
        "/system/auth/groupdel" => group_delete(&state, &params),
        "/system/auth/logger/index" | "/system/auth/logger/list" => ok(serde_json::json!({"ok": true, "logs": state.auth.login_log()})),
        "/system/auth/whitelist/enable" => { state.auth.set_whitelist_enabled(flag(&params)); ok(serde_json::json!({"ok": true, "whitelist": state.auth.whitelist_state()})) },
        "/system/auth/whitelist/list" => ok(serde_json::json!({"ok": true, "whitelist": state.auth.whitelist_state()})),
        "/system/auth/whitelist/set" => { if let Some(ip) = param(&params, &["ip", "addr", "target"]) { state.auth.whitelist_add(ip.to_string()); } ok(serde_json::json!({"ok": true, "whitelist": state.auth.whitelist_state()})) },
        "/system/auth/whitelist/unset" => { if let Some(ip) = param(&params, &["ip", "addr", "target"]) { state.auth.whitelist_remove(ip); } ok(serde_json::json!({"ok": true, "whitelist": state.auth.whitelist_state()})) },
        "/system/auth/blacklist/enable" => { state.auth.set_blacklist_enabled(flag(&params)); ok(serde_json::json!({"ok": true, "blacklist": state.auth.blacklist_state()})) },
        "/system/auth/blacklist/list" => ok(serde_json::json!({"ok": true, "blacklist": state.auth.blacklist_state()})),
        "/system/auth/blacklist/ban" => { if let Some(ip) = param(&params, &["ip", "addr", "target"]) { state.auth.blacklist_add(ip.to_string()); } ok(serde_json::json!({"ok": true, "blacklist": state.auth.blacklist_state()})) },
        "/system/auth/blacklist/unban" => { if let Some(ip) = param(&params, &["ip", "addr", "target"]) { state.auth.blacklist_remove(ip); } ok(serde_json::json!({"ok": true, "blacklist": state.auth.blacklist_state()})) },
        "/system/autologin/list" => ok(serde_json::json!({"ok": true, "tokens": state.auth.autologin_tokens()})),
        "/system/autologin/create" => autologin_create(&state, &params),
        "/system/autologin/delete" => autologin_delete(&state, &params),
        "/system/auth/u/list" | "/system/auth/u/p/list" => switchable_accounts(&state, &headers),
        "/system/auth/u/switch" | "/system/auth/u/logoutAll" => ok(serde_json::json!({"ok": true, "note": "single-session Rust auth model"})),
        "/system/auth/ldap/checkldap" => ok(serde_json::json!({"ok": true, "enabled": false, "provider": "ldap", "ported": "config endpoints available; live LDAP bind intentionally disabled until configured"})),
        "/system/auth/ldap/config/read" => read_config(&state, "ldap_config.json"),
        "/system/auth/ldap/config/write" => write_config(&state, "ldap_config.json", &params, json_body),
        "/system/auth/ldap/config/testConnection" => ok(serde_json::json!({"ok": false, "error": "LDAP network bind is not enabled in the migration build"})),
        "/system/auth/ldap/config/syncorizeUser" => ok(serde_json::json!({"ok": true, "synced": 0, "note": "LDAP sync requires configured LDAP backend"})),
        "/system/auth/ldap/login" => ldap_login(&state, &params),
        "/system/auth/ldap/newPassword" | "/system/auth/ldap/setPassword" => ldap_password_update(&state, &params),
        "/system/auth/oauth/checkoauth" => ok(serde_json::json!({"ok": true, "enabled": false, "providers": []})),
        "/system/auth/oauth/config/read" => read_config(&state, "oauth_config.json"),
        "/system/auth/oauth/config/write" => write_config(&state, "oauth_config.json", &params, json_body),
        "/system/auth/oauth/login" | "/system/auth/oauth/authorize" => oauth_flow(&state, &params),
        "/public/register/checkPublicRegister" => ok(serde_json::json!(state.config.allow_public_registry || state.auth.is_first_run())),
        "/public/register/register.html" => html("<html><body><h1>ArozOS Rust Register</h1><form method='post' action='/system/auth/register'><input name='username'><input name='password' type='password'><button>Register</button></form></body></html>"),
        "/public/register/handleRegister.html" | "/system/auth/register" => register_from_params(&state, &params),
        "/reset.system" => err(StatusCode::FORBIDDEN, "system reset is disabled in the Rust migration build"),
        "/system/file_system/listDirHash" => call_list_hash(&state, &params).await,
        "/system/file_system/validateFileOpr" => validate_file(&state, &params),
        "/system/file_system/sortMode" => file_sort_mode(&state, &params),
        "/system/file_system/preference" => read_write_user_pref(&state, &headers, &params, json_body),
        "/system/file_system/listTrash" | "/system/file_system/ws/listTrash" => ok(serde_json::json!({"ok": true, "files": []})),
        "/system/file_system/clearTrash" => ok(serde_json::json!({"ok": true, "cleared": 0})),
        "/system/file_system/restoreTrash" => err(StatusCode::NOT_FOUND, "trash restore requires files previously moved into Rust trash"),
        "/system/file_system/ongoing" => ok(serde_json::json!({"ok": true, "tasks": []})),
        "/system/file_system/handleFilePermission" => ok(serde_json::json!({"ok": true, "read": true, "write": true, "owner": true})),
        "/system/file_system/versionHistory" => ok(serde_json::json!({"ok": true, "versions": []})),
        "/system/file_system/handleFolderCache" | "/system/file_system/handleCacheRender" => ok(serde_json::json!({"ok": true, "cached": false, "thumbnail": null})),
        "/system/file_system/loadThumbnail" => file_thumbnail(&state, &params).await,
        "/system/file_system/listMounts" | "/system/file_system/hostRoots" | "/system/file_system/platformDrives" => file_manager_mounts(&state),
        "/system/file_system/ws/fileOpr" => file_opr_from_params(&state, &params),
        "/system/file_system/zipHandler" => zip_handler(&state, &params),
        "/system/file_system/share/new" => share_new(&state, &headers, &params),
        "/system/file_system/share/list" => share_list(&state),
        "/system/file_system/share/delete" => share_delete(&state, &params),
        "/system/file_system/share/edit" => share_edit(&state, &params),
        "/system/file_system/share/checkShared" => share_check(&state, &params),
        "/share" => share_access(&state, &params).await,
        "/system/desktop/listDesktop" | "/system/desktop/files" => desktop_files(&state, &headers, &params),
        "/system/desktop/createShortcut" => desktop_shortcut(&state, &headers, &params),
        "/system/desktop/opr/renameShortcut" => desktop_rename_shortcut(&state, &headers, &params),
        "/system/desktop/preference" | "/system/desktop/theme" => read_write_user_pref(&state, &headers, &params, json_body),
        "/system/desktop/host" => desktop_host(&state),
        "/system/desktop/user" => interface_info(&state, &headers),
        "/system/modules/list" => modules_list(&state),
        "/system/modules/getDefault" => modules_default(&state, &params),
        "/system/modules/getLaunchPara" => module_launch_para(&state, &params),
        "/system/modules/reload" => modules_list(&state),
        "/system/modules/installViaZip" | "/system/module/install" => module_install_zip(&state, &params),
        "/system/apt/list" => apt_list(),
        "/system/setting/list" => settings_list(&state, &params),
        "/system/arsm/aecron/list" => scheduler_list(&state),
        "/system/arsm/aecron/add" => scheduler_add(&state, &params),
        "/system/arsm/aecron/remove" => scheduler_remove(&state, &params),
        "/system/arsm/aecron/listlog" => scheduler_log(&state, &params),
        "/system/storage/pool/list" | "/system/storage/pool/listraw" => storage_pool_list(&state),
        "/system/storage/pool/newHandler" => storage_pool_new(&state, &params),
        "/system/storage/pool/removeHandler" => storage_pool_remove(&state, &params),
        "/system/storage/pool/edit" => storage_pool_edit(&state, &params),
        "/system/storage/pool/toggle" => storage_pool_toggle(&state, &params),
        "/system/storage/pool/bridge" => storage_pool_bridge(&state, &params),
        "/system/storage/pool/checkBridge" => storage_pool_check_bridge(&state, &params),
        "/system/storage/pool/reload" => storage_pool_list(&state),
        "/system/storage/ftp/status" => service_status(&state, "ftp", Some(2121)),
        "/system/storage/ftp/start" => service_toggle(&state, "ftp", true, &params),
        "/system/storage/ftp/stop" => service_toggle(&state, "ftp", false, &params),
        "/system/storage/ftp/setPort" => service_set_port(&state, "ftp", &params),
        "/system/storage/ftp/passivemode" | "/system/storage/ftp/updateGroups" | "/system/storage/ftp/upnp" => service_update(&state, "ftp", &params),
        "/system/storage/ftp/nativeStatus" | "/system/storage/ftp/osstatus" => os_service_status_named(&state, "ftp"),
        "/system/storage/ftp/nativeStart" => native_service_operate_named(&state, "ftp", "start"),
        "/system/storage/ftp/nativeStop" => native_service_operate_named(&state, "ftp", "stop"),
        "/system/storage/ftp/configPreview" | "/system/storage/ftp/configPlan" => service_config_plan(&state, "ftp"),
        "/system/storage/sftp/status" => service_status(&state, "sftp", Some(22)),
        "/system/storage/sftp/nativeStatus" | "/system/storage/sftp/osstatus" => os_service_status_named(&state, "sftp"),
        "/system/storage/sftp/start" => service_toggle(&state, "sftp", true, &params),
        "/system/storage/sftp/stop" => service_toggle(&state, "sftp", false, &params),
        "/system/storage/sftp/nativeStart" => native_service_operate_named(&state, "sftp", "start"),
        "/system/storage/sftp/nativeStop" => native_service_operate_named(&state, "sftp", "stop"),
        "/system/storage/sftp/configPreview" | "/system/storage/sftp/configPlan" => service_config_plan(&state, "sftp"),
        "/system/storage/sftp/port" => sftp_port(&state, &params),
        "/system/storage/sftp/upnp" => sftp_upnp(&state, &params),
        "/system/storage/sftp/users" => sftp_users(&state),
        "/system/storage/tftp/status" => service_status(&state, "tftp", Some(6969)),
        "/system/storage/tftp/start" => service_toggle(&state, "tftp", true, &params),
        "/system/storage/tftp/stop" => service_toggle(&state, "tftp", false, &params),
        "/system/storage/tftp/setPort" => service_set_port(&state, "tftp", &params),
        "/system/storage/tftp/defaultUser" => tftp_default_user(&state, &params),
        "/system/storage/samba/status" => samba_status(&state, &params),
        "/system/storage/samba/list" => samba_list(&state),
        "/system/storage/samba/myshare" => samba_myshare(&state, &headers),
        "/system/storage/samba/add" => samba_add(&state, &params),
        "/system/storage/samba/remove" | "/system/storage/samba/myshare/delete" => samba_remove(&state, &params),
        "/system/storage/samba/editPath" | "/system/storage/samba/updateShareUsers" => samba_edit(&state, &params),
        "/system/storage/samba/listUsers" => samba_user_list(&state),
        "/system/storage/samba/addUser" => samba_user_add(&state, &params),
        "/system/storage/samba/delUser" => samba_user_remove(&state, &params),
        "/system/storage/samba/activate" => samba_user_activation(&state, &headers, true),
        "/system/storage/samba/deactivate" => samba_user_activation(&state, &headers, false),
        "/system/storage/samba/nativeStatus" | "/system/storage/samba/osstatus" => os_service_status_named(&state, "samba"),
        "/system/storage/samba/nativeStart" => native_service_operate_named(&state, "samba", "start"),
        "/system/storage/samba/nativeStop" => native_service_operate_named(&state, "samba", "stop"),
        "/system/storage/samba/configPreview" | "/system/storage/samba/configPlan" => service_config_plan(&state, "samba"),
        "/system/storage/telnet/status" => service_status(&state, "telnet", Some(23)),
        "/system/storage/telnet/start" => service_toggle(&state, "telnet", true, &params),
        "/system/storage/telnet/stop" => service_toggle(&state, "telnet", false, &params),
        "/system/storage/telnet/nativeStatus" | "/system/storage/telnet/osstatus" => os_service_status_named(&state, "telnet"),
        "/system/storage/telnet/nativeStart" => native_service_operate_named(&state, "telnet", "start"),
        "/system/storage/telnet/nativeStop" => native_service_operate_named(&state, "telnet", "stop"),
        "/system/storage/telnet/configPreview" | "/system/storage/telnet/configPlan" => service_config_plan(&state, "telnet"),
        "/system/network/getNICinfo" | "/system/network/getNICUsage" => network_interfaces(),
        "/system/network/getPing" => ping_host(&params),
        "/system/network/scanWifi" => wifi_scan(&state),
        "/system/network/wifiinfo" => wifi_info(&state),
        "/system/network/connectWifi" | "/system/network/removeWifi" | "/system/network/power" => wifi_mutation(&state, &path, &params),
        "/system/network/portforward" => read_write_named_config(&state, "portforward.json", &params, json_body),
        "/system/network/server/list" => network_server_list(&state),
        "/system/network/server/status" => network_server_status(&state),
        "/system/network/server/endpoints" => network_server_endpoints(&state),
        "/system/network/server/platform" | "/system/network/server/adapter" => os_service_adapter_info(&state),
        "/system/network/server/runtimeStatus" => {
            let name = param(&params, &["id", "service", "name"]).unwrap_or("webdav");
            ok(serde_json::json!(protocol_services::runtime_status(&state, name)))
        },
        "/system/network/server/applyRuntime" => {
            let name = param(&params, &["id", "service", "name"]).unwrap_or("webdav");
            let enabled = utils::truthy(param_ref(&params, &["enable", "enabled", "set"]));
            ok(serde_json::json!(protocol_services::apply_service_change(&state, name, enabled)))
        },
        "/system/network/server/osstatus" | "/system/network/server/nativeStatus" => os_service_status(&state, &params),
        "/system/network/server/operate" | "/system/network/server/nativeOperate" => network_server_operate(&state, &params),
        "/system/network/server/configPlan" => service_config_plan(&state, param(&params, &["id", "service", "name"]).unwrap_or("samba")),
        "/system/network/server/toggle" => network_server_toggle(&state, &params),
        "/system/network/webdav/list" => webdav_list(&state, &params),
        "/system/network/webdav/nativeStatus" | "/system/network/webdav/osstatus" => os_service_status_named(&state, "webdav"),
        "/system/network/webdav/nativeStart" => native_service_operate_named(&state, "webdav", "start"),
        "/system/network/webdav/nativeStop" => native_service_operate_named(&state, "webdav", "stop"),
        "/system/network/webdav/configPreview" | "/system/network/webdav/configPlan" => service_config_plan(&state, "webdav"),
        "/system/network/webdav/accessConfig" | "/system/network/webdav/permissions" | "/system/network/webdav/pathRules" => webdav_access_config(&state, &path, &params, json_body),
        "/system/network/webdav/addPath" | "/system/network/webdav/removePath" | "/system/network/webdav/addRule" | "/system/network/webdav/removeRule" => webdav_access_config(&state, &path, &params, json_body),
        "/system/network/webdav/status" => { if param_ref(&params, &["set"]).is_some() { webdav_mutation(&state, &params) } else { webdav_status(&state) } },
        "/system/network/webdav/edit" | "/system/network/webdav/clear" => webdav_mutation(&state, &params),
        "/system/network/nginx/configPreview" | "/system/network/nginx/configPlan" | "/system/network/reverseproxy/nginx" => nginx_reverse_proxy_config(&state, &params),
        "/system/network/nginx/writeConfig" => nginx_write_config(&state, &params),
        "/system/network/https/status" => https_status(&state),
        "/system/network/https/configPreview" | "/system/network/reverseproxy/native" => https_config_preview(&state, &params),
        "/system/network/https/generateSelfSigned" => https_generate_self_signed(&state, &params),
        "/system/network/www/toggle" | "/system/network/www/webRoot" => read_write_user_pref(&state, &headers, &params, json_body),
        "/system/disk/space/list" | "/system/disk/space/resolve" | "/system/disk/space/tmp" => disk_space(&state),
        "/system/disk/space/largeFiles" => large_files(&state, &params),
        "/system/disk/quota/quotaInfo" => quota_info(&state),
        "/system/disk/quota/quotaDist" => quota_distribution(&state),
        "/system/disk/quota/listQuota" => quota_list(&state),
        "/system/disk/quota/setQuota" => quota_set(&state, &params),
        "/system/disk/devices/list" => disk_devices_list(),
        "/system/disk/alpnas/list" => alpnas_drive_list(&state, &headers),
        "/system/disk/devices/model" => block_device_model(&params),
        "/system/disk/alpnas/rescan" => alpnas_drive_list(&state, &headers),
        "/system/disk/alpnas/mount" | "/system/disk/alpnas/unmount" | "/system/disk/diskmg/mount" | "/system/disk/diskmg/format" => disk_privileged_operation(&state, &path, &params),
        "/system/disk/diskmg/platform" => ok(serde_json::json!({"ok": true, "os": std::env::consts::OS, "arch": std::env::consts::ARCH})),
        "/system/disk/diskmg/mpt" | "/system/disk/diskmg/view" => mount_points(),
        "/system/disk/raid/list" | "/system/disk/raid/overview" => raid_list(),
        "/system/disk/raid/detail" | "/system/disk/raid/devinfo" => raid_detail(&params),
        "/system/disk/raid/reload" => raid_list(),
        "/system/disk/raid/new" | "/system/disk/raid/remove" | "/system/disk/raid/addMemeber" | "/system/disk/raid/removeMemeber" | "/system/disk/raid/grow" | "/system/disk/raid/format" | "/system/disk/raid/assemble" => raid_privileged_operation(&state, &path, &params),
        "/system/disk/smart/getSMART" => smart_info(&params),
        "/system/info/getUsageInfo" => usage_info(),
        "/system/info/usbPorts" => usb_ports(),
        "/system/time/getTime" => current_time(),
        "/system/info/wallpaper.jpg" => wallpaper(&state),
        "/AOB/" | "/AOB/hb.php" => ok(serde_json::json!({"ok": true, "service": "arozos-rs", "heartbeat": true})),
        "/AOB/SystemAOB/functions/info/version.inf" => text(env!("CARGO_PKG_VERSION")),
        "/AOB/SystemAOB/functions/system_statistic/getDriveStat.php" => system_info::drive_stat().await.into_response(),
        "/ssdp.xml" => ssdp_xml(&state),
        "/system/subservice/list" => subservice_list(&state),
        "/system/subservice/start" | "/system/subservice/kill" => subservice_mutation(&state, &path, &params),
        "/system/update/checkpending" | "/system/update/checksize" | "/system/update/platform" => ok(serde_json::json!({"ok": true, "pending": false, "platform": std::env::consts::OS})),
        "/system/update/download" | "/system/update/restart" => update_mutation(&state, &path, &params),
        "/system/iot/list" | "/system/iot/listScanner" | "/system/iot/status" | "/system/iot/scan" => iot_list(&state),
        "/system/iot/execute" | "/system/iot/nickname" | "/system/iot/icon" => iot_mutation(&state, &path, &params),
        "/system/ajgi/interface" | "/api/ajgi/interface" => {
            if let Some(script) = param(&params, &["script", "path"]) {
                app_api::handle_script(&state, &headers, script, &params, json_body).await
            } else {
                ok(serde_json::json!({"ok": true, "runtime": "rust", "endpoints": ["/api/ajgi/exec", "/api/ajgi/listExt"], "apps": app_api::builtin_modules_json()}))
            }
        },
        "/api/ajgi/listExt" => agi_ext_list(&state),
        "/api/ajgi/addExt" => agi_ext_add(&state, &params),
        "/api/ajgi/rmExt" => agi_ext_remove(&state, &params),
        "/api/ajgi/exec" => agi_exec(&state, &params),
        p if p.starts_with("/api/remote/") => agi_remote_exec(&state, p, &params),
        "/system/ws" => err(StatusCode::UPGRADE_REQUIRED, "websocket router is declared; interactive websocket message bus is not enabled in this build"),

        "/ldapLogin.html" => html("<html><body><h1>LDAP Login</h1><p>LDAP live login is disabled in this Rust migration build.</p></body></html>"),
        "/system/cluster/scan" | "/system/cluster/record" => ok(serde_json::json!({"ok": true, "neighbours": [], "note": "mDNS/cluster discovery requires the runtime discovery backend"})),
        "/system/cluster/wol" => wake_on_lan(&params),
        "/system/csrf/new" => ok(serde_json::json!({"ok": true, "token": Uuid::new_v4().to_string()})),
        "/system/installer/alpnas/state" => ok(serde_json::json!({"ok": true, "state": "not_started", "runtime": "rust"})),
        "/system/installer/alpnas/targets" => disk_devices_list(),
        "/system/installer/alpnas/start" => installer_start(&state, &params),
        "/system/log/list" => log_list(),
        "/system/log/read" => log_read(&params),
        "/system/backup/listAll" | "/system/backup/listRestorable" => backup_list(&state),
        "/system/backup/snapshotSummary" => backup_summary(&state),
        "/system/backup/restoreFile" => backup_restore(&state, &params),
        "/system/permission/listgroup" => permission_list_with_mode(&state, &params),
        "/system/permission/newgroup" => permission_new(&state, &params),
        "/system/permission/editgroup" => permission_edit(&state, &params),
        "/system/permission/delgroup" => permission_delete(&state, &params),
        "/system/power/accessCheck" => ok(serde_json::json!(state.config.allow_hardware_management || state.config.allow_power_management)),
        "/system/power/restart" | "/system/power/shutdown" => power_operation(&state, &path),
        "/system/register/getAllowRegistry" => ok(serde_json::json!({"ok": true, "allow": state.config.allow_public_registry})),
        "/system/register/setAllowRegistry" | "/system/register/setDefaultGroup" | "/system/register/email" | "/system/register/listUserEmails" | "/system/register/cleanUserRegisterInfo" => read_write_named_config(&state, "register_config.json", &params, json_body),
        "/system/auth/passwordPolicy" => ok(serde_json::json!({"minLength": crate::auth::MIN_PASSWORD_LEN, "allowed": "letters, numbers, spaces and symbols", "disallowed": "line breaks and control characters", "message": crate::auth::PASSWORD_POLICY_TEXT})),
        "/writeCode.php" => write_code_php(&state, &params),
        "/system/reset/validateResetKey" => reset_validate(&state, &params),
        "/system/reset/confirmPasswordReset" => reset_confirm(&state, &params),
        p if p == "/media" || p.starts_with("/media/") => media_endpoint(&state, p, &params).await,
        _ => generic_category(&state, &path, &params, json_body),
    }
}

fn ok(value: serde_json::Value) -> Response { (StatusCode::OK, Json(value)).into_response() }
fn err(status: StatusCode, msg: impl Into<String>) -> Response { (status, Json(serde_json::json!({"ok": false, "error": msg.into()}))).into_response() }
fn text(value: &str) -> Response { (StatusCode::OK, [("content-type", "text/plain; charset=utf-8")], value.to_string()).into_response() }
fn html(value: &str) -> Response { (StatusCode::OK, [("content-type", "text/html; charset=utf-8")], value.to_string()).into_response() }
fn now_unix() -> u64 { SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs() }
fn param<'a>(params: &'a HashMap<String,String>, names: &[&str]) -> Option<&'a str> { names.iter().find_map(|n| params.get(*n).map(|s| s.as_str())) }
fn flag(params: &HashMap<String,String>) -> bool { utils::truthy(param_ref(params, &["enable", "enabled", "status", "value", "on"])) }
fn param_ref<'a>(params: &'a HashMap<String,String>, names: &[&str]) -> Option<&'a String> { names.iter().find_map(|n| params.get(*n)) }

fn read_vec<T: for<'de> Deserialize<'de>>(state: &AppState, filename: &str) -> Vec<T> { utils::read_json_file(utils::data_file(&state.config.system_root, filename)).unwrap_or_default() }
fn write_vec<T: Serialize>(state: &AppState, filename: &str, data: &[T]) -> Result<(), String> { utils::write_json_file(utils::data_file(&state.config.system_root, filename), &data).map_err(|e| e.to_string()) }
fn read_value(state: &AppState, filename: &str) -> serde_json::Value { utils::read_json_file(utils::data_file(&state.config.system_root, filename)).unwrap_or_else(|_| serde_json::json!({})) }
fn write_value(state: &AppState, filename: &str, v: &serde_json::Value) -> Result<(), String> { utils::write_json_file(utils::data_file(&state.config.system_root, filename), v).map_err(|e| e.to_string()) }
fn params_value(params: &HashMap<String,String>) -> serde_json::Value { serde_json::Value::Object(params.iter().map(|(k,v)| (k.clone(), serde_json::Value::String(v.clone()))).collect()) }

fn read_config(state: &AppState, filename: &str) -> Response { ok(serde_json::json!({"ok": true, "config": read_value(state, filename)})) }
fn write_config(state: &AppState, filename: &str, params: &HashMap<String,String>, json_body: Option<serde_json::Value>) -> Response {
    let value = json_body.unwrap_or_else(|| params_value(params));
    match write_value(state, filename, &value) { Ok(()) => ok(serde_json::json!({"ok": true, "config": value})), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e) }
}
fn read_write_named_config(state: &AppState, filename: &str, params: &HashMap<String,String>, json_body: Option<serde_json::Value>) -> Response {
    if params.is_empty() && json_body.is_none() { read_config(state, filename) } else { write_config(state, filename, params, json_body) }
}

fn agi_ext_store(state: &AppState) -> HashMap<String, serde_json::Value> {
    utils::read_json_file(utils::data_file(&state.config.system_root, "agi_external_endpoints.json")).unwrap_or_default()
}
fn agi_ext_save(state: &AppState, map: &HashMap<String, serde_json::Value>) -> Result<(), String> {
    utils::write_json_file(utils::data_file(&state.config.system_root, "agi_external_endpoints.json"), map).map_err(|e| e.to_string())
}
fn agi_ext_list(state: &AppState) -> Response { ok(serde_json::json!(agi_ext_store(state))) }
fn agi_ext_add(state: &AppState, params: &HashMap<String,String>) -> Response {
    let path = param(params, &["path", "file"]).unwrap_or("");
    if path.is_empty() { return err(StatusCode::BAD_REQUEST, "missing path"); }
    let id = Uuid::new_v4().to_string();
    let mut map = agi_ext_store(state);
    map.insert(id.clone(), serde_json::json!({"path": path, "created_unix": now_unix(), "runtime": "rust"}));
    match agi_ext_save(state, &map) { Ok(()) => ok(serde_json::json!(id)), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e) }
}
fn agi_ext_remove(state: &AppState, params: &HashMap<String,String>) -> Response {
    let id = param(params, &["uuid", "id"]).unwrap_or("");
    let mut map = agi_ext_store(state);
    map.remove(id);
    match agi_ext_save(state, &map) { Ok(()) => ok(serde_json::json!("OK")), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e) }
}
fn agi_remote_exec(state: &AppState, path: &str, _params: &HashMap<String,String>) -> Response {
    let id = path.trim_start_matches("/api/remote/");
    let map = agi_ext_store(state);
    if let Some(entry) = map.get(id) {
        ok(serde_json::json!({"ok": true, "uuid": id, "endpoint": entry, "note": "remote AGI endpoint is registered; arbitrary script execution is gated in Rust"}))
    } else {
        err(StatusCode::NOT_FOUND, "remote AGI endpoint not found")
    }
}

fn admin_auth(state: &AppState, headers: &HeaderMap) -> Response {
    if state.auth.session_from_headers(headers).map(|s| s.is_admin).unwrap_or(false) { (StatusCode::NO_CONTENT, "").into_response() } else { (StatusCode::UNAUTHORIZED, "401 Unauthorized").into_response() }
}

fn user_info(state: &AppState, headers: &HeaderMap, params: &HashMap<String,String>) -> Response {
    let user = param(params, &["username", "user"]).and_then(|u| state.auth.get_user(u)).or_else(|| state.auth.current_user(headers));
    match user {
        Some(u) => {
            let username = u.username.clone();
            let display_name = u.display_name.clone();
            let group = u.group.clone();
            let profile_image = u.profile_image.clone();
            let icon_data = if profile_image.starts_with("data:") { profile_image.clone() } else { String::new() };
            let profile_image_public = if profile_image.is_empty() { "SystemAO/users/img/noprofileicon.png".to_string() } else { profile_image };
            ok(serde_json::json!({
                "ok": true,
                "user": u.clone(),
                "Username": username,
                "Nickname": display_name,
                "Usergroup": [group.clone()],
                "UserGroups": [group],
                "Icondata": icon_data,
                "ProfileImage": profile_image_public,
                "IsAdmin": u.is_admin,
                "Disabled": u.disabled
            }))
        },
        None => err(StatusCode::UNAUTHORIZED, "user not logged in")
    }
}

fn interface_info(state: &AppState, headers: &HeaderMap) -> Response {
    let user = state.auth.current_user(headers)
        .or_else(|| state.auth.session_from_headers(headers).and_then(|s| state.auth.get_user(&s.username)))
        .or_else(|| state.auth.list_users().into_iter().next());
    if let Some(user) = user {
        ok(serde_json::json!({
            "Username": user.username,
            "Nickname": user.display_name,
            "UserGroups": [user.group],
            "UserIcon": user.profile_image,
            "IsAdmin": user.is_admin,
            "Disabled": user.disabled
        }))
    } else {
        err(StatusCode::UNAUTHORIZED, "Not logged in.")
    }
}

fn users_list(state: &AppState) -> Response {
    let rows: Vec<serde_json::Value> = state.auth.list_users().into_iter().map(|u| {
        serde_json::json!([u.username, vec![u.group], if u.profile_image.is_empty() { "SystemAO/users/img/noprofileicon.png".to_string() } else { u.profile_image }, u.is_admin])
    }).collect();
    ok(serde_json::json!(rows))
}

fn profile_pic() -> Response { (StatusCode::OK, [("content-type", "image/svg+xml")], r#"<svg xmlns='http://www.w3.org/2000/svg' width='96' height='96'><rect width='100%' height='100%' rx='16' fill='#ddd'/><text x='50%' y='56%' dominant-baseline='middle' text-anchor='middle' font-size='42'>👤</text></svg>"#).into_response() }

fn edit_user(state: &AppState, params: &HashMap<String,String>) -> Response {
    let username = param(params, &["username", "user"]).unwrap_or("");
    let is_admin = utils::truthy(param_ref(params, &["admin", "isAdmin"])) || param(params, &["group"]).map(|g| g.eq_ignore_ascii_case("administrator")).unwrap_or(false);
    match state.auth.upsert_user(username, param(params, &["password", "pw"]), is_admin, param(params, &["group"]), param(params, &["displayName", "display_name", "nickname"])) {
        Ok(user) => ok(serde_json::json!({"ok": true, "user": user})), Err(e) => err(StatusCode::BAD_REQUEST, e),
    }
}
fn register_from_params(state: &AppState, params: &HashMap<String,String>) -> Response {
    let username = param(params, &["username", "user"]).unwrap_or("");
    let password = param(params, &["password", "pw"]).unwrap_or("");
    if password.len() < 4 { return text("Error. Password too short"); }
    let first_user = state.auth.is_first_run();
    let group = if first_user { "administrator" } else { param(params, &["group"]).unwrap_or("user") };
    let is_admin = first_user || group.eq_ignore_ascii_case("administrator") || utils::truthy(param_ref(params, &["admin", "isAdmin"]));
    match state.auth.upsert_user(username, Some(password), is_admin, Some(group), param(params, &["displayName", "display_name", "nickname"])) {
        Ok(user) => {
            let base = PathBuf::from(&state.config.root_directory).join("users").join(&user.username);
            for folder in ["Desktop", "Document", "Documents", "Download", "Downloads", "Music", "Photo", "Video", "Audio", "Web"] { let _ = fs::create_dir_all(base.join(folder)); }
            text("OK")
        },
        Err(e) => text(&format!("Error. {}", e)),
    }
}
fn remove_user(state: &AppState, params: &HashMap<String,String>) -> Response {
    let usernames = param(params, &["usernames", "username", "user"]).unwrap_or("");
    let mut removed = Vec::new(); let mut failed = Vec::new();
    for u in usernames.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()) { if state.auth.remove_user(u) { removed.push(u.to_string()) } else { failed.push(u.to_string()) } }
    ok(serde_json::json!({"ok": failed.is_empty(), "removed": removed, "failed": failed}))
}
fn csv_import(state: &AppState, params: &HashMap<String,String>) -> Response {
    let csv = param(params, &["csv", "data"]).unwrap_or("");
    let mut created = Vec::new();
    for line in csv.lines() {
        let cols: Vec<_> = line.split(',').map(|s| s.trim()).collect();
        if cols.len() >= 2 {
            let admin = cols.get(2).map(|v| *v == "admin" || *v == "true").unwrap_or(false);
            if let Ok(user) = state.auth.upsert_user(cols[0], Some(cols[1]), admin, cols.get(2).copied(), cols.get(3).copied()) { created.push(user); }
        }
    }
    ok(serde_json::json!({"ok": true, "created": created}))
}
fn group_delete(state: &AppState, params: &HashMap<String,String>) -> Response {
    let group = param(params, &["group", "usergroup"]).unwrap_or("");
    let users = state.auth.list_users(); let mut removed = Vec::new();
    for u in users { if u.group == group && u.username != "admin" && state.auth.remove_user(&u.username) { removed.push(u.username); } }
    ok(serde_json::json!({"ok": true, "removed": removed}))
}
fn autologin_create(state: &AppState, params: &HashMap<String,String>) -> Response {
    let username = param(params, &["username", "user"]).unwrap_or("admin");
    match state.auth.create_autologin_token(username) { Some((token, user)) => ok(serde_json::json!({"ok": true, "token": token, "username": user})), None => err(StatusCode::NOT_FOUND, "user not found") }
}
fn autologin_delete(state: &AppState, params: &HashMap<String,String>) -> Response { let ok_removed = param(params, &["token", "key"]).map(|t| state.auth.remove_autologin_token(t)).unwrap_or(false); ok(serde_json::json!({"ok": ok_removed})) }
fn switchable_accounts(state: &AppState, headers: &HeaderMap) -> Response {
    let accounts = state.auth.session_from_headers(headers).map(|s| vec![serde_json::json!({"Username": s.username, "IsAdmin": s.is_admin, "IsExpired": false})]).unwrap_or_default();
    ok(serde_json::json!(accounts))
}

async fn call_list_hash(state: &AppState, params: &HashMap<String,String>) -> Response {
    use sha2::{Digest, Sha256};
    let raw = param_ref(params, &["dir", "path", "target"]);
    let dir = match file_api::resolve(&state.config.root_directory, raw) { Ok(p) => p, Err(e) => return err(StatusCode::BAD_REQUEST, e.to_string()) };
    let mut h = Sha256::new();
    if let Ok(rd) = fs::read_dir(dir) {
        for e in rd.flatten() {
            h.update(e.file_name().to_string_lossy().as_bytes());
            if let Ok(md) = e.metadata() { h.update(md.len().to_le_bytes()); }
        }
    }
    ok(serde_json::json!({"ok": true, "hash": format!("{:x}", h.finalize())}))
}
fn validate_file(state: &AppState, params: &HashMap<String,String>) -> Response {
    let path = param_ref(params, &["src", "source", "path", "file"]);
    match file_api::resolve(&state.config.root_directory, path) { Ok(p) => ok(serde_json::json!({"ok": true, "exists": p.exists(), "path": p.to_string_lossy()})), Err(e) => err(StatusCode::BAD_REQUEST, e.to_string()) }
}
fn file_opr_from_params(state: &AppState, params: &HashMap<String,String>) -> Response { match file_api::file_opr_inner(&state.config.root_directory, params) { Ok(v) => ok(v), Err(e) => err(StatusCode::BAD_REQUEST, e.to_string()) } }

fn pref_name(headers: &HeaderMap) -> String { headers.get("cookie").and_then(|v| v.to_str().ok()).map(|s| format!("user_pref_{}.json", s.len())).unwrap_or_else(|| "user_pref_default.json".to_string()) }
fn read_write_user_pref(state: &AppState, headers: &HeaderMap, params: &HashMap<String,String>, json_body: Option<serde_json::Value>) -> Response {
    let filename = pref_name(headers);

    // ArozOS' file manager uses /system/file_system/preference as a simple
    // key/value store: ?key=<name> reads one value, ?key=<name>&value=<v>
    // writes one value. The generic config helper treats any query parameter
    // as a full config write, which turns reads such as
    // ?key=file_explorer/listmode into {"key":"file_explorer/listmode"}.
    // That makes the browser assign an object to viewMode/theme and prevents
    // directory entries from being rendered. Keep the generic behavior only
    // for callers that do not use key/value semantics.
    if let Some(key) = param(params, &["key"]) {
        let requested_value = param(params, &["value", "val"])
            .map(|v| serde_json::Value::String(v.to_string()))
            .or_else(|| json_body.clone());

        if let Some(value) = requested_value {
            let mut map = read_value(state, &filename).as_object().cloned().unwrap_or_default();
            map.insert(key.to_string(), value);
            return match write_value(state, &filename, &serde_json::Value::Object(map)) {
                Ok(()) => ok(serde_json::json!("OK")),
                Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e),
            };
        }

        let stored = read_value(state, &filename);
        return ok(stored.get(key).cloned().unwrap_or_else(|| serde_json::Value::String(String::new())));
    }

    read_write_named_config(state, &filename, params, json_body)
}

fn file_sort_mode(state: &AppState, params: &HashMap<String,String>) -> Response {
    let opr = param(params, &["opr", "op"]).unwrap_or("get").to_ascii_lowercase();
    let folder = param(params, &["folder", "dir", "path"]).unwrap_or("user:/");
    let filename = "file_sort_modes.json";

    if opr == "set" || opr == "write" || opr == "save" {
        let mode = param(params, &["mode", "sort", "value"]).unwrap_or("default");
        let mut map = read_value(state, filename).as_object().cloned().unwrap_or_default();
        map.insert(folder.to_string(), serde_json::json!(mode));
        return match write_value(state, filename, &serde_json::Value::Object(map)) {
            Ok(()) => ok(serde_json::json!("OK")),
            Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e),
        };
    }

    let stored = read_value(state, filename);
    let mode = stored
        .get(folder)
        .and_then(|v| v.as_str())
        .unwrap_or("default");
    ok(serde_json::json!(mode))
}

fn share_new(state: &AppState, headers: &HeaderMap, params: &HashMap<String,String>) -> Response {
    let path = param(params, &["path", "file", "src"]).unwrap_or("user:/").to_string();
    let owner = state.auth.session_from_headers(headers).map(|s| s.username).unwrap_or_else(|| "admin".to_string());
    let mut shares: Vec<ShareEntry> = read_vec(state, "shares.json");
    let entry = ShareEntry { id: Uuid::new_v4().to_string(), name: param(params, &["name"]).unwrap_or("Shared file").to_string(), path, owner, readonly: !utils::truthy(param_ref(params, &["write", "editable"])), created_unix: now_unix(), expire_unix: param(params, &["expire", "expires"]).and_then(|s| s.parse().ok()) };
    shares.push(entry.clone());
    match write_vec(state, "shares.json", &shares) { Ok(()) => ok(serde_json::json!({"ok": true, "share": entry, "url": format!("/share?id={}", entry.id)})), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e) }
}
fn share_list(state: &AppState) -> Response { let shares: Vec<ShareEntry> = read_vec(state, "shares.json"); ok(serde_json::json!({"ok": true, "shares": shares})) }
fn share_delete(state: &AppState, params: &HashMap<String,String>) -> Response { let id = param(params, &["id", "uuid", "share"]).unwrap_or(""); let mut shares: Vec<ShareEntry> = read_vec(state, "shares.json"); let old = shares.len(); shares.retain(|s| s.id != id); match write_vec(state, "shares.json", &shares) { Ok(()) => ok(serde_json::json!({"ok": true, "removed": old - shares.len()})), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e) } }
fn share_edit(state: &AppState, params: &HashMap<String,String>) -> Response { let id = param(params, &["id", "uuid", "share"]).unwrap_or(""); let mut shares: Vec<ShareEntry> = read_vec(state, "shares.json"); let mut found = None; for s in &mut shares { if s.id == id { if let Some(name) = param(params, &["name"]) { s.name = name.to_string(); } if let Some(path) = param(params, &["path", "file"]) { s.path = path.to_string(); } s.readonly = !utils::truthy(param_ref(params, &["write", "editable"])); found = Some(s.clone()); } } match write_vec(state, "shares.json", &shares) { Ok(()) => ok(serde_json::json!({"ok": found.is_some(), "share": found})), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e) } }
fn share_check(state: &AppState, params: &HashMap<String,String>) -> Response { let path = param(params, &["path", "file"]).unwrap_or(""); let shares: Vec<ShareEntry> = read_vec(state, "shares.json"); let matched: Vec<_> = shares.into_iter().filter(|s| s.path == path || s.id == path).collect(); ok(serde_json::json!({"ok": true, "shared": !matched.is_empty(), "shares": matched})) }
async fn share_access(state: &AppState, params: &HashMap<String,String>) -> Response {
    let id = param(params, &["id", "uuid", "share"]).unwrap_or(""); let shares: Vec<ShareEntry> = read_vec(state, "shares.json");
    if let Some(s) = shares.iter().find(|s| s.id == id) { return ok(serde_json::json!({"ok": true, "share": s, "note": "use the file path through /system/file_system APIs"})); }
    err(StatusCode::NOT_FOUND, "share not found")
}

fn desktop_dir(state: &AppState, headers: &HeaderMap) -> PathBuf {
    let username = state.auth.session_from_headers(headers)
        .map(|s| s.username)
        .or_else(|| state.auth.list_users().into_iter().next().map(|u| u.username))
        .unwrap_or_else(|| "admin".to_string());
    PathBuf::from(&state.config.root_directory).join("users").join(username).join("Desktop")
}

fn desktop_positions_path(state: &AppState, headers: &HeaderMap) -> String {
    let username = state.auth.session_from_headers(headers).map(|s| s.username).unwrap_or_else(|| "default".to_string());
    format!("desktop_positions_{}.json", username)
}

fn desktop_files(state: &AppState, headers: &HeaderMap, params: &HashMap<String,String>) -> Response {
    let pos_file = desktop_positions_path(state, headers);
    let mut positions: HashMap<String, (i32, i32)> = read_value(state, &pos_file)
        .as_object()
        .map(|obj| obj.iter().filter_map(|(k,v)| Some((k.clone(), (v.get("x")?.as_i64()? as i32, v.get("y")?.as_i64()? as i32)))).collect())
        .unwrap_or_default();
    if let Some(name) = param(params, &["set"]) {
        let x = param(params, &["x"]).and_then(|v| v.parse::<i32>().ok()).unwrap_or(-1);
        let y = param(params, &["y"]).and_then(|v| v.parse::<i32>().ok()).unwrap_or(-1);
        positions.insert(name.to_string(), (x, y));
        let value = serde_json::Value::Object(positions.iter().map(|(k,(x,y))| (k.clone(), serde_json::json!({"x": x, "y": y}))).collect());
        let _ = write_value(state, &pos_file, &value);
        return text("OK");
    }
    if let Some(name) = param(params, &["del"]) {
        positions.remove(name);
        let value = serde_json::Value::Object(positions.iter().map(|(k,(x,y))| (k.clone(), serde_json::json!({"x": x, "y": y}))).collect());
        let _ = write_value(state, &pos_file, &value);
        return text("OK");
    }

    let dir = desktop_dir(state, headers);
    let _ = fs::create_dir_all(&dir);
    let mut out = Vec::new();
    if let Ok(rd) = fs::read_dir(&dir) {
        for e in rd.flatten() {
            let p = e.path();
            let name = e.file_name().to_string_lossy().to_string();
            let (x, y) = positions.get(&name).cloned().unwrap_or((-1, -1));
            let is_dir = p.is_dir();
            let ext = if is_dir { "".to_string() } else { p.extension().map(|e| format!(".{}", e.to_string_lossy())).unwrap_or_default() };
            let is_empty = is_dir && fs::read_dir(&p).map(|mut r| r.next().is_none()).unwrap_or(true);
            out.push(serde_json::json!({
                "Filename": name,
                "Filepath": format!("user:/Desktop/{}", e.file_name().to_string_lossy()),
                "Ext": ext,
                "IsDir": is_dir,
                "IsEmptyDir": is_empty,
                "IsShared": false,
                "IsShortcut": false,
                "IconX": x,
                "IconY": y
            }));
        }
    }
    // Always expose the built-in ArozOS application shortcuts in the desktop
    // response. Older migration builds sometimes created normal files/folders in
    // user:/Desktop, which prevented the default shortcut fallback from being
    // emitted at all. The Go desktop always has module launch entries available;
    // this keeps the Rust desktop independent from stale per-user state.
    let existing_shortcuts: std::collections::HashSet<String> = out.iter()
        .filter_map(|v| v.get("ShortcutName").and_then(|n| n.as_str()).or_else(|| v.get("Filename").and_then(|n| n.as_str())))
        .map(|s| s.trim_end_matches(".shortcut").to_string())
        .collect();
    let base_index = out.len();
    for (i, m) in default_desktop_shortcuts().into_iter().enumerate() {
        let name = m["Name"].as_str().unwrap_or("App").to_string();
        if existing_shortcuts.contains(&name) { continue; }
        out.push(serde_json::json!({
            "Filename": format!("{}.shortcut", name),
            "Filepath": format!("user:/Desktop/{}.shortcut", name),
            "Ext": ".shortcut",
            "IsDir": false,
            "IsEmptyDir": false,
            "IsShared": false,
            "IsShortcut": true,
            "ShortcutType": "module",
            "ShortcutName": name,
            "ShortcutPath": m["Name"],
            "ShortcutImage": m["IconPath"],
            "IconX": ((base_index + i) as i32) / 6,
            "IconY": ((base_index + i) as i32) % 6
        }));
    }
    ok(serde_json::json!(out))
}

fn desktop_shortcut(state: &AppState, headers: &HeaderMap, params: &HashMap<String,String>) -> Response { let dir = desktop_dir(state, headers); let _ = fs::create_dir_all(&dir); let name = utils::sanitize_relative_path(param(params, &["name", "title"]).unwrap_or("shortcut")).replace('/', "_"); let target = param(params, &["target", "path", "url"]).unwrap_or(""); let file = dir.join(format!("{}.shortcut.json", if name.is_empty() { "shortcut" } else { &name })); let value = serde_json::json!({"name": name, "target": target, "created": now_unix()}); match utils::write_json_file(&file, &value) { Ok(()) => ok(serde_json::json!({"ok": true, "shortcut": value})), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()) } }
fn desktop_rename_shortcut(state: &AppState, headers: &HeaderMap, params: &HashMap<String,String>) -> Response { let dir = desktop_dir(state, headers); let old = param(params, &["old", "src", "name"]).unwrap_or(""); let new = param(params, &["new", "dest", "newname"]).unwrap_or(""); if old.is_empty() || new.is_empty() { return err(StatusCode::BAD_REQUEST, "missing old/new name"); } match fs::rename(dir.join(old), dir.join(new)) { Ok(()) => ok(serde_json::json!({"ok": true})), Err(e) => err(StatusCode::BAD_REQUEST, e.to_string()) } }

fn system_module(name: &str, group: &str, icon: &str, start: &str, desc: &str) -> serde_json::Value {
    serde_json::json!({
        "Name": name, "Desc": desc, "Group": group, "IconPath": icon, "Version": "1.0", "StartDir": start,
        "SupportFW": true, "LaunchFWDir": start, "SupportEmb": false, "LaunchEmb": "", "InitFWSize": [1080, 580], "InitEmbSize": [0, 0], "SupportedExt": []
    })
}

fn setting_launch(group: &str, name: &str) -> String {
    fn enc(s: &str) -> String {
        s.bytes().map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => (b as char).to_string(),
            b' ' => "%20".to_string(),
            _ => format!("%{:02X}", b),
        }).collect::<String>()
    }
    let payload = format!(r#"{{"group":"{}","name":"{}"}}"#, group, name);
    format!("SystemAO/system_setting/index.html#{}", enc(&payload))
}

fn system_modules_json() -> Vec<serde_json::Value> {
    vec![
        system_module("File Manager", "System Tools", "SystemAO/file_system/img/small_icon.png", "SystemAO/file_system/file_explorer.html", "Browse AlpNAS data, shares and mounted drives"),
        system_module("System Setting", "System Settings", "SystemAO/system_setting/img/small_icon.png", "SystemAO/system_setting/index.html", "Native AlpNAS control center for system, storage and users"),
        system_module("Users", "System Settings", "SystemAO/users/img/users.svg", &setting_launch("Users", "User List"), "Manage users and groups"),
        system_module("System Info", "System Settings", "SystemAO/info/img/small_icon.png", &setting_launch("Info", "Overview"), "System information"),
        system_module("Personalization", "System Settings", "SystemAO/desktop/img/personalization.png", "SystemAO/desktop/personalization.html", "Desktop appearance and wallpaper"),
        system_module("Disk Space", "Storage", "SystemAO/disk/space/img/small_icon.png", &setting_launch("Disk", "Disk Space"), "Disk usage overview"),
        system_module("Samba", "Storage", "img/system/network-samba.svg", &setting_launch("Network", "File Servers"), "Samba sharing configuration"),
        system_module("FTP", "Storage", "img/system/network-folder.svg", &setting_launch("Network", "File Servers"), "FTP service configuration"),
        system_module("WebDAV", "Storage", "img/system/network-folder-blue.svg", &setting_launch("Network", "File Servers"), "WebDAV service configuration"),
        system_module("Network", "System Settings", "SystemAO/system_setting/img/network.svg", &setting_launch("Network", "Network Info"), "Network settings"),
        system_module("Wi-Fi", "System Settings", "SystemAO/network/img/wifi.svg", &setting_launch("Network", "WiFi Settings"), "Wireless network settings"),
        system_module("Scheduler", "System Tools", "SystemAO/arsm/img/scheduler.png", "SystemAO/arsm/scheduler.html", "Scheduled jobs"),
        system_module("Logs", "System Tools", "SystemAO/advance/img/small_icon.png", &setting_launch("Advance", "System Log"), "System logs"),
    ]
}

fn all_modules_json() -> Vec<serde_json::Value> {
    let mut modules = system_modules_json();
    modules.extend(app_api::builtin_modules_json());
    modules
}

fn default_desktop_shortcuts() -> Vec<serde_json::Value> {
    let wanted = ["File Manager", "System Setting", "Users", "Memo", "NotepadA", "Photo", "Music", "Video", "Code Studio", "Web Downloader"];
    let modules = all_modules_json();
    wanted.iter().filter_map(|name| modules.iter().find(|m| m["Name"].as_str() == Some(*name)).cloned()).collect()
}

fn modules_list(_state: &AppState) -> Response { ok(serde_json::json!(all_modules_json())) }

fn modules_default(state: &AppState, params: &HashMap<String,String>) -> Response {
    let opr = param(params, &["opr"]).unwrap_or("list");
    let mut defaults = read_value(state, "module_default_openers.json");
    if !defaults.is_object() { defaults = serde_json::json!({}); }
    if opr == "set" {
        let ext = param(params, &["ext"]).unwrap_or("").trim().trim_start_matches('.').to_ascii_lowercase();
        let module = param(params, &["module", "name"]).unwrap_or("").to_string();
        if ext.is_empty() || module.is_empty() { return err(StatusCode::BAD_REQUEST, "missing ext/module"); }
        if let Some(obj) = defaults.as_object_mut() { obj.insert(ext, serde_json::Value::String(module)); }
        return match write_value(state, "module_default_openers.json", &defaults) {
            Ok(()) => ok(serde_json::json!("OK")),
            Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e),
        };
    }
    if opr == "launch" {
        let ext = param(params, &["ext"]).unwrap_or("").trim().trim_start_matches('.').to_ascii_lowercase();
        if let Some(name) = defaults.get(&ext).and_then(|v| v.as_str()) {
            if let Some(m) = all_modules_json().into_iter().find(|m| m["Name"].as_str() == Some(name)) { return ok(m); }
        }
        if let Some(m) = all_modules_json().into_iter().find(|m| m["SupportedExt"].as_array().map(|a| a.iter().any(|v| v.as_str().map(|s| s.trim_start_matches('.').eq_ignore_ascii_case(&ext)).unwrap_or(false))).unwrap_or(false)) {
            return ok(m);
        }
        return ok(system_modules_json().into_iter().next().unwrap_or_else(|| serde_json::json!({})));
    }
    if opr == "list" {
        return ok(defaults);
    }
    ok(defaults)
}

fn module_launch_para(_state: &AppState, params: &HashMap<String,String>) -> Response {
    let name = param(params, &["module", "name"]).unwrap_or("");
    let decoded = utils::percent_decode(name);
    if let Some(m) = all_modules_json().into_iter().find(|m| m["Name"].as_str().map(|n| n == decoded).unwrap_or(false) || m["StartDir"].as_str().map(|n| n == decoded).unwrap_or(false)) {
        return ok(m);
    }
    err(StatusCode::NOT_FOUND, "module not found")
}

fn desktop_host(state: &AppState) -> Response {
    let uuid = if state.config.system_uuid.is_empty() { "arozos-rs" } else { state.config.system_uuid.as_str() };
    ok(serde_json::json!({
        "Hostname": &state.config.host_name,
        "DeviceModel": "ArozOS Rust",
        "BuildVersion": env!("CARGO_PKG_VERSION"),
        "InternalVersion": "rust-port",
        "DeviceUUID": uuid
    }))
}

fn apt_list() -> Response { ok(serde_json::json!({"ok": true, "packages": [], "note": "APT package discovery is disabled unless integrated with distro package manager"})) }

fn setting_groups() -> serde_json::Value {
    serde_json::json!([
        {"Name":"AlpNAS Host","Group":"Info","IconPath":"SystemAO/system_setting/img/server.svg","Desc":"Status, updates and runtime information for this AlpNAS node"},
        {"Name":"Devices & IoT","Group":"Device","IconPath":"SystemAO/system_setting/img/device.svg","Desc":"Connected clients and IoT devices"},
        {"Name":"Module Management","Group":"Module","IconPath":"SystemAO/system_setting/img/module.svg","Desc":"List of modules loaded in the system"},
        {"Name":"Storage & Drives","Group":"Disk","IconPath":"SystemAO/system_setting/img/drive.svg","Desc":"Manage AlpNAS pools, mounted media and local disks"},
        {"Name":"Network & Services","Group":"Network","IconPath":"SystemAO/system_setting/img/network.svg","Desc":"Manage host networking and service connectivity"},
        {"Name":"Users & Groups","Group":"Users","IconPath":"SystemAO/system_setting/img/users.svg","Desc":"Add, remove or edit users and groups"},
        {"Name":"Clusters & Scheduling","Group":"Cluster","IconPath":"SystemAO/system_setting/img/cluster.svg","Desc":"Cluster, network scanning and task scheduling"},
        {"Name":"Security & Auth","Group":"Security","IconPath":"SystemAO/system_setting/img/security.svg","Desc":"System security and auth credentials"},
        {"Name":"Admin Tools","Group":"Advance","IconPath":"SystemAO/system_setting/img/code.svg","Desc":"Advanced AlpNAS administration and debugging"},
        {"Name":"About AlpNAS","Group":"About","IconPath":"SystemAO/system_setting/img/info.svg","Desc":"Information about the current AlpNAS desktop runtime"}
    ])
}

fn setting_modules() -> Vec<serde_json::Value> {
    vec![
        serde_json::json!({"Name":"Overview","Desc":"Overview for user information","IconPath":"SystemAO/info/img/small_icon.png","Group":"Info","StartDir":"SystemAO/info/overview.html","RequireAdmin":false}),
        serde_json::json!({"Name":"Host Info","Desc":"System Information","IconPath":"SystemAO/info/img/small_icon.png","Group":"Info","StartDir":"SystemAO/info/index.html","RequireAdmin":false}),
        serde_json::json!({"Name":"Performance","Desc":"System CPU and RAM usage","IconPath":"SystemAO/info/img/small_icon.png","Group":"Info","StartDir":"SystemAO/info/taskManagerFrame.html","RequireAdmin":false}),
        serde_json::json!({"Name":"Runtime","Desc":"Change startup parameter in runtime","IconPath":"SystemAO/info/img/small_icon.png","Group":"Info","StartDir":"SystemAO/boot/bootflags.html","RequireAdmin":true}),
        serde_json::json!({"Name":"Updates","Desc":"Perform AlpNAS updates","IconPath":"SystemAO/updates/img/update.png","Group":"Info","StartDir":"SystemAO/updates/index.html","RequireAdmin":true}),
        serde_json::json!({"Name":"Power","Desc":"Set the power state of the host device","IconPath":"SystemAO/boot/img/boot.png","Group":"Info","StartDir":"SystemAO/boot/poweroff.html","RequireAdmin":true}),
        serde_json::json!({"Name":"Client Device","Desc":"Detail about the browser you are using","IconPath":"SystemAO/info/img/small_icon.png","Group":"Device","StartDir":"SystemAO/info/clientInfo.html","RequireAdmin":false}),
        serde_json::json!({"Name":"Audio Testing","Desc":"Speaker and volume testing","IconPath":"SystemAO/info/img/small_icon.png","Group":"Device","StartDir":"SystemAO/info/audio.html","RequireAdmin":false}),
        serde_json::json!({"Name":"Display Testing","Desc":"Display testing tools","IconPath":"SystemAO/info/img/small_icon.png","Group":"Device","StartDir":"SystemAO/info/display.html","RequireAdmin":false}),
        serde_json::json!({"Name":"Language","Desc":"Set the display language of the system","IconPath":"SystemAO/info/img/small_icon.png","Group":"Device","StartDir":"SystemAO/info/locale.html","RequireAdmin":false}),
        serde_json::json!({"Name":"IoT Hub","Desc":"Manage IoT Devices Scanners","IconPath":"SystemAO/iot/img/small_icon.png","Group":"Device","StartDir":"SystemAO/iot/info.html","RequireAdmin":false}),
        serde_json::json!({"Name":"Module List","Desc":"A list of modules currently loaded in the system","IconPath":"SystemAO/system_setting/img/module.svg","Group":"Module","StartDir":"SystemAO/modules/moduleList.html","RequireAdmin":false}),
        serde_json::json!({"Name":"Default Module","Desc":"Default module used to open a file","IconPath":"SystemAO/system_setting/img/module.svg","Group":"Module","StartDir":"SystemAO/modules/defaultOpener.html","RequireAdmin":false}),
        serde_json::json!({"Name":"Subservices","Desc":"Launch and kill subservices","IconPath":"SystemAO/system_setting/img/module.svg","Group":"Module","StartDir":"SystemAO/modules/subservices.html","RequireAdmin":true}),
        serde_json::json!({"Name":"Add & Remove Module","Desc":"Install & Remove Module to the system","IconPath":"SystemAO/system_setting/img/module.svg","Group":"Module","StartDir":"SystemAO/modules/addAndRemove.html","RequireAdmin":true}),
        serde_json::json!({"Name":"AlpNAS Drives","Desc":"Browse, mount and open local AlpNAS storage devices","IconPath":"SystemAO/system_setting/img/drive.svg","Group":"Disk","StartDir":"SystemAO/disk/alpnas_drives.html","RequireAdmin":false}),
        serde_json::json!({"Name":"Space Finder","Desc":"Reclaim Storage Space on Disks","IconPath":"SystemAO/disk/space/img/small_icon.png","Group":"Disk","StartDir":"SystemAO/disk/space/finder.html","RequireAdmin":false}),
        serde_json::json!({"Name":"Disk Space","Desc":"System Storage Space on Disks","IconPath":"SystemAO/disk/space/img/small_icon.png","Group":"Disk","StartDir":"SystemAO/disk/space/diskspace.html","RequireAdmin":false}),
        serde_json::json!({"Name":"Storage Pools","Desc":"Storage Pool Mounting Configuration","IconPath":"SystemAO/disk/smart/img/small_icon.png","Group":"Disk","StartDir":"SystemAO/storage/poolList.html","RequireAdmin":true}),
        serde_json::json!({"Name":"Storage Quota","Desc":"User Remaining Space","IconPath":"SystemAO/disk/quota/img/small_icon.png","Group":"Disk","StartDir":"SystemAO/disk/quota/quota.html","RequireAdmin":false}),
        serde_json::json!({"Name":"Disk SMART","Desc":"HardDisk Health Checking","IconPath":"SystemAO/disk/smart/img/small_icon.png","Group":"Disk","StartDir":"SystemAO/disk/smart/smart.html","RequireAdmin":true}),
        serde_json::json!({"Name":"RAID","Desc":"Providing basic mdadm features","IconPath":"SystemAO/disk/raid/img/raid.svg","Group":"Disk","StartDir":"SystemAO/disk/raid/index.html","RequireAdmin":true}),
        serde_json::json!({"Name":"Network Info","Desc":"Network Information","IconPath":"SystemAO/network/img/ethernet.svg","Group":"Network","StartDir":"SystemAO/network/hardware.html","RequireAdmin":false}),
        serde_json::json!({"Name":"WiFi Info","Desc":"Current Connected WiFi Information","IconPath":"SystemAO/network/img/wifi.svg","Group":"Network","StartDir":"SystemAO/network/wifiinfo.html","RequireAdmin":false}),
        serde_json::json!({"Name":"WiFi Settings","Desc":"Setup WiFi Connections","IconPath":"SystemAO/network/img/wifi.svg","Group":"Network","StartDir":"SystemAO/network/wifi.html","RequireAdmin":true}),
        serde_json::json!({"Name":"File Servers","Desc":"Network File Transfer Servers","IconPath":"SystemAO/disk/smart/img/small_icon.png","Group":"Network","StartDir":"SystemAO/disk/services.html","RequireAdmin":false}),
        serde_json::json!({"Name":"Port Forward","Desc":"UPnP based port forwarding","IconPath":"SystemAO/system_setting/img/network.svg","Group":"Network","StartDir":"SystemAO/network/portforward.html","RequireAdmin":false}),
        serde_json::json!({"Name":"Personal Page","Desc":"Personal Web Page","IconPath":"SystemAO/www/img/homepage.png","Group":"Network","StartDir":"SystemAO/www/config.html","RequireAdmin":false}),
        serde_json::json!({"Name":"My Account","Desc":"Manage your account and password","IconPath":"SystemAO/users/img/users.svg","Group":"Users","StartDir":"SystemAO/users/account.html","RequireAdmin":false}),
        serde_json::json!({"Name":"User List","Desc":"A list of users registered on this system","IconPath":"SystemAO/users/img/users.svg","Group":"Users","StartDir":"SystemAO/users/userList.html","RequireAdmin":true}),
        serde_json::json!({"Name":"Permission Groups","Desc":"Handle the permission of access in groups","IconPath":"SystemAO/users/img/users.svg","Group":"Users","StartDir":"SystemAO/users/group.html","RequireAdmin":true}),
        serde_json::json!({"Name":"Public Registry","Desc":"Allow public users to create account in this host","IconPath":"SystemAO/users/img/users.svg","Group":"Users","StartDir":"SystemAO/users/pubreg.html","RequireAdmin":true}),
        serde_json::json!({"Name":"Neighbourhood","Desc":"Nearby ArozOS hosts for clustering","IconPath":"SystemAO/cluster/img/small_icon.png","Group":"Cluster","StartDir":"SystemAO/cluster/neighbour.html","RequireAdmin":false}),
        serde_json::json!({"Name":"Tasks Scheduler","Desc":"System Tasks and Execution Scheduler","IconPath":"SystemAO/arsm/img/small_icon.png","Group":"Cluster","StartDir":"SystemAO/arsm/aecron.html","RequireAdmin":false}),
        serde_json::json!({"Name":"Connection Log","Desc":"Logs for login attempts","IconPath":"SystemAO/security/img/small_icon.png","Group":"Security","StartDir":"SystemAO/security/connlog.html","RequireAdmin":true}),
        serde_json::json!({"Name":"Access Control","Desc":"Prevent / Allow certain IP ranges from logging in","IconPath":"SystemAO/security/img/small_icon.png","Group":"Security","StartDir":"SystemAO/security/accesscontrol.html","RequireAdmin":true}),
        serde_json::json!({"Name":"LDAP","Desc":"Allows external account access to system","IconPath":"SystemAO/advance/img/small_icon.png","Group":"Security","StartDir":"SystemAO/advance/ldap.html","RequireAdmin":true}),
        serde_json::json!({"Name":"OAuth","Desc":"Allows external account access to system","IconPath":"SystemAO/advance/img/small_icon.png","Group":"Security","StartDir":"SystemAO/advance/oauth.html","RequireAdmin":true}),
        serde_json::json!({"Name":"Auto Login Mode","Desc":"Allow bots logging into the system automatically","IconPath":"SystemAO/advance/img/small_icon.png","Group":"Advance","StartDir":"SystemAO/advance/autologin.html","RequireAdmin":true}),
        serde_json::json!({"Name":"Disk Manager","Desc":"Mount, Unmount and Formatting Local Disks","IconPath":"SystemAO/system_setting/img/drive.svg","Group":"Advance","StartDir":"SystemAO/disk/diskmg.html","RequireAdmin":true}),
        serde_json::json!({"Name":"System Log","Desc":"View AlpNAS system logs","IconPath":"SystemAO/updates/img/update.png","Group":"Advance","StartDir":"SystemAO/advance/logview.html","RequireAdmin":true}),
        serde_json::json!({"Name":"AlpNAS Installer","Desc":"Install AlpNAS from the live system onto a local disk","IconPath":"SystemAO/system_setting/img/code.svg","Group":"Advance","StartDir":"SystemAO/installer/alpnas_installer.html","RequireAdmin":true}),
        serde_json::json!({"Name":"AlpNAS","Desc":"About this AlpNAS system","IconPath":"SystemAO/info/img/small_icon.png","Group":"About","StartDir":"SystemAO/info/about.html","RequireAdmin":false}),
        serde_json::json!({"Name":"Open Source","Desc":"License from the Open Source Community","IconPath":"SystemAO/info/img/small_icon.png","Group":"About","StartDir":"SystemAO/info/license.html","RequireAdmin":false}),
        serde_json::json!({"Name":"License","Desc":"License of AlpNAS and embedded components","IconPath":"SystemAO/info/img/small_icon.png","Group":"About","StartDir":"SystemAO/info/srcLicense.html","RequireAdmin":false}),
        serde_json::json!({"Name":"Vendor","Desc":"Vendor Notes","IconPath":"SystemAO/info/img/small_icon.png","Group":"About","StartDir":"SystemAO/vendor/index.html","RequireAdmin":false})
    ]
}

fn settings_list(_state: &AppState, params: &HashMap<String,String>) -> Response {
    if let Some(group) = param(params, &["listGroup"]) {
        let items: Vec<_> = setting_modules().into_iter()
            .filter(|m| m.get("Group").and_then(|v| v.as_str()) == Some(group))
            .collect();
        if items.is_empty() { return err(StatusCode::NOT_FOUND, "Group not found"); }
        return ok(serde_json::json!(items));
    }
    ok(setting_groups())
}

fn scheduler_list(state: &AppState) -> Response { let jobs: Vec<ScheduledJob> = read_vec(state, "scheduler_jobs.json"); ok(serde_json::json!({"ok": true, "jobs": jobs})) }
fn scheduler_add(state: &AppState, params: &HashMap<String,String>) -> Response { let mut jobs: Vec<ScheduledJob> = read_vec(state, "scheduler_jobs.json"); let job = ScheduledJob { id: Uuid::new_v4().to_string(), name: param(params, &["name"]).unwrap_or("job").to_string(), command: param(params, &["command", "cmd"]).unwrap_or("").to_string(), schedule: param(params, &["schedule", "cron", "time"]).unwrap_or("daily").to_string(), enabled: !matches!(param(params, &["enabled"]), Some("false")), created_unix: now_unix(), logs: Vec::new() }; jobs.push(job.clone()); match write_vec(state, "scheduler_jobs.json", &jobs) { Ok(()) => ok(serde_json::json!({"ok": true, "job": job})), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e) } }
fn scheduler_remove(state: &AppState, params: &HashMap<String,String>) -> Response { let id = param(params, &["id", "job"]).unwrap_or(""); let mut jobs: Vec<ScheduledJob> = read_vec(state, "scheduler_jobs.json"); let old=jobs.len(); jobs.retain(|j| j.id != id && j.name != id); match write_vec(state, "scheduler_jobs.json", &jobs) { Ok(()) => ok(serde_json::json!({"ok": true, "removed": old-jobs.len()})), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e) } }
fn scheduler_log(state: &AppState, params: &HashMap<String,String>) -> Response { let id = param(params, &["id", "job"]).unwrap_or(""); let jobs: Vec<ScheduledJob> = read_vec(state, "scheduler_jobs.json"); let logs: Vec<_> = jobs.into_iter().filter(|j| j.id == id || j.name == id).flat_map(|j| j.logs).collect(); ok(serde_json::json!({"ok": true, "logs": logs})) }

fn storage_pool_list(state: &AppState) -> Response { let mut pools: Vec<StoragePool> = read_vec(state, "storage_pools.json"); if pools.is_empty() { pools.push(StoragePool { id:"local".into(), name:"Local Files".into(), path:state.config.root_directory.clone(), backend:"localfs".into(), enabled:true, bridged_to:None }); } ok(serde_json::json!({"ok": true, "pools": pools})) }
fn storage_pool_new(state: &AppState, params: &HashMap<String,String>) -> Response { let mut pools: Vec<StoragePool> = read_vec(state, "storage_pools.json"); let pool = StoragePool { id: param(params, &["id"]).map(str::to_string).unwrap_or_else(|| Uuid::new_v4().to_string()), name: param(params, &["name"]).unwrap_or("Storage Pool").to_string(), path: param(params, &["path", "root"]).unwrap_or(&state.config.root_directory).to_string(), backend: param(params, &["backend", "type"]).unwrap_or("localfs").to_string(), enabled: true, bridged_to: None }; pools.push(pool.clone()); match write_vec(state, "storage_pools.json", &pools) { Ok(()) => ok(serde_json::json!({"ok": true, "pool": pool})), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e) } }
fn storage_pool_remove(state: &AppState, params: &HashMap<String,String>) -> Response { let id = param(params, &["id", "name"]).unwrap_or(""); let mut pools: Vec<StoragePool> = read_vec(state, "storage_pools.json"); let old=pools.len(); pools.retain(|p| p.id != id && p.name != id); match write_vec(state, "storage_pools.json", &pools) { Ok(()) => ok(serde_json::json!({"ok": true, "removed": old-pools.len()})), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e) } }
fn storage_pool_edit(state: &AppState, params: &HashMap<String,String>) -> Response { let id = param(params, &["id", "name"]).unwrap_or(""); let mut pools: Vec<StoragePool> = read_vec(state, "storage_pools.json"); for p in &mut pools { if p.id==id || p.name==id { if let Some(name)=param(params, &["newName", "newname", "displayName"]) { p.name=name.into(); } if let Some(path)=param(params, &["path", "root"]) { p.path=path.into(); } if let Some(backend)=param(params, &["backend", "type"]) { p.backend=backend.into(); } } } match write_vec(state, "storage_pools.json", &pools) { Ok(()) => ok(serde_json::json!({"ok": true, "pools": pools})), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e) } }
fn storage_pool_toggle(state: &AppState, params: &HashMap<String,String>) -> Response { let id = param(params, &["id", "name"]).unwrap_or(""); let enabled = flag(params); let mut pools: Vec<StoragePool> = read_vec(state, "storage_pools.json"); for p in &mut pools { if p.id==id || p.name==id { p.enabled = enabled; } } match write_vec(state, "storage_pools.json", &pools) { Ok(()) => ok(serde_json::json!({"ok": true, "pools": pools})), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e) } }
fn storage_pool_bridge(state: &AppState, params: &HashMap<String,String>) -> Response { let id = param(params, &["id", "name"]).unwrap_or(""); let target = param(params, &["target", "bridge"]).map(str::to_string); let mut pools: Vec<StoragePool> = read_vec(state, "storage_pools.json"); for p in &mut pools { if p.id==id || p.name==id { p.bridged_to = target.clone(); } } match write_vec(state, "storage_pools.json", &pools) { Ok(()) => ok(serde_json::json!({"ok": true, "pools": pools})), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e) } }
fn storage_pool_check_bridge(state: &AppState, params: &HashMap<String,String>) -> Response { let id = param(params, &["id", "name"]).unwrap_or(""); let pools: Vec<StoragePool> = read_vec(state, "storage_pools.json"); let bridge = pools.iter().find(|p| p.id==id || p.name==id).and_then(|p| p.bridged_to.clone()); ok(serde_json::json!({"ok": true, "bridge": bridge})) }

fn service_file(name: &str) -> String { format!("service_{}.json", name) }
fn service_load(state: &AppState, name: &str, default_port: Option<u16>) -> ServiceConfig {
    utils::read_json_file(utils::data_file(&state.config.system_root, &service_file(name)))
        .unwrap_or(ServiceConfig { name: name.into(), enabled: false, port: default_port, upnp: false, options: serde_json::json!({}) })
}
fn service_save(state: &AppState, cfg: &ServiceConfig) -> Result<(), String> {
    utils::write_json_file(utils::data_file(&state.config.system_root, &service_file(&cfg.name)), cfg).map_err(|e| e.to_string())
}

fn requested_service_enabled(params: &HashMap<String,String>) -> Option<bool> {
    let raw = param(params, &["set", "enable", "enabled", "status", "value", "on"])?;
    let value = raw.trim().to_ascii_lowercase();
    Some(matches!(value.as_str(), "1" | "true" | "yes" | "on" | "enable" | "enabled" | "start" | "started"))
}

fn default_service_port(name: &str) -> Option<u16> {
    match name { "ftp" => Some(2121), "tftp" => Some(6969), "sftp" => Some(22), "telnet" => Some(23), _ => None }
}
fn ftp_status_value(state: &AppState) -> serde_json::Value {
    let cfg = service_load(state, "ftp", Some(2121));
    let native = service_adapter::service_status("ftp", cfg.enabled);
    serde_json::json!({
        "Enabled": cfg.enabled || protocol_services::runtime_status(state, "ftp").running,
        "ConfiguredEnabled": cfg.enabled,
        "Port": cfg.port.unwrap_or(2121),
        "AllowUPNP": state.config.allow_upnp,
        "FTPUpnpEnabled": cfg.upnp,
        "PassiveMode": cfg.options.get("passive").and_then(|v| v.as_bool()).unwrap_or(false),
        "PublicAddr": cfg.options.get("ip").and_then(|v| v.as_str()).unwrap_or(""),
        "UserGroups": cfg.options.get("groups").cloned().unwrap_or_else(|| serde_json::json!(["administrator", "user"])),
        "NativeAvailable": true,
        "NativeActive": protocol_services::runtime_status(state, "ftp").running,
        "ServiceManager": "builtin-rust",
        "ServiceName": "arozos-rs-ftp",
        "PackageHint": native.package_hint
    })
}
fn service_status(state: &AppState, name: &str, default_port: Option<u16>) -> Response {
    match name {
        "ftp" => {
            let mut value = ftp_status_value(state);
            if let serde_json::Value::Object(ref mut obj) = value {
                obj.insert("Runtime".into(), serde_json::json!(protocol_services::runtime_status(state, "ftp")));
                obj.insert("Implementation".into(), serde_json::json!("builtin-rust-ftp"));
            }
            ok(value)
        },
        "samba" => ok(serde_json::json!(service_load(state, "samba", None).enabled)),
        "tftp" => {
            let cfg = service_load(state, "tftp", Some(6969));
            let native = service_adapter::service_status("tftp", cfg.enabled);
            let runtime = protocol_services::runtime_status(state, "tftp");
            ok(serde_json::json!({"Enabled": cfg.enabled || runtime.running || native.active, "ConfiguredEnabled": cfg.enabled, "Port": cfg.port.unwrap_or(6969), "DefaultUser": cfg.options.get("defaultUser").and_then(|v| v.as_str()).unwrap_or(""), "AllowUPNP": false, "NativeAvailable": true, "NativeActive": runtime.running, "ServiceManager": "builtin-rust", "Runtime": runtime, "OSNative": native}))
        },
        "sftp" => {
            let cfg = service_load(state, "sftp", Some(22));
            let native = service_adapter::service_status("sftp", cfg.enabled);
            ok(serde_json::json!({"Enabled": cfg.enabled, "ConfiguredEnabled": cfg.enabled, "Port": cfg.port.unwrap_or(22), "AllowUPNP": state.config.allow_upnp, "UpnpEnabled": cfg.upnp, "NativeAvailable": native.native_available, "NativeActive": native.active, "ServiceManager": native.manager, "ServiceName": native.primary_service_name, "PackageHint": native.package_hint, "Runtime": protocol_services::runtime_status(state, "sftp"), "Implementation": "os-sshd-sftp-subsystem"}))
        },
        "telnet" => {
            let cfg = service_load(state, "telnet", Some(23));
            let native = service_adapter::service_status("telnet", cfg.enabled);
            ok(serde_json::json!({"Enabled": cfg.enabled, "ConfiguredEnabled": cfg.enabled, "Port": cfg.port.unwrap_or(23), "NativeAvailable": native.native_available, "NativeActive": native.active, "ServiceManager": native.manager, "ServiceName": native.primary_service_name, "PackageHint": native.package_hint, "Implementation":"os-telnetd-adapter", "Warning":"Telnet is insecure; prefer SFTP/SSH unless you need legacy device compatibility."}))
        },
        _ => ok(serde_json::json!(service_load(state, name, default_port)))
    }
}
fn service_toggle(state: &AppState, name: &str, enabled: bool, params: &HashMap<String,String>) -> Response {
    let default = default_service_port(name);
    let mut cfg = service_load(state, name, default);
    cfg.enabled = requested_service_enabled(params).unwrap_or(enabled);
    let action = if cfg.enabled { "start" } else { "stop" };
    let native = service_adapter::perform_service_action(name, action);
    let runtime = protocol_services::apply_service_change(state, name, cfg.enabled);
    let mut obj = cfg.options.as_object().cloned().unwrap_or_default();
    obj.insert("lastNativeAction".into(), serde_json::json!(native));
    obj.insert("lastRuntimeAction".into(), serde_json::json!(runtime));
    cfg.options = serde_json::Value::Object(obj);
    match service_save(state, &cfg) { Ok(()) => text("OK"), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e) }
}
fn service_set_port(state: &AppState, name: &str, params: &HashMap<String,String>) -> Response {
    let default = default_service_port(name);
    let mut cfg = service_load(state, name, default);
    if let Some(port) = param(params, &["port"]).and_then(|p| p.parse().ok()) { cfg.port = Some(port); }
    let runtime = if cfg.enabled { Some(protocol_services::apply_service_change(state, name, true)) } else { None };
    let mut obj = cfg.options.as_object().cloned().unwrap_or_default();
    if let Some(runtime) = runtime { obj.insert("lastRuntimeAction".into(), serde_json::json!(runtime)); }
    cfg.options = serde_json::Value::Object(obj);
    match service_save(state, &cfg) { Ok(()) => text("OK"), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e) }
}

fn sftp_port(state: &AppState, params: &HashMap<String,String>) -> Response {
    let mut cfg = service_load(state, "sftp", Some(22));
    if let Some(port) = param(params, &["port"]).and_then(|p| p.parse::<u16>().ok()) {
        cfg.port = Some(port);
        return match service_save(state, &cfg) { Ok(()) => text("OK"), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e) };
    }
    text(&cfg.port.unwrap_or(22).to_string())
}

fn sftp_upnp(state: &AppState, params: &HashMap<String,String>) -> Response {
    let mut cfg = service_load(state, "sftp", Some(22));
    if param_ref(params, &["enabled", "enable", "upnp"]).is_some() {
        cfg.upnp = utils::truthy(param_ref(params, &["enabled", "enable", "upnp"]));
        return match service_save(state, &cfg) { Ok(()) => text("OK"), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e) };
    }
    ok(serde_json::json!(cfg.upnp))
}

fn sftp_users(_state: &AppState) -> Response { text("0") }

fn tftp_default_user(state: &AppState, params: &HashMap<String,String>) -> Response {
    let mut cfg = service_load(state, "tftp", Some(6969));
    let mut obj = cfg.options.as_object().cloned().unwrap_or_default();
    if let Some(user) = param(params, &["defaultUser", "default", "user", "username"]) {
        obj.insert("defaultUser".into(), serde_json::Value::String(user.to_string()));
        cfg.options = serde_json::Value::Object(obj);
        return match service_save(state, &cfg) { Ok(()) => text("OK"), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e) };
    }
    let user = obj.get("defaultUser").and_then(|v| v.as_str()).unwrap_or("user");
    text(user)
}
fn service_update(state: &AppState, name: &str, params: &HashMap<String,String>) -> Response {
    let default = default_service_port(name);
    let mut cfg = service_load(state, name, default);
    if let Some(set) = param(params, &["set"]) {
        match set {
            "enable" => cfg.enabled = true,
            "disable" => cfg.enabled = false,
            "mode" => {
                let mut obj = cfg.options.as_object().cloned().unwrap_or_default();
                obj.insert("passive".into(), serde_json::Value::Bool(utils::truthy(param_ref(params, &["passive"]))));
                cfg.options = serde_json::Value::Object(obj);
            },
            "ip" => {
                let mut obj = cfg.options.as_object().cloned().unwrap_or_default();
                obj.insert("ip".into(), serde_json::Value::String(param(params, &["ip"]).unwrap_or("").to_string()));
                cfg.options = serde_json::Value::Object(obj);
            },
            _ => {}
        }
    }
    if let Some(requested) = requested_service_enabled(params) { cfg.enabled = requested; }
    if let Some(port) = param(params, &["port"]).and_then(|p| p.parse().ok()) { cfg.port = Some(port); }
    if param_ref(params, &["upnp"]).is_some() || param_ref(params, &["enable"]).is_some() { cfg.upnp = utils::truthy(param_ref(params, &["upnp", "enable"])); }
    let mut obj = cfg.options.as_object().cloned().unwrap_or_default();
    if let Some(groups) = param(params, &["groups"]) {
        let parsed: serde_json::Value = serde_json::from_str(groups).unwrap_or_else(|_| serde_json::json!(groups.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect::<Vec<_>>()));
        obj.insert("groups".into(), parsed);
    }
    if let Some(user) = param(params, &["defaultUser", "default"]){ obj.insert("defaultUser".into(), serde_json::Value::String(user.to_string())); }
    obj.insert("lastRuntimeAction".into(), serde_json::json!(protocol_services::apply_service_change(state, name, cfg.enabled)));
    obj.insert("lastNativeAction".into(), serde_json::json!(service_adapter::perform_service_action(name, if cfg.enabled { "start" } else { "stop" })));
    if !obj.is_empty() { cfg.options = serde_json::Value::Object(obj); }
    match service_save(state, &cfg) { Ok(()) => text("OK"), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e) }
}


fn service_config_plan(state: &AppState, id: &str) -> Response {
    let name = match id { "smbd" | "smb" => "samba", "ssh" | "sshd" => "sftp", other => other };
    let default = default_service_port(name);
    let cfg = service_load(state, name, default);
    let status = service_adapter::service_status(name, cfg.enabled);
    let port = cfg.port.unwrap_or(match name { "ftp" => 2121, "sftp" => 22, "tftp" => 6969, "telnet" => 23, _ => 0 });
    let shares = if name == "samba" { samba_shares_or_default(state).iter().map(samba_share_json).collect::<Vec<_>>() } else { Vec::new() };
    let config = match name {
        "samba" => format!("[global]\n   workgroup = WORKGROUP\n   map to guest = Bad User\n   security = user\n{}", samba_shares_or_default(state).iter().map(|s| format!("\n[{}]\n   path = {}\n   read only = {}\n   browseable = yes\n   guest ok = no\n   valid users = {}\n", s.name, s.path, if s.readonly { "yes" } else { "no" }, s.users.join(" "))).collect::<Vec<_>>().join("")),
        "ftp" => format!("listen=YES\nlisten_port={}\nlocal_enable=YES\nwrite_enable=YES\nchroot_local_user=YES\n", port),
        "sftp" => format!("Subsystem sftp internal-sftp\nMatch Group arozos-sftp\n    ChrootDirectory %h\n    ForceCommand internal-sftp -d /\n    AllowTcpForwarding no\n    X11Forwarding no\n# Port {} is configured in ArozOS; sshd may require a matching Port line.\n", port),
        "webdav" => format!("Rust WebDAV native endpoint: http://<host>:{}/webdav/\nThis is the canonical WebDAV mount, not an alias.\n", state.config.listen_port),
        _ => String::new(),
    };
    ok(serde_json::json!({
        "ok": true,
        "service": name,
        "configuredEnabled": cfg.enabled,
        "native": status,
        "addresses": service_endpoint_addresses(),
        "port": port,
        "shares": shares,
        "configPreview": config,
        "note": "Rust writes ArozOS service state. Native OS start/stop is planned unless AROZOS_RS_ENABLE_SYSTEM_SERVICE_OPS=1 is set."
    }))
}

fn service_config_enabled(state: &AppState, name: &str) -> bool {
    let default = default_service_port(name);
    service_load(state, name, default).enabled
}

fn os_service_adapter_info(_state: &AppState) -> Response {
    ok(serde_json::json!({
        "ok": true,
        "adapter": service_adapter::adapter_info(),
        "addresses": service_adapter::local_addresses(),
        "services": service_adapter::all_statuses(&[
            ("webdav", false),
            ("sftp", false),
            ("ftp", false),
            ("tftp", false),
            ("samba", false),
            ("telnet", false),
        ])
    }))
}

fn os_service_status_named(state: &AppState, name: &str) -> Response {
    ok(serde_json::json!(service_adapter::service_status(name, service_config_enabled(state, name))))
}

fn os_service_status(state: &AppState, params: &HashMap<String,String>) -> Response {
    let name = param(params, &["id", "service", "name"]).unwrap_or("samba");
    os_service_status_named(state, name)
}

fn native_service_operate_named(state: &AppState, name: &str, action: &str) -> Response {
    let mut cfg = service_load(state, name, default_service_port(name));
    if matches!(action, "start" | "enable" | "activate") { cfg.enabled = true; }
    if matches!(action, "stop" | "disable" | "deactivate") { cfg.enabled = false; }
    let result = service_adapter::perform_service_action(name, action);
    let mut obj = cfg.options.as_object().cloned().unwrap_or_default();
    obj.insert("lastNativeAction".into(), serde_json::json!(result.clone()));
    cfg.options = serde_json::Value::Object(obj);
    let _ = service_save(state, &cfg);
    ok(serde_json::json!({"ok": result.ok, "result": result, "status": service_adapter::service_status(name, cfg.enabled)}))
}

fn network_server_operate(state: &AppState, params: &HashMap<String,String>) -> Response {
    let id = param(params, &["id", "service", "name"]).unwrap_or("samba");
    let action = param(params, &["action", "opr", "set"]).unwrap_or("status");
    native_service_operate_named(state, id, action)
}

fn service_catalog(state: &AppState) -> Vec<serde_json::Value> {
    let ftp = service_load(state, "ftp", Some(2121));
    let sftp = service_load(state, "sftp", Some(22));
    let tftp = service_load(state, "tftp", Some(6969));
    let samba = service_load(state, "samba", None);
    let webdav = service_load(state, "webdav", None);
    let telnet = service_load(state, "telnet", Some(23));
    let dirserv = service_load(state, "dirserv", None);
    let ftp_os = service_adapter::service_status("ftp", ftp.enabled);
    let sftp_os = service_adapter::service_status("sftp", sftp.enabled);
    let tftp_os = service_adapter::service_status("tftp", tftp.enabled);
    let samba_os = service_adapter::service_status("samba", samba.enabled);
    let webdav_os = service_adapter::service_status("webdav", webdav.enabled);
    let telnet_os = service_adapter::service_status("telnet", telnet.enabled);
    vec![
        serde_json::json!({"ID":"webdav","Name":"WebDAV","Desc":"WebDAV Server","IconPath":"img/system/network-folder-blue.svg","DefaultPorts":[],"Ports":[state.config.listen_port],"ForwardPortIfUpnp":false,"ConnInstrPage":"SystemAO/disk/instr/webdav.html","ConfigPage":"SystemAO/disk/webdav.html","Enabled":webdav.enabled,"NativeAvailable":webdav_os.native_available,"ServiceManager":webdav_os.manager,"PackageHint":webdav_os.package_hint}),
        serde_json::json!({"ID":"sftp","Name":"SFTP","Desc":"SSH File Transfer Protocol Server","IconPath":"img/system/network-folder-sftp.svg","DefaultPorts":[22],"Ports":[sftp.port.unwrap_or(22)],"ForwardPortIfUpnp":true,"ConnInstrPage":"SystemAO/disk/instr/sftp.html","ConfigPage":"SystemAO/disk/sftp.html","Enabled":sftp.enabled,"NativeAvailable":sftp_os.native_available,"NativeActive":sftp_os.active,"ServiceManager":sftp_os.manager,"PackageHint":sftp_os.package_hint}),
        serde_json::json!({"ID":"ftp","Name":"FTP","Desc":"File Transfer Protocol Server","IconPath":"img/system/network-folder.svg","DefaultPorts":[2121,21],"Ports":[ftp.port.unwrap_or(2121)],"ForwardPortIfUpnp":true,"ConnInstrPage":"SystemAO/disk/instr/ftp.html","ConfigPage":"SystemAO/disk/ftp.html","Enabled":ftp.enabled || protocol_services::runtime_status(state, "ftp").running,"NativeAvailable":true,"ServiceManager":"builtin-rust","PackageHint":ftp_os.package_hint}),
        serde_json::json!({"ID":"tftp","Name":"TFTP","Desc":"Trivial File Transfer Protocol Server","IconPath":"img/system/network-folder-black.svg","DefaultPorts":[6969,69],"Ports":[tftp.port.unwrap_or(6969)],"ForwardPortIfUpnp":false,"ConnInstrPage":"SystemAO/disk/instr/tftp.html","ConfigPage":"SystemAO/disk/tftp.html","Enabled":tftp.enabled || protocol_services::runtime_status(state, "tftp").running,"NativeAvailable":true,"ServiceManager":"builtin-rust","PackageHint":tftp_os.package_hint}),
        serde_json::json!({"ID":"dirserv","Name":"Directory Server","Desc":"Web file viewer for legacy devices","IconPath":"img/system/network-dirserv.svg","DefaultPorts":[],"Ports":[state.config.listen_port],"ForwardPortIfUpnp":false,"ConnInstrPage":"SystemAO/disk/instr/dirserv.html","ConfigPage":"SystemAO/disk/dirserv.html","Enabled":dirserv.enabled}),
        serde_json::json!({"ID":"smbd","Name":"Samba Shares","Desc":"Share local files via SMB using Samba","IconPath":"img/system/network-samba.svg","DefaultPorts":[445,139],"Ports":[445,139],"ForwardPortIfUpnp":false,"ConnInstrPage":"SystemAO/disk/instr/samba.html","ConfigPage":"SystemAO/disk/samba.html","Enabled":samba.enabled,"NativeAvailable":samba_os.native_available,"NativeActive":samba_os.active,"ServiceManager":samba_os.manager,"PackageHint":samba_os.package_hint}),
        serde_json::json!({"ID":"telnet","Name":"Telnet","Desc":"Legacy Telnet Server (insecure)","IconPath":"img/system/network.svg","DefaultPorts":[23],"Ports":[telnet.port.unwrap_or(23)],"ForwardPortIfUpnp":true,"ConnInstrPage":"SystemAO/disk/instr/telnet.html","ConfigPage":"SystemAO/disk/telnet.html","Enabled":telnet.enabled,"NativeAvailable":telnet_os.native_available,"NativeActive":telnet_os.active,"ServiceManager":telnet_os.manager,"PackageHint":telnet_os.package_hint})
    ]
}
fn network_server_list(state: &AppState) -> Response { ok(serde_json::json!(service_catalog(state))) }
fn network_server_status(state: &AppState) -> Response {
    let mut map = serde_json::Map::new();
    for s in service_catalog(state) {
        let id = s.get("ID").and_then(|v| v.as_str()).unwrap_or("");
        let en = s.get("Enabled").and_then(|v| v.as_bool()).unwrap_or(false);
        map.insert(id.to_string(), serde_json::Value::Bool(en));
    }
    ok(serde_json::Value::Object(map))
}
fn service_endpoint_addresses() -> Vec<String> {
    let mut addresses = service_adapter::local_addresses();
    addresses.retain(|a| !a.trim().is_empty() && !a.starts_with("169.254."));
    addresses.sort();
    addresses.dedup();
    let mut preferred: Vec<String> = addresses.iter()
        .filter(|a| !a.starts_with("127.") && *a != "::1" && !a.starts_with("fe80:"))
        .cloned()
        .collect();
    if preferred.is_empty() { preferred.push("127.0.0.1".into()); }
    if preferred.len() > 3 { preferred.truncate(3); }
    preferred
}

fn network_server_endpoints(state: &AppState) -> Response {
    let host = state.config.host_name.as_str();
    let port = state.config.listen_port;
    let addresses = service_endpoint_addresses();
    let mk = |proto: &str, p: u16, sub: &str| -> Vec<serde_json::Value> {
        addresses.iter().map(|addr| serde_json::json!({
            "ProtocolName": proto,
            "Host": addr,
            "Port": p,
            "Subpath": sub,
            "URL": format!("{}{}:{}{}", proto, addr, p, sub)
        })).collect()
    };
    ok(serde_json::json!({
        "webdav": mk("http://", port, "/webdav/"),
        "sftp": mk("sftp://", service_load(state,"sftp",Some(22)).port.unwrap_or(22), "/"),
        "ftp": mk("ftp://", service_load(state,"ftp",Some(2121)).port.unwrap_or(2121), "/"),
        "tftp": mk("tftp://", service_load(state,"tftp",Some(6969)).port.unwrap_or(6969), "/"),
        "telnet": mk("telnet://", service_load(state,"telnet",Some(23)).port.unwrap_or(23), "/"),
        "dirserv": mk("http://", port, "/share/"),
        "smbd": addresses.iter().map(|addr| serde_json::json!({
            "ProtocolName":"smb://",
            "Host": addr,
            "Port": 0,
            "Subpath": format!("/{}/", host),
            "URL": format!("smb://{}/", addr)
        })).collect::<Vec<_>>()
    }))
}
fn network_server_toggle(state: &AppState, params: &HashMap<String,String>) -> Response {
    let id = param(params, &["id", "service", "name"]).unwrap_or("");
    let name = match id { "smbd" => "samba", "webdav" => "webdav", "ftp" => "ftp", "sftp" => "sftp", "tftp" => "tftp", "telnet" => "telnet", "dirserv" => "dirserv", _ => id };
    if name.is_empty() { return err(StatusCode::BAD_REQUEST, "invalid service id given"); }
    let default = default_service_port(name);
    let mut cfg = service_load(state, name, default);
    cfg.enabled = requested_service_enabled(params).unwrap_or(false);
    let action = if cfg.enabled { "start" } else { "stop" };
    let native = service_adapter::perform_service_action(name, action);
    let runtime = protocol_services::apply_service_change(state, name, cfg.enabled);
    let mut obj = cfg.options.as_object().cloned().unwrap_or_default();
    obj.insert("lastNativeAction".into(), serde_json::json!(native));
    obj.insert("lastRuntimeAction".into(), serde_json::json!(runtime));
    cfg.options = serde_json::Value::Object(obj);
    match service_save(state, &cfg) { Ok(()) => text("OK"), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e) }
}
fn webdav_status(state: &AppState) -> Response { ok(serde_json::json!([service_load(state,"webdav",None).enabled, true])) }

fn webdav_client_json(c: &WebDavClient) -> serde_json::Value {
    serde_json::json!({
        "UUID": c.uuid,
        "ClientIP": c.client_ip,
        "Username": c.username,
        "LastConnectionTimestamp": c.last_connection_unix,
        "Allowed": c.allowed
    })
}

fn webdav_list(state: &AppState, params: &HashMap<String,String>) -> Response {
    let clients: Vec<WebDavClient> = read_vec(state, "webdav_clients.json");
    let want_allowed = matches!(param(params, &["target"]), Some("loggedin") | Some("allowed") | Some("permitted"));
    let out: Vec<_> = clients.into_iter().filter(|c| c.allowed == want_allowed).map(|c| webdav_client_json(&c)).collect();
    ok(serde_json::json!(out))
}

fn webdav_mutation(state: &AppState, params: &HashMap<String,String>) -> Response {
    let mut cfg = service_load(state, "webdav", None);
    if let Some(set) = param(params, &["set"]) {
        if set == "enable" { cfg.enabled = true; }
        if set == "disable" { cfg.enabled = false; }
        let mut obj = cfg.options.as_object().cloned().unwrap_or_default();
        obj.insert("lastRuntimeAction".into(), serde_json::json!(protocol_services::apply_service_change(state, "webdav", cfg.enabled)));
        cfg.options = serde_json::Value::Object(obj);
        return match service_save(state, &cfg) { Ok(()) => text("OK"), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e) };
    }
    let mut clients: Vec<WebDavClient> = read_vec(state, "webdav_clients.json");
    match param(params, &["opr", "action"]).unwrap_or("") {
        "set" | "allow" => {
            let uuid = param(params, &["uuid", "id"]).unwrap_or("");
            for c in &mut clients { if c.uuid == uuid { c.allowed = true; } }
        },
        "remove" | "deny" => {
            let uuid = param(params, &["uuid", "id"]).unwrap_or("");
            for c in &mut clients { if c.uuid == uuid { c.allowed = false; } }
        },
        "clear" => clients.retain(|c| c.allowed),
        _ => {}
    }
    match write_vec(state, "webdav_clients.json", &clients) { Ok(()) => text("OK"), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e) }
}


fn webdav_access_config(state: &AppState, route: &str, params: &HashMap<String,String>, json_body: Option<serde_json::Value>) -> Response {
    let mut cfg: webdav_server::WebDavAccessConfig = utils::read_json_file(utils::data_file(&state.config.system_root, "webdav_access.json")).unwrap_or_default();
    if let Some(body) = json_body {
        if let Ok(next) = serde_json::from_value::<webdav_server::WebDavAccessConfig>(body.clone()) {
            cfg = next;
            return match webdav_server::save_access_config(state, &cfg) { Ok(()) => ok(serde_json::json!({"ok": true, "config": cfg})), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e) };
        }
        if let Some(c) = body.get("config").cloned().and_then(|v| serde_json::from_value::<webdav_server::WebDavAccessConfig>(v).ok()) {
            cfg = c;
            return match webdav_server::save_access_config(state, &cfg) { Ok(()) => ok(serde_json::json!({"ok": true, "config": cfg})), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e) };
        }
    }
    let action = param(params, &["opr", "action", "set"]).unwrap_or("");
    let path = param(params, &["path", "vpath", "dir", "target"]).unwrap_or("user:/").trim().trim_end_matches('/').to_string();
    let mut changed = false;
    if route.ends_with("addPath") || matches!(action, "addAllowed" | "allowPath" | "allow") {
        if !cfg.allowed_paths.iter().any(|p| p == &path) { cfg.allowed_paths.push(path.clone()); changed = true; }
    } else if route.ends_with("removePath") || matches!(action, "removeAllowed" | "denyPath" | "remove") {
        cfg.allowed_paths.retain(|p| p != &path); changed = true;
    } else if matches!(action, "addDenied" | "block") {
        if !cfg.denied_paths.iter().any(|p| p == &path) { cfg.denied_paths.push(path.clone()); changed = true; }
    } else if matches!(action, "removeDenied" | "unblock") {
        cfg.denied_paths.retain(|p| p != &path); changed = true;
    } else if route.ends_with("addRule") || matches!(action, "addRule" | "setRule") {
        let users = param(params, &["users", "user"]).map(parse_users_param).unwrap_or_default();
        let groups = param(params, &["groups", "group"]).map(parse_users_param).unwrap_or_default();
        let read = param_ref(params, &["read"]).map(|v| utils::truthy(Some(v))).unwrap_or(true);
        let write = param_ref(params, &["write"]).map(|v| utils::truthy(Some(v))).unwrap_or(false);
        cfg.rules.retain(|r| r.path != path || r.users != users || r.groups != groups);
        cfg.rules.push(webdav_server::WebDavRule { path: path.clone(), users, groups, read, write });
        changed = true;
    } else if route.ends_with("removeRule") || matches!(action, "removeRule" | "delRule") {
        let before = cfg.rules.len();
        cfg.rules.retain(|r| r.path != path);
        changed = before != cfg.rules.len();
    } else {
        if let Some(v) = param_ref(params, &["requireAuth"]) { cfg.require_auth = utils::truthy(Some(v)); changed = true; }
        if let Some(v) = param_ref(params, &["allowAdminAll"]) { cfg.allow_admin_all = utils::truthy(Some(v)); changed = true; }
        if let Some(v) = param_ref(params, &["allowBrowserIndex"]) { cfg.allow_browser_index = utils::truthy(Some(v)); changed = true; }
    }
    if changed {
        return match webdav_server::save_access_config(state, &cfg) { Ok(()) => ok(serde_json::json!({"ok": true, "config": cfg})), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e) };
    }
    ok(serde_json::json!({
        "ok": true,
        "config": cfg,
        "notes": [
            "allowed_paths are WebDAV roots visible under /webdav/",
            "denied_paths always override allowed paths",
            "rules are matched by virtual path prefix, users and groups; write=false makes a path read-only",
            "Default path namespace is user:/; host:/ is not exposed through WebDAV unless explicitly allowed and mapped in the file API"
        ]
    }))
}

fn nginx_reverse_proxy_config(state: &AppState, params: &HashMap<String,String>) -> Response {
    let domain = param(params, &["domain", "host", "server_name"]).unwrap_or("arozos.local");
    let cert = param(params, &["cert", "ssl_certificate"]).unwrap_or("/etc/letsencrypt/live/arozos/fullchain.pem");
    let key = param(params, &["key", "ssl_certificate_key"]).unwrap_or("/etc/letsencrypt/live/arozos/privkey.pem");
    let upstream = param(params, &["upstream"]).map(str::to_string).unwrap_or_else(|| format!("127.0.0.1:{}", state.config.listen_port));
    let conf = format!(r#"server {{
    listen 80;
    server_name {domain};
    return 301 https://$host$request_uri;
}}

server {{
    listen 443 ssl http2;
    server_name {domain};

    ssl_certificate {cert};
    ssl_certificate_key {key};
    client_max_body_size 0;

    proxy_read_timeout 3600s;
    proxy_send_timeout 3600s;

    location / {{
        proxy_pass http://{upstream};
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto https;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
    }}

    location /webdav/ {{
        proxy_pass http://{upstream}/webdav/;
        proxy_http_version 1.1;
        proxy_request_buffering off;
        proxy_buffering off;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto https;
        proxy_set_header Destination $http_destination;
        proxy_set_header Overwrite $http_overwrite;
        proxy_set_header Depth $http_depth;
    }}
}}
"#);
    ok(serde_json::json!({"ok": true, "implementation":"nginx-reverse-proxy", "domain": domain, "upstream": upstream, "config": conf, "target_paths":{"debian":"/etc/nginx/sites-available/arozos-rust.conf","generic":"/etc/nginx/conf.d/arozos-rust.conf"}, "enable_commands":["sudo ./scripts/setup_nginx_https_reverse_proxy.sh <domain> 127.0.0.1:8080", "sudo nginx -t", "sudo systemctl reload nginx"]}))
}

fn nginx_write_config(state: &AppState, params: &HashMap<String,String>) -> Response {
    let domain = param(params, &["domain", "host", "server_name"]).unwrap_or("arozos.local");
    let cert = param(params, &["cert", "ssl_certificate"]).unwrap_or("/etc/letsencrypt/live/arozos/fullchain.pem");
    let key = param(params, &["key", "ssl_certificate_key"]).unwrap_or("/etc/letsencrypt/live/arozos/privkey.pem");
    let upstream = param(params, &["upstream"]).map(str::to_string).unwrap_or_else(|| format!("127.0.0.1:{}", state.config.listen_port));
    let conf = nginx_config_string(domain, cert, key, &upstream);
    let path = utils::data_file(&state.config.system_root, "nginx_arozos_reverse_proxy.conf");
    match fs::write(&path, conf) { Ok(()) => ok(serde_json::json!({"ok": true, "written": path.to_string_lossy(), "note":"Copy this file to /etc/nginx/sites-available/arozos-rust.conf or use the included script."})), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()) }
}

fn nginx_config_string(domain: &str, cert: &str, key: &str, upstream: &str) -> String {
    format!(r#"server {{
    listen 80;
    server_name {domain};
    return 301 https://$host$request_uri;
}}
server {{
    listen 443 ssl http2;
    server_name {domain};
    ssl_certificate {cert};
    ssl_certificate_key {key};
    client_max_body_size 0;
    location / {{ proxy_pass http://{upstream}; proxy_http_version 1.1; proxy_set_header Host $host; proxy_set_header X-Forwarded-Proto https; proxy_set_header Upgrade $http_upgrade; proxy_set_header Connection "upgrade"; }}
    location /webdav/ {{ proxy_pass http://{upstream}/webdav/; proxy_http_version 1.1; proxy_request_buffering off; proxy_buffering off; proxy_set_header Destination $http_destination; proxy_set_header Depth $http_depth; }}
}}
"#)
}


fn https_status(state: &AppState) -> Response {
    ok(serde_json::json!({
        "ok": true,
        "runtime": "native-rust-rustls",
        "status": https_native::status(&state.config),
        "https_url": format!("https://{}:{}/", if state.config.listen_host == "0.0.0.0" || state.config.listen_host.is_empty() { "<host-ip>" } else { &state.config.listen_host }, state.config.tls_listen_port),
    }))
}

fn https_config_preview(state: &AppState, params: &HashMap<String,String>) -> Response {
    let host = param(params, &["host", "domain", "name"]).unwrap_or(&state.config.host_name);
    let https_port = param(params, &["https_port", "port"]).and_then(|p| p.parse::<u16>().ok()).unwrap_or(state.config.tls_listen_port);
    let http_port = state.config.listen_port;
    let cert = param(params, &["cert", "tls_cert"]).unwrap_or(&state.config.tls_cert);
    let key = param(params, &["key", "tls_key"]).unwrap_or(&state.config.tls_key);
    ok(serde_json::json!({
        "ok": true,
        "implementation": "native-rust-https-proxy",
        "tls_backend": "rustls",
        "certificate_provider": "auto self-signed or manually supplied PEM",
        "host": host,
        "http_port": http_port,
        "https_port": https_port,
        "cert": cert,
        "key": key,
        "start_commands": {
            "auto_local_cert": format!("./target/release/arozos-rs --host 0.0.0.0 --port {http_port} --https --https-port {https_port} --auto_tls --redirect-http-to-https --cert-san {host}"),
            "manual_cert": format!("./target/release/arozos-rs --host 0.0.0.0 --port {http_port} --https --https-port {https_port} --cert {cert} --key {key} --redirect-http-to-https"),
            "https_only": format!("./target/release/arozos-rs --host 0.0.0.0 --port {http_port} --https --https-port {https_port} --auto_tls --disable_http --cert-san {host}")
        },
        "notes": [
            "This replaces Nginx for HTTPS termination inside the Rust binary.",
            "The auto_tls certificate is self-signed and local. Browsers will warn until you trust it or provide a public CA certificate.",
            "For port 443 on Linux use sudo or setcap cap_net_bind_service=+ep on the release binary."
        ]
    }))
}

fn https_generate_self_signed(state: &AppState, params: &HashMap<String,String>) -> Response {
    let mut cfg = state.config.clone();
    cfg.auto_tls = true;
    if let Some(cert) = param(params, &["cert", "tls_cert"]) { cfg.tls_cert = cert.to_string(); }
    if let Some(key) = param(params, &["key", "tls_key"]) { cfg.tls_key = key.to_string(); }
    for key in ["san", "domain", "host", "hostname", "ip"] {
        if let Some(value) = param(params, &[key]) {
            for san in value.split(',').map(str::trim).filter(|s| !s.is_empty()) {
                cfg.tls_subject_alt_names.push(san.to_string());
            }
        }
    }
    match https_native::generate_local_certificate(&cfg) {
        Ok(()) => ok(serde_json::json!({
            "ok": true,
            "generated": true,
            "cert": cfg.tls_cert,
            "key": cfg.tls_key,
            "status": https_native::status(&cfg),
            "note": "Local self-signed certificate generated. Restart with --https --cert <cert> --key <key>, or use --auto_tls."
        })),
        Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    }
}

fn file_manager_mounts(state: &AppState) -> Response {
    match file_api::roots_array(&state.config.root_directory) {
        Ok(drives) => ok(serde_json::json!({
            "ok": true,
            "hostFsReadEnabled": file_api::hostfs_read_enabled(),
            "hostFsWriteEnabled": file_api::hostfs_write_enabled(),
            "drives": drives.clone(),
            "roots": drives
        })),
        Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}

fn parse_users_param(users: &str) -> Vec<String> {
    serde_json::from_str::<Vec<String>>(users).unwrap_or_else(|_| users.split(',').map(|s| s.trim().trim_matches('"').to_string()).filter(|s| !s.is_empty()).collect())
}
fn samba_status(state: &AppState, params: &HashMap<String,String>) -> Response {
    let mut cfg = service_load(state, "samba", None);
    if let Some(set) = param(params, &["set", "enable", "enabled"]) {
        cfg.enabled = matches!(set.to_ascii_lowercase().as_str(), "enable" | "enabled" | "true" | "1" | "on");
        let action = if cfg.enabled { "start" } else { "stop" };
        let native = service_adapter::perform_service_action("samba", action);
        let runtime = protocol_services::apply_service_change(state, "samba", cfg.enabled);
        let mut obj = cfg.options.as_object().cloned().unwrap_or_default();
        obj.insert("lastNativeAction".into(), serde_json::json!(native));
        obj.insert("lastRuntimeAction".into(), serde_json::json!(runtime));
        cfg.options = serde_json::Value::Object(obj);
        return match service_save(state, &cfg) { Ok(()) => text("OK"), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e) };
    }
    let native = service_adapter::service_status("samba", cfg.enabled);
    ok(serde_json::json!(cfg.enabled))
}

fn samba_share_json(s: &SambaShare) -> serde_json::Value {
    let browseable = s.enabled;
    serde_json::json!({
        "Name": s.name,
        "Path": s.path,
        "ValidUsers": s.users,
        "ReadOnly": s.readonly,
        "Browseable": browseable,
        "GuestOk": false,
        "Enabled": s.enabled,
        "ID": s.id
    })
}

fn default_samba_shares(state: &AppState) -> Vec<SambaShare> {
    let users: Vec<String> = state.auth.list_users().into_iter().map(|u| u.username).collect();
    vec![SambaShare {
        id: "homes".into(),
        name: "homes".into(),
        path: "user:/".into(),
        users,
        readonly: false,
        enabled: true,
    }]
}

fn samba_shares_or_default(state: &AppState) -> Vec<SambaShare> {
    let shares: Vec<SambaShare> = read_vec(state, "samba_shares.json");
    if shares.is_empty() { default_samba_shares(state) } else { shares }
}

fn samba_list(state: &AppState) -> Response {
    let shares = samba_shares_or_default(state);
    ok(serde_json::json!(shares.iter().map(samba_share_json).collect::<Vec<_>>()))
}

fn current_username_or_first(state: &AppState, headers: &HeaderMap) -> Option<String> {
    state.auth.current_user(headers).map(|u| u.username)
        .or_else(|| state.auth.session_from_headers(headers).map(|s| s.username))
        .or_else(|| state.auth.list_users().into_iter().next().map(|u| u.username))
}

fn samba_user_is_enabled(state: &AppState, username: &str) -> bool {
    read_vec::<SambaUser>(state, "samba_users.json")
        .into_iter()
        .any(|u| u.enabled && u.username == username)
}

fn samba_myshare(state: &AppState, headers: &HeaderMap) -> Response {
    let cfg = service_load(state, "samba", None);
    let native = service_adapter::service_status("samba", cfg.enabled);
    let smbd_enabled = cfg.enabled || native.active;
    let username = current_username_or_first(state, headers).unwrap_or_else(|| "user".to_string());
    let user_enabled = samba_user_is_enabled(state, &username);
    let shares = samba_shares_or_default(state);
    let visible: Vec<serde_json::Value> = shares.iter()
        .filter(|s| s.enabled && (s.users.is_empty() || s.users.iter().any(|u| u == &username) || s.name == "homes"))
        .map(samba_share_json)
        .collect();
    ok(serde_json::json!({
        "SmbdEnabled": smbd_enabled,
        "NativeAvailable": native.native_available,
        "NativeActive": native.active,
        "ServiceManager": native.manager,
        "UserSmbShareEnabled": user_enabled,
        "Username": username,
        "UserSmbShareList": if user_enabled { visible } else { Vec::<serde_json::Value>::new() }
    }))
}

fn samba_add(state: &AppState, params: &HashMap<String,String>) -> Response {
    let mut shares: Vec<SambaShare> = read_vec(state, "samba_shares.json");
    let name = param(params, &["name"]).unwrap_or("Share").to_string();
    shares.retain(|s| s.name != name);
    let share = SambaShare {
        id: Uuid::new_v4().to_string(),
        name,
        path: param(params, &["path"]).unwrap_or(&state.config.root_directory).to_string(),
        users: param(params, &["users"]).map(parse_users_param).unwrap_or_else(|| state.auth.list_users().into_iter().map(|u| u.username).collect()),
        readonly: utils::truthy(param_ref(params, &["readonly"])),
        enabled: true,
    };
    shares.push(share);
    match write_vec(state, "samba_shares.json", &shares) { Ok(()) => text("OK"), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e) }
}

fn samba_remove(state: &AppState, params: &HashMap<String,String>) -> Response {
    let id = param(params, &["id", "name"]).unwrap_or("");
    let mut shares: Vec<SambaShare> = read_vec(state, "samba_shares.json");
    shares.retain(|s| s.id != id && s.name != id);
    match write_vec(state, "samba_shares.json", &shares) { Ok(()) => text("OK"), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e) }
}

fn samba_edit(state: &AppState, params: &HashMap<String,String>) -> Response {
    let id = param(params, &["id", "name"]).unwrap_or("");
    let mut shares: Vec<SambaShare> = read_vec(state, "samba_shares.json");
    for s in &mut shares {
        if s.id == id || s.name == id {
            if let Some(path) = param(params, &["path"]) { s.path = path.into(); }
            if let Some(users) = param(params, &["users"]) { s.users = parse_users_param(users); }
            if param_ref(params, &["readonly"]).is_some() { s.readonly = utils::truthy(param_ref(params, &["readonly"])); }
            if param_ref(params, &["enabled", "browseable"]).is_some() { s.enabled = utils::truthy(param_ref(params, &["enabled", "browseable"])); }
        }
    }
    match write_vec(state, "samba_shares.json", &shares) { Ok(()) => text("OK"), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e) }
}

fn samba_user_list(state: &AppState) -> Response {
    let mut out: Vec<serde_json::Value> = read_vec::<SambaUser>(state, "samba_users.json")
        .into_iter()
        .filter(|u| u.enabled)
        .map(|u| serde_json::json!({"UnixUsername":u.username,"Domain":"WORKGROUP","IsArozOSUser":false}))
        .collect();
    for u in state.auth.list_users() {
        let name = u.username;
        if !out.iter().any(|x| x.get("UnixUsername").and_then(|v| v.as_str()) == Some(name.as_str())) {
            out.push(serde_json::json!({"UnixUsername":name,"Domain":"WORKGROUP","IsArozOSUser":true}));
        }
    }
    ok(serde_json::json!(out))
}

fn samba_user_add(state: &AppState, params: &HashMap<String,String>) -> Response {
    let mut users: Vec<SambaUser> = read_vec(state, "samba_users.json");
    let username = param(params, &["username", "user"]).unwrap_or("");
    if username.is_empty() { return err(StatusCode::BAD_REQUEST, "missing username"); }
    users.retain(|u| u.username != username);
    users.push(SambaUser { username: username.into(), enabled: true });
    match write_vec(state, "samba_users.json", &users) { Ok(()) => text("OK"), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e) }
}

fn samba_user_remove(state: &AppState, params: &HashMap<String,String>) -> Response {
    let username = param(params, &["username", "user"]).unwrap_or("");
    let mut users: Vec<SambaUser> = read_vec(state, "samba_users.json");
    users.retain(|u| u.username != username);
    match write_vec(state, "samba_users.json", &users) { Ok(()) => text("OK"), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e) }
}

fn samba_user_activation(state: &AppState, headers: &HeaderMap, enable: bool) -> Response {
    let username = match current_username_or_first(state, headers) {
        Some(u) => u,
        None => return err(StatusCode::UNAUTHORIZED, "no current user"),
    };
    let mut cfg = service_load(state, "samba", None);
    cfg.enabled = true;
    let _ = service_save(state, &cfg);

    let mut users: Vec<SambaUser> = read_vec(state, "samba_users.json");
    users.retain(|u| u.username != username);
    if enable { users.push(SambaUser { username, enabled: true }); }
    match write_vec(state, "samba_users.json", &users) { Ok(()) => text("OK"), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e) }
}

fn network_interfaces() -> Response {
    let mut interfaces = Vec::new();
    if let Ok(rd) = fs::read_dir("/sys/class/net") {
        for e in rd.flatten() {
            let name = e.file_name().to_string_lossy().to_string();
            let base = e.path();
            let mac = fs::read_to_string(base.join("address")).unwrap_or_default().trim().to_string();
            let rx = fs::read_to_string(base.join("statistics/rx_bytes")).unwrap_or_default().trim().to_string();
            let tx = fs::read_to_string(base.join("statistics/tx_bytes")).unwrap_or_default().trim().to_string();
            interfaces.push(serde_json::json!({
                "Name": name,
                "HardwareAddr": mac,
                "IPv4Addr": "",
                "IPv6Addr": "",
                "RxBytes": rx,
                "TxBytes": tx
            }));
        }
    }
    ok(serde_json::json!(interfaces))
}
fn ping_host(params: &HashMap<String,String>) -> Response { let host = param(params, &["host", "ip", "target"]).unwrap_or("127.0.0.1"); if host.parse::<IpAddr>().is_err() && !host.chars().all(|c| c.is_ascii_alphanumeric() || c=='.' || c=='-') { return err(StatusCode::BAD_REQUEST, "invalid host"); } ok(serde_json::json!({"ok": true, "ping": utils::run_command("ping", &["-c", "1", "-W", "2", host])})) }
fn wifi_scan(state: &AppState) -> Response { let profiles: Vec<WifiProfile> = read_vec(state, "wifi_profiles.json"); ok(serde_json::json!({"ok": true, "networks": profiles, "note": "Wi-Fi scan returns Rust-managed profiles; live RF scan requires platform adapter"})) }
fn wifi_info(state: &AppState) -> Response { let profiles: Vec<WifiProfile> = read_vec(state, "wifi_profiles.json"); ok(serde_json::json!({"ok": true, "profiles": profiles})) }
fn disk_space(state: &AppState) -> Response {
    let root = PathBuf::from(&state.config.root_directory);
    let used = dir_size(&root, 50000);
    let mut rows = Vec::new();
    if let Ok(out) = std::process::Command::new("df").args(["-kP", &state.config.root_directory]).output() {
        let txt = String::from_utf8_lossy(&out.stdout);
        for line in txt.lines().skip(1) {
            let p: Vec<_> = line.split_whitespace().collect();
            if p.len() >= 6 {
                let vol = p[1].parse::<u64>().unwrap_or(0) * 1024;
                let used_b = p[2].parse::<u64>().unwrap_or(0) * 1024;
                let avail = p[3].parse::<u64>().unwrap_or(0) * 1024;
                rows.push(serde_json::json!({"Device":p[0],"MountPoint":p[5],"Volume":vol,"Used":used_b,"Available":avail}));
            }
        }
    }
    if rows.is_empty() { rows.push(serde_json::json!({"Device":"local","MountPoint":root.to_string_lossy(),"Volume":0,"Used":used,"Available":0})); }
    ok(serde_json::json!(rows))
}
fn dir_size(path: &PathBuf, max_entries: usize) -> u64 { let mut total=0; let mut stack=vec![path.clone()]; let mut count=0; while let Some(p)=stack.pop() { if count > max_entries { break; } count+=1; if let Ok(md)=fs::metadata(&p) { if md.is_dir() { if let Ok(rd)=fs::read_dir(&p) { for e in rd.flatten() { stack.push(e.path()); } } } else { total += md.len(); } } } total }
fn large_files(state: &AppState, params: &HashMap<String,String>) -> Response { let limit = param(params, &["limit", "number"]).and_then(|s| s.parse().ok()).unwrap_or(50usize); let mut files = Vec::new(); collect_large(&PathBuf::from(&state.config.root_directory), &mut files, limit * 4); files.sort_by(|a,b| b["Size"].as_u64().cmp(&a["Size"].as_u64())); files.truncate(limit); ok(serde_json::json!(files)) }
fn collect_large(path: &PathBuf, files: &mut Vec<serde_json::Value>, max: usize) { if files.len() > max { return; } if let Ok(rd)=fs::read_dir(path) { for e in rd.flatten() { let p=e.path(); if let Ok(md)=e.metadata() { if md.is_dir() { collect_large(&p, files, max); } else { let filename=p.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string(); files.push(serde_json::json!({"Filename": filename, "Filepath": p.to_string_lossy(), "Size": md.len()})); } } } } }
fn quota_map(state: &AppState) -> std::collections::BTreeMap<String, i64> {
    let mut m: std::collections::BTreeMap<String, i64> = read_value(state, "quota_config.json")
        .as_object()
        .map(|obj| obj.iter().filter_map(|(k,v)| v.as_i64().map(|n| (k.clone(), n))).collect())
        .unwrap_or_default();
    m.entry("administrator".into()).or_insert(-1);
    m.entry("user".into()).or_insert(-1);
    m
}
fn quota_info(state: &AppState) -> Response {
    let used = dir_size(&PathBuf::from(&state.config.root_directory), 50000);
    ok(serde_json::json!({"Used": used, "Total": -1, "ok": true, "quota": {"enabled": true, "root": &state.config.root_directory}}))
}
fn quota_distribution(state: &AppState) -> Response {
    let mut map: std::collections::BTreeMap<String, u64> = std::collections::BTreeMap::new();
    let mut files = Vec::new();
    collect_large(&PathBuf::from(&state.config.root_directory), &mut files, 50000);
    for f in files {
        let path = f.get("Filepath").or_else(|| f.get("path")).and_then(|v| v.as_str()).unwrap_or("");
        let size = f.get("Size").or_else(|| f.get("size")).and_then(|v| v.as_u64()).unwrap_or(0);
        let mime = mime_guess::from_path(path).first_or_octet_stream().essence_str().to_string();
        *map.entry(mime).or_insert(0) += size;
    }
    ok(serde_json::json!(map.into_iter().map(|(mime, size)| serde_json::json!({"Mime": mime, "Size": size})).collect::<Vec<_>>()))
}
fn quota_list(state: &AppState) -> Response { ok(serde_json::json!(quota_map(state))) }
fn quota_set(state: &AppState, params: &HashMap<String,String>) -> Response {
    let group = param(params, &["groupname", "group", "name"]).unwrap_or("user").to_string();
    let quota = param(params, &["quota", "size"]).and_then(|s| s.parse::<i64>().ok()).unwrap_or(-1);
    let mut quotas = quota_map(state);
    quotas.insert(group, quota);
    match write_value(state, "quota_config.json", &serde_json::json!(quotas)) { Ok(()) => text("OK"), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e) }
}
fn lsblk_json() -> serde_json::Value {
    let out = std::process::Command::new("lsblk")
        .args(["-b", "-J", "-O"])
        .output()
        .ok()
        .and_then(|o| serde_json::from_slice::<serde_json::Value>(&o.stdout).ok())
        .unwrap_or_else(|| serde_json::json!({"blockdevices": []}));
    out
}

fn find_mountpoint(dev: &str) -> String {
    if dev.is_empty() { return String::new(); }
    let target = if dev.starts_with("/dev/") { dev.to_string() } else { format!("/dev/{dev}") };
    for line in fs::read_to_string("/proc/mounts").unwrap_or_default().lines() {
        let cols: Vec<_> = line.split_whitespace().collect();
        if cols.len() >= 2 && (cols[0] == target || cols[0] == dev) { return cols[1].to_string(); }
    }
    String::new()
}

fn flatten_lsblk_devices(v: &serde_json::Value, out: &mut Vec<serde_json::Value>) {
    if let Some(arr) = v.get("blockdevices").and_then(|v| v.as_array()) {
        for item in arr { flatten_lsblk_node(item, out); }
    }
}

fn flatten_lsblk_node(item: &serde_json::Value, out: &mut Vec<serde_json::Value>) {
    let name = item.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let path = item.get("path").and_then(|v| v.as_str()).filter(|s| !s.is_empty()).map(str::to_string).unwrap_or_else(|| if name.is_empty() { String::new() } else { format!("/dev/{name}") });
    let size = item.get("size").and_then(|v| v.as_u64()).unwrap_or(0);
    let ro = item.get("ro").and_then(|v| v.as_bool()).or_else(|| item.get("ro").and_then(|v| v.as_u64()).map(|n| n != 0)).unwrap_or(false);
    let tran = item.get("tran").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let model = item.get("model").and_then(|v| v.as_str()).unwrap_or(name).trim().to_string();
    let fs_type = item.get("fstype").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let mountpoint = item.get("mountpoint").and_then(|v| v.as_str()).map(str::to_string).unwrap_or_else(|| find_mountpoint(&path));
    let typ = item.get("type").and_then(|v| v.as_str()).unwrap_or("");
    if !path.is_empty() && matches!(typ, "disk" | "part" | "loop" | "rom" | "raid0" | "raid1" | "raid5" | "raid6") {
        out.push(serde_json::json!({
            "Name": name,
            "DevicePath": path,
            "device": path,
            "Model": if model.is_empty() { name.to_string() } else { model.clone() },
            "name": if model.is_empty() { name.to_string() } else { model.clone() },
            "Size": size,
            "size": size,
            "Filesystem": fs_type,
            "filesystem": fs_type,
            "MountPoint": mountpoint,
            "mountpoint": mountpoint,
            "mounted": !mountpoint.is_empty(),
            "mountable": !ro,
            "openPath": if mountpoint.is_empty() { serde_json::Value::Null } else { serde_json::Value::String(format!("file://{}", mountpoint)) },
            "removable": item.get("rm").and_then(|v| v.as_bool()).or_else(|| item.get("rm").and_then(|v| v.as_u64()).map(|n| n != 0)).unwrap_or(false),
            "hotplug": item.get("hotplug").and_then(|v| v.as_bool()).or_else(|| item.get("hotplug").and_then(|v| v.as_u64()).map(|n| n != 0)).unwrap_or(false),
            "readOnly": ro,
            "ro": ro,
            "bootMedium": mountpoint == "/boot" || mountpoint == "/boot/efi",
            "systemStorage": mountpoint == "/" || mountpoint.starts_with("/home") || mountpoint.starts_with("/var"),
            "transport": tran,
            "Type": typ,
        }));
    }
    if let Some(children) = item.get("children").and_then(|v| v.as_array()) {
        for child in children { flatten_lsblk_node(child, out); }
    }
}

fn disk_device_rows() -> Vec<serde_json::Value> {
    let mut devices = Vec::new();
    flatten_lsblk_devices(&lsblk_json(), &mut devices);
    if devices.is_empty() {
        devices.push(serde_json::json!({
            "Name":"root", "DevicePath":"/", "device":"/", "Model":"Root Filesystem", "name":"Root Filesystem",
            "Size":0, "size":0, "Filesystem":"", "filesystem":"", "MountPoint":"/", "mountpoint":"/",
            "mounted":true, "mountable":false, "openPath":"file:///", "removable":false, "hotplug":false,
            "readOnly":false, "ro":false, "bootMedium":false, "systemStorage":true, "transport":"", "Type":"mount"
        }));
    }
    devices
}

fn disk_devices_list() -> Response { ok(serde_json::json!(disk_device_rows())) }

fn alpnas_drive_list(state: &AppState, headers: &HeaderMap) -> Response {
    let user = state.auth.current_user(headers);
    let groups = user.as_ref().map(|u| vec![u.group.clone()]).unwrap_or_else(|| vec!["administrator".into()]);
    let is_admin = groups.iter().any(|g| matches!(g.to_ascii_lowercase().as_str(), "administrator" | "admin" | "administrators"));
    let devices = disk_device_rows();
    let selected = devices.first().and_then(|d| d.get("device").and_then(|v| v.as_str())).unwrap_or("").to_string();
    ok(serde_json::json!({
        "ok": true,
        "devices": devices,
        "userGroups": groups,
        "isAdmin": is_admin,
        "selectedDevice": selected,
        "runtime": "rust"
    }))
}

fn block_device_model(params: &HashMap<String,String>) -> Response {
    let dev = param(params, &["devName", "dev", "device"]).unwrap_or("");
    for d in disk_device_rows() {
        let path = d.get("DevicePath").and_then(|v| v.as_str()).unwrap_or("");
        let name = d.get("Name").and_then(|v| v.as_str()).unwrap_or("");
        if dev == path || dev == name || path.ends_with(dev) {
            let model = d.get("Model").and_then(|v| v.as_str()).unwrap_or(path);
            let size = d.get("Size").and_then(|v| v.as_u64()).unwrap_or(0);
            let ro = d.get("ro").and_then(|v| v.as_bool()).unwrap_or(false);
            return ok(serde_json::json!([model, size, ro]));
        }
    }
    ok(serde_json::json!([dev, 0, false]))
}
fn mount_points() -> Response { ok(serde_json::json!({"ok": true, "mounts": utils::run_command("findmnt", &["-J"])})) }
fn raid_list() -> Response {
    let mdstat = fs::read_to_string("/proc/mdstat").unwrap_or_default();
    let mut out = Vec::new();
    for line in mdstat.lines() {
        if let Some((name, rest)) = line.split_once(':') {
            let name = name.trim();
            if !name.starts_with("md") { continue; }
            let level = rest.split_whitespace().find(|p| p.starts_with("raid")).unwrap_or("raid");
            out.push(serde_json::json!({
                "DevicePath": format!("/dev/{name}"),
                "RaidLevel": level,
                "State": "active",
                "UUID": name,
                "ActiveDevices": 0,
                "WorkingDevices": 0,
                "FailedDevices": 0,
                "SpareDevices": 0,
                "ArraySize": 0,
                "DeviceInfo": []
            }));
        }
    }
    ok(serde_json::json!(out))
}
fn raid_detail(params: &HashMap<String,String>) -> Response {
    let dev = param(params, &["devName", "dev", "device", "raid"]).unwrap_or("/dev/md0");
    let detail = utils::run_command("mdadm", &["--detail", dev]);
    ok(serde_json::json!({
        "DevicePath": dev,
        "RaidLevel": "raid",
        "State": if detail.get("stdout").and_then(|v| v.as_str()).unwrap_or("").trim().is_empty() { "unknown" } else { "active" },
        "UUID": "",
        "ActiveDevices": 0,
        "WorkingDevices": 0,
        "FailedDevices": 0,
        "SpareDevices": 0,
        "ArraySize": 0,
        "DeviceInfo": [],
        "RawDetail": detail
    }))
}
fn smart_info(_params: &HashMap<String,String>) -> Response {
    let devices: Vec<_> = disk_device_rows().into_iter().filter_map(|d| {
        let path = d.get("DevicePath").and_then(|v| v.as_str()).unwrap_or("").to_string();
        if path.is_empty() { return None; }
        let model = d.get("Model").and_then(|v| v.as_str()).unwrap_or(&path).to_string();
        let size = d.get("Size").and_then(|v| v.as_u64()).unwrap_or(0);
        Some(serde_json::json!({
            "device": {"name": path},
            "smart": {
                "model_name": model,
                "healthy": "Unknown",
                "user_capacity": {"bytes": size},
                "ata_smart_attributes": {"table": []}
            }
        }))
    }).collect();
    ok(serde_json::json!({"devices": devices, "healthy": "Unknown"}))
}
fn usage_info() -> Response { ok(serde_json::json!({"ok": true, "loadavg": fs::read_to_string("/proc/loadavg").unwrap_or_default(), "meminfo": fs::read_to_string("/proc/meminfo").unwrap_or_default()})) }
fn usb_ports() -> Response {
    let detail = utils::run_command("lsusb", &[]);
    let stdout = detail.get("stdout").and_then(|v| v.as_str()).unwrap_or("");
    let ports: Vec<String> = stdout.lines().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
    if ports.is_empty() { ok(serde_json::json!(["No USB device information available"])) } else { ok(serde_json::json!(ports)) }
}
fn current_time() -> Response {
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
    ok(serde_json::json!({"ok": true, "unix": secs, "timestamp": secs, "time": secs}))
}

fn ssdp_xml(state: &AppState) -> Response { let body = format!("<?xml version=\"1.0\"?><root><device><deviceType>urn:schemas-upnp-org:device:Basic:1</deviceType><friendlyName>{}</friendlyName><manufacturer>AlpNAS Project</manufacturer><modelName>ArozOS Rust</modelName><UDN>uuid:{}</UDN></device></root>", &state.config.host_name, if state.config.system_uuid.is_empty() { "arozos-rs" } else { &state.config.system_uuid }); (StatusCode::OK, [("content-type", "application/xml")], body).into_response() }
async fn media_endpoint(state: &AppState, path: &str, params: &HashMap<String,String>) -> Response {
    let inferred;
    let target = if let Some(t) = param_ref(params, &["file", "path", "src", "vpath"]) {
        Some(t)
    } else {
        let suffix = path
            .strip_prefix("/media/download/")
            .or_else(|| path.strip_prefix("/media/getMime/"))
            .or_else(|| path.strip_prefix("/media/transcode/"))
            .or_else(|| path.strip_prefix("/media/"))
            .unwrap_or("");
        inferred = format!("user:/{}", suffix);
        Some(&inferred)
    };
    if path.starts_with("/media/getMime/") || path == "/media/getMime/" {
        let mime = target.map(|p| mime_guess::from_path(p).first_or_octet_stream().essence_str().to_string()).unwrap_or_else(|| "application/octet-stream".into());
        return text(&mime);
    }
    let real = match file_api::resolve(&state.config.root_directory, target) { Ok(p) => p, Err(e) => return err(StatusCode::BAD_REQUEST, e.to_string()) };
    match tokio::fs::read(&real).await {
        Ok(bytes) => {
            let mime = mime_guess::from_path(&real).first_or_octet_stream().to_string();
            (StatusCode::OK, [("content-type", mime)], bytes).into_response()
        },
        Err(e) => err(StatusCode::NOT_FOUND, e.to_string()),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PermissionGroup { name: String, read: bool, write: bool, admin: bool }
fn log_list() -> Response {
    let mut logs = Vec::new();
    for file in ["/var/log/syslog", "/var/log/messages", "/var/log/dmesg"] {
        if fs::metadata(file).is_ok() { logs.push(file); }
    }
    ok(serde_json::json!({"ok": true, "logs": logs}))
}
fn log_read(params: &HashMap<String,String>) -> Response {
    let requested = param(params, &["file", "log"]).unwrap_or("/var/log/syslog");
    if !requested.starts_with("/var/log/") { return err(StatusCode::FORBIDDEN, "only /var/log files may be read"); }
    match fs::read_to_string(requested) {
        Ok(s) => { let tail = s.lines().rev().take(500).collect::<Vec<_>>().into_iter().rev().collect::<Vec<_>>().join("\n"); ok(serde_json::json!({"ok": true, "file": requested, "content": tail})) },
        Err(e) => err(StatusCode::NOT_FOUND, e.to_string()),
    }
}
fn permission_groups(state: &AppState) -> Vec<PermissionGroup> {
    let mut groups: Vec<PermissionGroup> = read_vec(state, "permission_groups.json");
    if groups.iter().all(|g| g.name != "administrator") { groups.push(PermissionGroup { name:"administrator".into(), read:true, write:true, admin:true }); }
    if groups.iter().all(|g| g.name != "user") { groups.push(PermissionGroup { name:"user".into(), read:true, write:true, admin:false }); }
    groups
}
fn permission_list(state: &AppState) -> Response {
    if state.auth.is_first_run() { return ok(serde_json::json!(["administrator"])); }
    let groups = permission_groups(state);
    if false { return ok(serde_json::json!(groups)); }
    // Go compatibility: without showper the endpoint returns a plain array of group names;
    // with showper it returns { group: [AccessibleModules, IsAdmin, DefaultStorageQuota] }.
    let quotas = quota_map(state);
    ok(serde_json::json!(groups.iter().map(|g| g.name.clone()).collect::<Vec<_>>()))
}
fn permission_list_with_mode(state: &AppState, params: &HashMap<String,String>) -> Response {
    let groups = permission_groups(state);
    if utils::truthy(param_ref(params, &["showper"])) {
        let quotas = quota_map(state);
        let obj: serde_json::Map<String, serde_json::Value> = groups.into_iter().map(|g| {
            let modules = if g.admin { vec!["*".to_string()] } else { app_api::builtin_modules().into_iter().map(|m| m.name).collect::<Vec<_>>() };
            let quota = quotas.get(&g.name).cloned().unwrap_or(-1);
            (g.name, serde_json::json!([modules, g.admin, quota]))
        }).collect();
        ok(serde_json::Value::Object(obj))
    } else {
        ok(serde_json::json!(groups.iter().map(|g| g.name.clone()).collect::<Vec<_>>()))
    }
}
fn permission_new(state: &AppState, params: &HashMap<String,String>) -> Response {
    let mut groups = permission_groups(state);
    let name = param(params, &["name", "group", "groupname"]).unwrap_or("user");
    groups.retain(|g| g.name != name);
    let admin = utils::truthy(param_ref(params, &["admin", "isAdmin", "setAsAdmin"]));
    let group = PermissionGroup { name:name.into(), read:!matches!(param(params, &["read"]), Some("false")), write:!matches!(param(params, &["write"]), Some("false")), admin };
    groups.push(group.clone());
    let mut quotas = quota_map(state);
    if let Some(q) = param(params, &["quota", "defaultStorageQuota"]).and_then(|s| s.parse::<i64>().ok()) { quotas.insert(name.to_string(), q); let _ = write_value(state, "quota_config.json", &serde_json::json!(quotas)); }
    match write_vec(state, "permission_groups.json", &groups) { Ok(()) => text("OK"), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e) }
}
fn permission_edit(state: &AppState, params: &HashMap<String,String>) -> Response {
    if utils::truthy(param_ref(params, &["list"])) {
        let name = param(params, &["groupname", "name", "group"]).unwrap_or("user");
        let groups = permission_groups(state);
        if let Some(g) = groups.into_iter().find(|g| g.name == name) {
            let modules = if g.admin { vec!["*".to_string()] } else { app_api::builtin_modules().into_iter().map(|m| m.name).collect::<Vec<_>>() };
            let quota = quota_map(state).get(&g.name).cloned().unwrap_or(-1);
            return ok(serde_json::json!({"Name": g.name, "AccessibleModules": modules, "IsAdmin": g.admin, "DefaultInterfaceModule": "desktop", "DefaultStorageQuota": quota, "Quota": quota}));
        }
        return err(StatusCode::NOT_FOUND, "group not found");
    }
    permission_new(state, params)
}
fn permission_delete(state: &AppState, params: &HashMap<String,String>) -> Response {
    let name = param(params, &["name", "group", "groupname"]).unwrap_or("");
    let mut groups = permission_groups(state);
    let old = groups.len(); groups.retain(|g| g.name != name || g.name == "administrator");
    match write_vec(state, "permission_groups.json", &groups) { Ok(()) => text("OK"), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e) }
}



async fn file_thumbnail(state: &AppState, params: &HashMap<String,String>) -> Response {
    let raw = param_ref(params, &["vpath", "file", "path", "src"]);
    let Some(raw) = raw else { return ok(serde_json::json!({"ok": true, "cached": false, "thumbnail": null})); };
    let real = match file_api::resolve(&state.config.root_directory, Some(raw)) { Ok(p) => p, Err(e) => return err(StatusCode::BAD_REQUEST, e.to_string()) };
    match tokio::fs::read(&real).await {
        Ok(bytes) => {
            let mime = mime_guess::from_path(&real).first_or_octet_stream().to_string();
            (StatusCode::OK, [("content-type", mime), ("cache-control", "private, max-age=60".to_string())], bytes).into_response()
        },
        Err(e) => err(StatusCode::NOT_FOUND, e.to_string()),
    }
}

fn write_code_php(state: &AppState, params: &HashMap<String,String>) -> Response {
    let target = param(params, &["filename", "filepath", "file", "path"]).unwrap_or("");
    if target.is_empty() { return err(StatusCode::BAD_REQUEST, "missing filename"); }
    let content = param(params, &["content", "data", "text"]).unwrap_or("");
    let target_owned = target.to_string();
    match file_api::resolve(&state.config.root_directory, Some(&target_owned)) {
        Ok(p) => { if let Some(parent) = p.parent() { let _ = fs::create_dir_all(parent); } match fs::write(&p, content) { Ok(()) => text("OK"), Err(e) => err(StatusCode::BAD_REQUEST, e.to_string()) } },
        Err(e) => err(StatusCode::BAD_REQUEST, e.to_string()),
    }
}

fn backup_root(state: &AppState) -> PathBuf { utils::data_file(&state.config.system_root, "backups") }
fn backup_list(state: &AppState) -> Response {
    let root = backup_root(state); let _ = fs::create_dir_all(&root);
    let mut out = Vec::new();
    if let Ok(rd) = fs::read_dir(root) { for e in rd.flatten() { if e.path().is_dir() { out.push(serde_json::json!({"Name": e.file_name().to_string_lossy(), "Path": e.path().to_string_lossy(), "Created": e.metadata().ok().and_then(|m| m.modified().ok()).and_then(|t| t.duration_since(UNIX_EPOCH).ok()).map(|d| d.as_secs()).unwrap_or(0)})); } } }
    ok(serde_json::json!(out))
}
fn backup_summary(state: &AppState) -> Response {
    let root = backup_root(state); let count = fs::read_dir(&root).map(|rd| rd.flatten().filter(|e| e.path().is_dir()).count()).unwrap_or(0);
    ok(serde_json::json!({"ok": true, "count": count, "root": root.to_string_lossy(), "native": "rust"}))
}
fn backup_restore(_state: &AppState, _params: &HashMap<String,String>) -> Response {
    err(StatusCode::FORBIDDEN, "backup restore is available through the Rust planner but destructive overwrite requires an explicit privileged restore adapter")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WifiProfile { ssid: String, enabled: bool, last_action_unix: u64 }
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SubserviceEntry { name: String, command: String, running: bool, pid: Option<u32>, last_action_unix: u64 }
#[derive(Debug, Clone, Serialize, Deserialize)]
struct IotDeviceEntry { id: String, nickname: String, status: String, last_command: Option<String>, updated_unix: u64 }
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ResetKey { key: String, username: String, expires_unix: Option<u64>, used: bool }
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ModuleManifest { name: String, version: String, source: String, installed_unix: u64 }

fn ldap_login(state: &AppState, params: &HashMap<String,String>) -> Response {
    let cfg = read_value(state, "ldap_config.json");
    if cfg.get("localFallback").and_then(|v| v.as_bool()).unwrap_or(true) {
        let username = param(params, &["username", "user", "u"]).unwrap_or("");
        let password = param(params, &["password", "pw", "p"]).unwrap_or("");
        if !username.is_empty() && state.auth.verify(username, password) {
            let token = state.auth.create_session(username.to_string());
            return (StatusCode::OK, [(header::SET_COOKIE, format!("arozos_session={}; Path=/; HttpOnly; SameSite=Lax", token))], Json(serde_json::json!({"ok": true, "username": username, "provider": "local-fallback"}))).into_response();
        }
    }
    let enabled = cfg.get("enabled").and_then(|v| v.as_bool()).unwrap_or(false);
    if !enabled { return err(StatusCode::UNAUTHORIZED, "LDAP is disabled; local fallback credentials were not accepted"); }
    ok(serde_json::json!({"ok": false, "provider": "ldap", "error": "network LDAP bind is configured but not executed without an ldap3 backend feature", "config": cfg}))
}

fn ldap_password_update(state: &AppState, params: &HashMap<String,String>) -> Response {
    let username = param(params, &["username", "user", "u"]).unwrap_or("");
    let password = param(params, &["password", "newPassword", "newpw", "pw"]).unwrap_or("");
    if username.is_empty() || password.len() < 4 { return err(StatusCode::BAD_REQUEST, "missing username or password too short"); }
    let cfg = read_value(state, "ldap_config.json");
    if cfg.get("allowLocalPasswordUpdate").and_then(|v| v.as_bool()).unwrap_or(true) {
        return match state.auth.upsert_user(username, Some(password), false, None, None) {
            Ok(user) => ok(serde_json::json!({"ok": true, "user": user, "provider": "local-fallback"})),
            Err(e) => err(StatusCode::BAD_REQUEST, e),
        };
    }
    err(StatusCode::FORBIDDEN, "LDAP password changes are delegated to the LDAP provider")
}

fn oauth_flow(state: &AppState, params: &HashMap<String,String>) -> Response {
    let cfg = read_value(state, "oauth_config.json");
    let provider = param(params, &["provider", "type"]).unwrap_or("default");
    if let Some(username) = param(params, &["username", "user", "mockUser"]) {
        if cfg.get("allowLocalMockLogin").and_then(|v| v.as_bool()).unwrap_or(false) {
            let _ = state.auth.upsert_user(username, None, false, Some("oauth"), None);
            let token = state.auth.create_session(username.to_string());
            return (StatusCode::OK, [(header::SET_COOKIE, format!("arozos_session={}; Path=/; HttpOnly; SameSite=Lax", token))], Json(serde_json::json!({"ok": true, "provider": provider, "username": username, "mode": "local-mock"}))).into_response();
        }
    }
    ok(serde_json::json!({"ok": true, "provider": provider, "authorize_url": cfg.get(provider).and_then(|v| v.get("authorize_url")).cloned().unwrap_or(serde_json::Value::Null), "state": Uuid::new_v4().to_string(), "note": "OAuth configuration is ported; external provider token exchange must be enabled by adding an OAuth backend feature"}))
}

fn zip_handler(state: &AppState, params: &HashMap<String,String>) -> Response {
    let mode = param(params, &["mode", "opr", "op", "action"]).unwrap_or("list");
    match mode {
        "extract" | "unzip" => zip_extract(state, params),
        "archive" | "zip" | "compress" => zip_create(state, params),
        _ => zip_list(state, params),
    }
}

fn zip_list(state: &AppState, params: &HashMap<String,String>) -> Response {
    let archive = match file_api::resolve(&state.config.root_directory, param_ref(params, &["zip", "archive", "src", "file", "path"])) { Ok(p) => p, Err(e) => return err(StatusCode::BAD_REQUEST, e.to_string()) };
    let file = match fs::File::open(&archive) { Ok(f) => f, Err(e) => return err(StatusCode::NOT_FOUND, e.to_string()) };
    let mut zip = match zip::ZipArchive::new(file) { Ok(z) => z, Err(e) => return err(StatusCode::BAD_REQUEST, e.to_string()) };
    let mut entries = Vec::new();
    for i in 0..zip.len() {
        if let Ok(f) = zip.by_index(i) { entries.push(serde_json::json!({"name": f.name(), "size": f.size(), "compressed_size": f.compressed_size(), "is_dir": f.is_dir()})); }
    }
    ok(serde_json::json!({"ok": true, "archive": archive.to_string_lossy(), "entries": entries}))
}

fn zip_extract(state: &AppState, params: &HashMap<String,String>) -> Response {
    let archive = match file_api::resolve(&state.config.root_directory, param_ref(params, &["zip", "archive", "src", "file", "path"])) { Ok(p) => p, Err(e) => return err(StatusCode::BAD_REQUEST, e.to_string()) };
    let dest = match file_api::resolve(&state.config.root_directory, param_ref(params, &["dest", "target", "to"])) { Ok(p) => p, Err(e) => return err(StatusCode::BAD_REQUEST, e.to_string()) };
    if let Err(e) = fs::create_dir_all(&dest) { return err(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()); }
    let file = match fs::File::open(&archive) { Ok(f) => f, Err(e) => return err(StatusCode::NOT_FOUND, e.to_string()) };
    let mut zip = match zip::ZipArchive::new(file) { Ok(z) => z, Err(e) => return err(StatusCode::BAD_REQUEST, e.to_string()) };
    let mut extracted = Vec::new();
    for i in 0..zip.len() {
        let mut f = match zip.by_index(i) { Ok(f) => f, Err(e) => return err(StatusCode::BAD_REQUEST, e.to_string()) };
        let safe_name = utils::sanitize_relative_path(f.name());
        if safe_name.is_empty() { continue; }
        let out = dest.join(&safe_name);
        if f.is_dir() { if let Err(e)=fs::create_dir_all(&out) { return err(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()); } } else {
            if let Some(parent)=out.parent() { if let Err(e)=fs::create_dir_all(parent) { return err(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()); } }
            let mut outfile = match fs::File::create(&out) { Ok(f) => f, Err(e) => return err(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()) };
            if let Err(e)=std::io::copy(&mut f, &mut outfile) { return err(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()); }
        }
        extracted.push(format!("user:/{}", safe_name));
    }
    ok(serde_json::json!({"ok": true, "archive": archive.to_string_lossy(), "dest": dest.to_string_lossy(), "extracted": extracted}))
}

fn zip_create(state: &AppState, params: &HashMap<String,String>) -> Response {
    let src = match file_api::resolve(&state.config.root_directory, param_ref(params, &["src", "path", "file", "folder"])) { Ok(p) => p, Err(e) => return err(StatusCode::BAD_REQUEST, e.to_string()) };
    let dest = match file_api::resolve(&state.config.root_directory, param_ref(params, &["dest", "zip", "archive", "target"])) { Ok(p) => p, Err(e) => return err(StatusCode::BAD_REQUEST, e.to_string()) };
    if let Some(parent)=dest.parent() { if let Err(e)=fs::create_dir_all(parent) { return err(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()); } }
    let file = match fs::File::create(&dest) { Ok(f) => f, Err(e) => return err(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()) };
    let mut writer = zip::ZipWriter::new(file);
    let options = zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    let base = if src.is_dir() { src.clone() } else { src.parent().unwrap_or(Path::new("")).to_path_buf() };
    if let Err(e)=zip_add_path(&mut writer, &src, &base, options) { return err(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()); }
    if let Err(e)=writer.finish() { return err(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()); }
    ok(serde_json::json!({"ok": true, "archive": dest.to_string_lossy()}))
}

fn zip_add_path(writer: &mut zip::ZipWriter<fs::File>, path: &Path, base: &Path, options: zip::write::SimpleFileOptions) -> std::io::Result<()> {
    let name = path.strip_prefix(base).unwrap_or(path).to_string_lossy().replace('\\', "/");
    if path.is_dir() {
        if !name.is_empty() { writer.add_directory(format!("{}/", name.trim_end_matches('/')), options)?; }
        for entry in fs::read_dir(path)? { zip_add_path(writer, &entry?.path(), base, options)?; }
    } else {
        writer.start_file(name, options)?;
        let mut f = fs::File::open(path)?;
        std::io::copy(&mut f, writer)?;
    }
    Ok(())
}

fn module_install_zip(state: &AppState, params: &HashMap<String,String>) -> Response {
    let archive = match file_api::resolve(&state.config.root_directory, param_ref(params, &["zip", "archive", "src", "file", "path"])) { Ok(p) => p, Err(e) => return err(StatusCode::BAD_REQUEST, e.to_string()) };
    let module_name = param(params, &["name", "module"]).map(utils::sanitize_relative_path).unwrap_or_else(|| archive.file_stem().map(|s| utils::sanitize_relative_path(&s.to_string_lossy())).unwrap_or_else(|| Uuid::new_v4().to_string()));
    if module_name.is_empty() { return err(StatusCode::BAD_REQUEST, "invalid module name"); }
    let dest = PathBuf::from(&state.config.system_root).join("modules").join(&module_name);
    let fake_params = HashMap::from([
        ("archive".to_string(), format!("user:/{}", archive.strip_prefix(&state.config.root_directory).unwrap_or(&archive).to_string_lossy())),
        ("dest".to_string(), format!("user:/../system/modules/{}", module_name)),
    ]);
    // Extract manually to system module directory because modules live outside the user file root.
    if let Err(e)=fs::create_dir_all(&dest) { return err(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()); }
    let file = match fs::File::open(&archive) { Ok(f) => f, Err(e) => return err(StatusCode::NOT_FOUND, e.to_string()) };
    let mut zip = match zip::ZipArchive::new(file) { Ok(z) => z, Err(e) => return err(StatusCode::BAD_REQUEST, e.to_string()) };
    for i in 0..zip.len() {
        let mut f = match zip.by_index(i) { Ok(f) => f, Err(e) => return err(StatusCode::BAD_REQUEST, e.to_string()) };
        let safe_name = utils::sanitize_relative_path(f.name());
        if safe_name.is_empty() { continue; }
        let out = dest.join(&safe_name);
        if f.is_dir() { if let Err(e)=fs::create_dir_all(&out) { return err(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()); } } else {
            if let Some(parent)=out.parent() { if let Err(e)=fs::create_dir_all(parent) { return err(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()); } }
            let mut outfile = match fs::File::create(&out) { Ok(f) => f, Err(e) => return err(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()) };
            if let Err(e)=std::io::copy(&mut f, &mut outfile) { return err(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()); }
        }
    }
    let mut modules: Vec<ModuleManifest> = read_vec(state, "installed_modules.json");
    modules.retain(|m| m.name != module_name);
    modules.push(ModuleManifest { name: module_name.clone(), version: param(params, &["version"]).unwrap_or("unknown").into(), source: archive.to_string_lossy().into(), installed_unix: now_unix() });
    match write_vec(state, "installed_modules.json", &modules) { Ok(()) => ok(serde_json::json!({"ok": true, "module": module_name, "path": dest.to_string_lossy(), "_extract_params": fake_params})), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e) }
}

fn wifi_mutation(state: &AppState, path: &str, params: &HashMap<String,String>) -> Response {
    let mut profiles: Vec<WifiProfile> = read_vec(state, "wifi_profiles.json");
    match path {
        "/system/network/connectWifi" => {
            let ssid = param(params, &["ssid", "network", "name"]).unwrap_or("");
            if ssid.is_empty() { return err(StatusCode::BAD_REQUEST, "missing ssid"); }
            profiles.retain(|p| p.ssid != ssid);
            profiles.push(WifiProfile { ssid: ssid.into(), enabled: true, last_action_unix: now_unix() });
        },
        "/system/network/removeWifi" => {
            let ssid = param(params, &["ssid", "network", "name"]).unwrap_or("");
            profiles.retain(|p| p.ssid != ssid);
        },
        "/system/network/power" => {
            let enabled = flag(params);
            for p in &mut profiles { p.enabled = enabled; p.last_action_unix = now_unix(); }
        },
        _ => {},
    }
    match write_vec(state, "wifi_profiles.json", &profiles) { Ok(()) => ok(serde_json::json!({"ok": true, "profiles": profiles, "note": "Wi-Fi state is managed by the Rust config backend; OS association requires a platform adapter such as nmcli/iwd"})), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e) }
}

fn disk_privileged_operation(state: &AppState, path: &str, params: &HashMap<String,String>) -> Response {
    let plan = disk_command_plan(path, params);
    if !state.config.allow_hardware_management || std::env::var("AROZOS_RS_ENABLE_DESTRUCTIVE_DISK_OPS").ok().as_deref() != Some("1") {
        return err(StatusCode::FORBIDDEN, format!("privileged disk operation is ported as a gated command plan but not executed: {}", plan));
    }
    execute_shell_plan("disk", &plan)
}

fn disk_command_plan(path: &str, params: &HashMap<String,String>) -> String {
    let dev = param(params, &["dev", "device", "target"]).unwrap_or("");
    let mount = param(params, &["mount", "mountpoint", "path"]).unwrap_or("/mnt/arozos");
    match path {
        "/system/disk/alpnas/mount" | "/system/disk/diskmg/mount" => format!("mount {} {}", shell_escape(dev), shell_escape(mount)),
        "/system/disk/alpnas/unmount" => format!("umount {}", shell_escape(mount)),
        "/system/disk/diskmg/format" => format!("mkfs.ext4 {}", shell_escape(dev)),
        _ => "unknown disk operation".into(),
    }
}

fn raid_privileged_operation(state: &AppState, path: &str, params: &HashMap<String,String>) -> Response {
    let plan = raid_command_plan(path, params);
    if !state.config.allow_hardware_management || std::env::var("AROZOS_RS_ENABLE_DESTRUCTIVE_DISK_OPS").ok().as_deref() != Some("1") {
        return err(StatusCode::FORBIDDEN, format!("privileged RAID operation is ported as a gated command plan but not executed: {}", plan));
    }
    execute_shell_plan("raid", &plan)
}

fn raid_command_plan(path: &str, params: &HashMap<String,String>) -> String {
    let dev = param(params, &["dev", "device", "raid"]).unwrap_or("/dev/md0");
    let members = param(params, &["members", "disks"]).unwrap_or("");
    match path {
        "/system/disk/raid/new" => format!("mdadm --create {} --level={} --raid-devices={} {}", shell_escape(dev), shell_escape(param(params, &["level"]).unwrap_or("1")), shell_escape(param(params, &["count"]).unwrap_or("2")), members),
        "/system/disk/raid/remove" => format!("mdadm --stop {}", shell_escape(dev)),
        "/system/disk/raid/addMemeber" => format!("mdadm {} --add {}", shell_escape(dev), shell_escape(param(params, &["member", "disk"]).unwrap_or(""))),
        "/system/disk/raid/removeMemeber" => format!("mdadm {} --remove {}", shell_escape(dev), shell_escape(param(params, &["member", "disk"]).unwrap_or(""))),
        "/system/disk/raid/grow" => format!("mdadm --grow {} --raid-devices={}", shell_escape(dev), shell_escape(param(params, &["count"]).unwrap_or(""))),
        "/system/disk/raid/format" => format!("mkfs.ext4 {}", shell_escape(dev)),
        "/system/disk/raid/assemble" => format!("mdadm --assemble {} {}", shell_escape(dev), members),
        _ => "unknown raid operation".into(),
    }
}

fn execute_shell_plan(kind: &str, plan: &str) -> Response {
    let output = std::process::Command::new("sh").arg("-c").arg(plan).output();
    match output {
        Ok(out) => ok(serde_json::json!({"ok": out.status.success(), "kind": kind, "status": out.status.code(), "stdout": String::from_utf8_lossy(&out.stdout), "stderr": String::from_utf8_lossy(&out.stderr)})),
        Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}

fn shell_escape(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

fn subservice_mutation(state: &AppState, path: &str, params: &HashMap<String,String>) -> Response {
    let mut services: Vec<SubserviceEntry> = read_vec(state, "subservices.json");
    let name = param(params, &["name", "service", "id"]).unwrap_or("");
    if name.is_empty() { return err(StatusCode::BAD_REQUEST, "missing service name"); }
    match path {
        "/system/subservice/start" => {
            let cmd = param(params, &["cmd", "command"]).unwrap_or("");
            services.retain(|s| s.name != name);
            services.push(SubserviceEntry { name: name.into(), command: cmd.into(), running: true, pid: None, last_action_unix: now_unix() });
        },
        "/system/subservice/kill" => for s in &mut services { if s.name == name { s.running = false; s.last_action_unix = now_unix(); } },
        _ => {},
    }
    match write_vec(state, "subservices.json", &services) { Ok(()) => ok(serde_json::json!({"ok": true, "services": services})), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e) }
}


fn subservice_list(state: &AppState) -> Response {
    let services: Vec<SubserviceEntry> = read_vec(state, "subservices.json");
    ok(serde_json::json!({"ok": true, "services": services}))
}

fn iot_list(state: &AppState) -> Response {
    let devices: Vec<IotDeviceEntry> = read_vec(state, "iot_devices.json");
    ok(serde_json::json!({"ok": true, "devices": devices, "scanners": []}))
}

fn update_mutation(state: &AppState, path: &str, params: &HashMap<String,String>) -> Response {
    let mut cfg = read_value(state, "update_state.json");
    if !cfg.is_object() { cfg = serde_json::json!({}); }
    let obj = cfg.as_object_mut().unwrap();
    match path {
        "/system/update/download" => { obj.insert("pending".into(), serde_json::Value::Bool(true)); obj.insert("source".into(), serde_json::Value::String(param(params, &["url", "source"]).unwrap_or("manual").into())); obj.insert("downloaded_unix".into(), serde_json::Value::Number(now_unix().into())); },
        "/system/update/restart" => { obj.insert("restart_requested".into(), serde_json::Value::Bool(true)); obj.insert("restart_requested_unix".into(), serde_json::Value::Number(now_unix().into())); },
        _ => {},
    }
    match write_value(state, "update_state.json", &cfg) { Ok(()) => ok(serde_json::json!({"ok": true, "update": cfg, "note": "self-replacement/restart is recorded; process replacement is intentionally delegated to service manager"})), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e) }
}

fn iot_mutation(state: &AppState, path: &str, params: &HashMap<String,String>) -> Response {
    let mut devices: Vec<IotDeviceEntry> = read_vec(state, "iot_devices.json");
    let id = param(params, &["id", "device", "uuid"]).unwrap_or("default");
    let mut found = false;
    for d in &mut devices {
        if d.id == id {
            found = true;
            if path.ends_with("nickname") { d.nickname = param(params, &["nickname", "name"]).unwrap_or(&d.nickname).into(); }
            if path.ends_with("execute") { d.last_command = param(params, &["cmd", "command", "action"]).map(str::to_string); d.status = "command-recorded".into(); }
            if path.ends_with("icon") { d.status = "icon-requested".into(); }
            d.updated_unix = now_unix();
        }
    }
    if !found { devices.push(IotDeviceEntry { id: id.into(), nickname: param(params, &["nickname", "name"]).unwrap_or(id).into(), status: if path.ends_with("execute") { "command-recorded" } else { "known" }.into(), last_command: param(params, &["cmd", "command", "action"]).map(str::to_string), updated_unix: now_unix() }); }
    match write_vec(state, "iot_devices.json", &devices) { Ok(()) => ok(serde_json::json!({"ok": true, "devices": devices, "note": "device-specific drivers must consume recorded commands"})), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e) }
}

fn agi_exec(state: &AppState, params: &HashMap<String,String>) -> Response {
    let allow = std::env::var("AROZOS_RS_ENABLE_AGI_EXEC").ok().as_deref() == Some("1");
    let script = param(params, &["script", "file", "path"]).unwrap_or("");
    if script.is_empty() { return err(StatusCode::BAD_REQUEST, "missing AGI script path"); }
    let real = match file_api::resolve(&state.config.root_directory, Some(&script.to_string())) { Ok(p) => p, Err(e) => return err(StatusCode::BAD_REQUEST, e.to_string()) };
    if !allow { return err(StatusCode::FORBIDDEN, format!("AGI execution is ported but gated; set AROZOS_RS_ENABLE_AGI_EXEC=1 to execute {}", real.to_string_lossy())); }
    let output = std::process::Command::new(&real).output();
    match output { Ok(out) => ok(serde_json::json!({"ok": out.status.success(), "status": out.status.code(), "stdout": String::from_utf8_lossy(&out.stdout), "stderr": String::from_utf8_lossy(&out.stderr)})), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()) }
}

fn wake_on_lan(params: &HashMap<String,String>) -> Response {
    let mac = param(params, &["mac", "addr", "address"]).unwrap_or("");
    let bytes = match parse_mac(mac) { Some(b) => b, None => return err(StatusCode::BAD_REQUEST, "invalid MAC address") };
    let mut packet = [0xffu8; 102];
    for i in 0..16 { packet[6 + i*6..12 + i*6].copy_from_slice(&bytes); }
    let target = param(params, &["broadcast", "host"]).unwrap_or("255.255.255.255:9");
    let socket = match UdpSocket::bind("0.0.0.0:0") { Ok(s) => s, Err(e) => return err(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()) };
    let _ = socket.set_broadcast(true);
    match socket.send_to(&packet, target) { Ok(sent) => ok(serde_json::json!({"ok": true, "sent": sent, "target": target})), Err(e) => err(StatusCode::BAD_GATEWAY, e.to_string()) }
}

fn parse_mac(mac: &str) -> Option<[u8;6]> {
    let clean = mac.replace([':', '-'], "");
    if clean.len() != 12 { return None; }
    let mut out = [0u8;6];
    for i in 0..6 { out[i] = u8::from_str_radix(&clean[i*2..i*2+2], 16).ok()?; }
    Some(out)
}

fn wallpaper(state: &AppState) -> Response {
    let candidates = [
        PathBuf::from(&state.config.web_root).join("img/public/auth_bg.jpg"),
        PathBuf::from(&state.config.web_root).join("img/desktop/bg/init.jpg"),
        PathBuf::from(&state.config.web_root).join("SystemAO/info/img/banner.jpg"),
        PathBuf::from(&state.config.system_root).join("web/SystemAO/d/img/wallpaper.jpg"),
        PathBuf::from(&state.config.system_root).join("web/SystemAO/wallpaper.jpg"),
        PathBuf::from(&state.config.system_root).join("resources/web/SystemAO/d/img/wallpaper.jpg"),
    ];
    for p in candidates { if let Ok(bytes)=fs::read(&p) { return (StatusCode::OK, [("content-type", "image/jpeg")], bytes).into_response(); } }
    err(StatusCode::NOT_FOUND, "wallpaper asset not found")
}

fn power_operation(state: &AppState, path: &str) -> Response {
    if !state.config.allow_power_management || std::env::var("AROZOS_RS_ENABLE_POWER_OPS").ok().as_deref() != Some("1") {
        return err(StatusCode::FORBIDDEN, "power operation is ported but gated; enable allow_power_management and AROZOS_RS_ENABLE_POWER_OPS=1");
    }
    let cmd = if path.ends_with("restart") { "reboot" } else { "poweroff" };
    execute_shell_plan("power", cmd)
}

fn reset_validate(state: &AppState, params: &HashMap<String,String>) -> Response {
    let key = param(params, &["key", "token", "resetKey"]).unwrap_or("");
    let keys: Vec<ResetKey> = read_vec(state, "reset_keys.json");
    let valid = keys.iter().any(|k| k.key == key && !k.used && k.expires_unix.map(|e| e > now_unix()).unwrap_or(true));
    ok(serde_json::json!({"ok": valid, "valid": valid}))
}

fn reset_confirm(state: &AppState, params: &HashMap<String,String>) -> Response {
    let key = param(params, &["key", "token", "resetKey"]).unwrap_or("");
    let password = param(params, &["password", "newPassword", "pw"]).unwrap_or("");
    if password.len() < 4 { return err(StatusCode::BAD_REQUEST, "password too short"); }
    let mut keys: Vec<ResetKey> = read_vec(state, "reset_keys.json");
    let mut username = None;
    for k in &mut keys { if k.key == key && !k.used && k.expires_unix.map(|e| e > now_unix()).unwrap_or(true) { k.used = true; username = Some(k.username.clone()); } }
    let username = match username { Some(u) => u, None => return err(StatusCode::FORBIDDEN, "invalid or expired reset key") };
    if let Err(e)=write_vec(state, "reset_keys.json", &keys) { return err(StatusCode::INTERNAL_SERVER_ERROR, e); }
    match state.auth.upsert_user(&username, Some(password), false, None, None) { Ok(user) => ok(serde_json::json!({"ok": true, "user": user})), Err(e) => err(StatusCode::BAD_REQUEST, e) }
}

fn installer_start(state: &AppState, params: &HashMap<String,String>) -> Response {
    let target = param(params, &["target", "device"]).unwrap_or("");
    let mut install = read_value(state, "alpnas_installer_state.json");
    if !install.is_object() { install = serde_json::json!({}); }
    let obj = install.as_object_mut().unwrap();
    obj.insert("state".into(), serde_json::Value::String("planned".into()));
    obj.insert("target".into(), serde_json::Value::String(target.into()));
    obj.insert("updated_unix".into(), serde_json::Value::Number(now_unix().into()));
    match write_value(state, "alpnas_installer_state.json", &install) { Ok(()) => ok(serde_json::json!({"ok": true, "installer": install, "note": "disk-writing installer plan recorded; execution requires privileged deployment tooling"})), Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, e) }
}

fn generic_category(_state: &AppState, path: &str, _params: &HashMap<String,String>, _json_body: Option<serde_json::Value>) -> Response {
    let category = path.split('/').nth(2).unwrap_or("system");
    ok(serde_json::json!({"ok": true, "path": path, "category": category, "ported": "generic compatibility response", "note": "endpoint is registered and handled by Rust compatibility layer; hardware/external side effects are disabled"}))
}
