# Endpoint Porting Matrix

| Path | Access | Rust status | Original handler |
|---|---:|---|---|
| `/` | public | native redirect | `mrouter(fs;h;http.HandlerFunc(func(w http.ResponseWriter, r *http.Request;func(w http.ResponseWriter, r *http.Request` |
| `/AOB/` | public | compatibility handler | `func(w http.ResponseWriter, r *http.Request` |
| `/AOB/SystemAOB/functions/info/version.inf` | public | compatibility handler | `systemIdServeVersonNumber` |
| `/AOB/SystemAOB/functions/system_statistic/getDriveStat.php` | public | compatibility handler | `systemIdGetDriveStates` |
| `/AOB/hb.php` | public | compatibility handler | `systemIdResponseBetaScan` |
| `/alpnas/auth/admin` | public | compatibility handler | `func(w http.ResponseWriter, r *http.Request` |
| `/api/ajgi/addExt` | authenticated | compatibility handler | `gw.AddExternalEndPoint` |
| `/api/ajgi/exec` | public | compatibility handler | `gw.HandleAgiExecutionRequestWithToken` |
| `/api/ajgi/interface` | public | compatibility handler | `func(w http.ResponseWriter, r *http.Request` |
| `/api/ajgi/listExt` | authenticated | compatibility handler | `gw.ListExternalEndpoint` |
| `/api/ajgi/rmExt` | authenticated | compatibility handler | `gw.RemoveExternalEndPoint` |
| `/api/auth/login` | public-or-token | native auth/autologin | `authAgent.HandleAutologinTokenLogin` |
| `/ldapLogin.html` | public-or-token | compatibility handler | `ldapHandler.HandleLoginPage` |
| `/media/` | public-or-token | native media compatibility | `mediaServer.ServerMedia` |
| `/media/download/` | public-or-token | native media compatibility | `mediaServer.ServerMedia` |
| `/media/getMime/` | public-or-token | native media compatibility | `mediaServer.ServeMediaMime` |
| `/media/transcode/` | public-or-token | native media compatibility | `mediaServer.ServeVideoWithTranscode;func(w http.ResponseWriter, r *http.Request` |
| `/public/register/checkPublicRegister` | public | compatibility handler | `registerHandler.HandleRegisterCheck` |
| `/public/register/handleRegister.html` | public | compatibility handler | `registerHandler.HandleRegisterRequest` |
| `/public/register/register.html` | public | compatibility handler | `registerHandler.HandleRegisterInterface` |
| `/reset.system` | public | compatibility handler | `system_resetpw_alpnasResetSystem` |
| `/share` | public-or-token | compatibility handler | `shareManager.HandleShareAccess` |
| `/ssdp.xml` | public | compatibility handler | `a.handleSSDP` |
| `/system/ajgi/interface` | public | compatibility handler | `func(w http.ResponseWriter, r *http.Request` |
| `/system/apt/list` | authenticated | compatibility handler | `apt.HandlePackageListRequest` |
| `/system/arsm/aecron/add` | authenticated | compatibility handler | `systemScheduler.HandleAddJob` |
| `/system/arsm/aecron/list` | public | compatibility handler | `func(w http.ResponseWriter, r *http.Request` |
| `/system/arsm/aecron/listlog` | authenticated | compatibility handler | `systemScheduler.HandleShowLog` |
| `/system/arsm/aecron/remove` | authenticated | compatibility handler | `systemScheduler.HandleJobRemoval` |
| `/system/auth/blacklist/ban` | admin | compatibility handler | `authAgent.BlacklistManager.HandleAddBannedIP` |
| `/system/auth/blacklist/enable` | admin | compatibility handler | `authAgent.BlacklistManager.HandleSetBlacklistEnable` |
| `/system/auth/blacklist/list` | admin | compatibility handler | `authAgent.BlacklistManager.HandleListBannedIPs` |
| `/system/auth/blacklist/unban` | admin | compatibility handler | `authAgent.BlacklistManager.HandleRemoveBannedIP` |
| `/system/auth/checkLogin` | public | native auth | `authAgent.CheckLogin` |
| `/system/auth/csvimport` | admin | compatibility handler | `authAgent.HandleCreateUserAccountsFromCSV` |
| `/system/auth/groupdel` | admin | compatibility handler | `authAgent.HandleUserDeleteByGroup` |
| `/system/auth/ldap/checkldap` | public | compatibility handler | `ldapHandler.HandleCheckLDAP` |
| `/system/auth/ldap/config/read` | admin | compatibility handler | `ldapHandler.ReadConfig` |
| `/system/auth/ldap/config/syncorizeUser` | admin | compatibility handler | `ldapHandler.SynchronizeUser` |
| `/system/auth/ldap/config/testConnection` | admin | compatibility handler | `ldapHandler.TestConnection` |
| `/system/auth/ldap/config/write` | admin | compatibility handler | `ldapHandler.WriteConfig` |
| `/system/auth/ldap/login` | public | compatibility handler | `ldapHandler.HandleLogin` |
| `/system/auth/ldap/newPassword` | public | compatibility handler | `ldapHandler.HandleNewPasswordPage` |
| `/system/auth/ldap/setPassword` | public | compatibility handler | `ldapHandler.HandleSetPassword` |
| `/system/auth/logger/index` | admin | compatibility handler | `authAgent.Logger.HandleIndexListing` |
| `/system/auth/logger/list` | admin | compatibility handler | `authAgent.Logger.HandleTableListing` |
| `/system/auth/login` | public-or-token | native auth | `authAgent.HandleLogin` |
| `/system/auth/logout` | public | native auth | `authAgent.HandleLogout` |
| `/system/auth/oauth/authorize` | public | compatibility handler | `oAuthHandler.HandleAuthorize` |
| `/system/auth/oauth/checkoauth` | public | compatibility handler | `oAuthHandler.CheckOAuth` |
| `/system/auth/oauth/config/read` | admin | compatibility handler | `oAuthHandler.ReadConfig` |
| `/system/auth/oauth/config/write` | admin | compatibility handler | `oAuthHandler.WriteConfig` |
| `/system/auth/oauth/login` | public | compatibility handler | `oAuthHandler.HandleLogin` |
| `/system/auth/register` | public | native auth | `authAgent.HandleRegister` |
| `/system/auth/u/list` | authenticated | compatibility handler | `authAgent.SwitchableAccountManager.HandleSwitchableAccountListing` |
| `/system/auth/u/logoutAll` | authenticated | compatibility handler | `authAgent.SwitchableAccountManager.HandleLogoutAllAccounts` |
| `/system/auth/u/p/list` | public | compatibility handler | `func(w http.ResponseWriter, r *http.Request` |
| `/system/auth/u/switch` | authenticated | compatibility handler | `authAgent.SwitchableAccountManager.HandleAccountSwitch` |
| `/system/auth/unregister` | admin | compatibility handler | `authAgent.HandleUnregister` |
| `/system/auth/whitelist/enable` | admin | compatibility handler | `authAgent.WhitelistManager.HandleSetWhitelistEnable` |
| `/system/auth/whitelist/list` | admin | compatibility handler | `authAgent.WhitelistManager.HandleListWhitelistedIPs` |
| `/system/auth/whitelist/set` | admin | compatibility handler | `authAgent.WhitelistManager.HandleAddWhitelistedIP` |
| `/system/auth/whitelist/unset` | admin | compatibility handler | `authAgent.WhitelistManager.HandleRemoveWhitelistedIP` |
| `/system/autologin/create` | admin | compatibility handler | `autoLoginHandler.HandleUserTokenCreation` |
| `/system/autologin/delete` | admin | compatibility handler | `autoLoginHandler.HandleUserTokenRemoval` |
| `/system/autologin/list` | admin | compatibility handler | `autoLoginHandler.HandleUserTokensListing` |
| `/system/bootflags` | admin | native config | `handleBootFlagsFunction` |
| `/system/cluster/record` | authenticated | compatibility handler | `NeighbourDiscoverer.HandleScanRecord` |
| `/system/cluster/scan` | authenticated | compatibility handler | `NeighbourDiscoverer.HandleScanningRequest` |
| `/system/cluster/wol` | authenticated | compatibility handler | `NeighbourDiscoverer.HandleWakeOnLan` |
| `/system/csrf/new` | authenticated | compatibility handler | `CSRFTokenManager.HandleNewToken` |
| `/system/desktop/createShortcut` | authenticated | compatibility handler | `desktop_shortcutHandler` |
| `/system/desktop/files` | authenticated | compatibility handler | `desktop_fileLocation_handler` |
| `/system/desktop/host` | authenticated | compatibility handler | `desktop_hostdetailHandler` |
| `/system/desktop/listDesktop` | authenticated | compatibility handler | `desktop_listFiles` |
| `/system/desktop/opr/renameShortcut` | authenticated | compatibility handler | `desktop_handleShortcutRename` |
| `/system/desktop/preference` | authenticated | compatibility handler | `desktop_preference_handler` |
| `/system/desktop/theme` | authenticated | compatibility handler | `desktop_theme_handler` |
| `/system/desktop/user` | authenticated | compatibility handler | `desktop_handleUserInfo` |
| `/system/disk/alpnas/list` | authenticated | compatibility handler | `alpnasHandleDriveList` |
| `/system/disk/alpnas/mount` | authenticated | compatibility handler | `alpnasHandleDriveMount` |
| `/system/disk/alpnas/rescan` | authenticated | compatibility handler | `alpnasHandleDriveRescan` |
| `/system/disk/alpnas/unmount` | authenticated | compatibility handler | `alpnasHandleDriveUnmount` |
| `/system/disk/devices/list` | admin | compatibility handler | `raidManager.HandleListUsableDevices` |
| `/system/disk/devices/model` | admin | compatibility handler | `raidManager.HandleResolveDiskModelLabel` |
| `/system/disk/diskmg/format` | admin | compatibility handler | `func(w http.ResponseWriter, r *http.Request` |
| `/system/disk/diskmg/mount` | admin | compatibility handler | `func(w http.ResponseWriter, r *http.Request` |
| `/system/disk/diskmg/mpt` | admin | compatibility handler | `diskmg.HandleListMountPoints` |
| `/system/disk/diskmg/platform` | admin | compatibility handler | `diskmg.HandlePlatform` |
| `/system/disk/diskmg/view` | authenticated | compatibility handler | `diskmg.HandleView` |
| `/system/disk/quota/quotaDist` | public | compatibility handler | `system_disk_quota_handleFileDistributionView` |
| `/system/disk/quota/quotaInfo` | public | compatibility handler | `system_disk_quota_handleQuotaInfo` |
| `/system/disk/quota/setQuota` | public | compatibility handler | `system_disk_quota_setQuota` |
| `/system/disk/raid/addMemeber` | admin | compatibility handler | `raidManager.HandleAddDiskToRAIDVol` |
| `/system/disk/raid/assemble` | admin | compatibility handler | `func(w http.ResponseWriter, r *http.Request;raidManager.HandleRaidDevicesAssemble` |
| `/system/disk/raid/detail` | admin | compatibility handler | `raidManager.HandleLoadArrayDetail` |
| `/system/disk/raid/devinfo` | admin | compatibility handler | `raidManager.HandlListChildrenDeviceInfo` |
| `/system/disk/raid/format` | admin | compatibility handler | `raidManager.HandleFormatRaidDevice` |
| `/system/disk/raid/grow` | admin | compatibility handler | `raidManager.HandleGrowRAIDArray` |
| `/system/disk/raid/list` | admin | compatibility handler | `raidManager.HandleListRaidDevices` |
| `/system/disk/raid/new` | admin | compatibility handler | `raidManager.HandleCreateRAIDDevice` |
| `/system/disk/raid/overview` | admin | compatibility handler | `raidManager.HandleRenderOverview` |
| `/system/disk/raid/reload` | admin | compatibility handler | `raidManager.HandleMdadmFlushReload` |
| `/system/disk/raid/remove` | admin | compatibility handler | `func(w http.ResponseWriter, r *http.Request` |
| `/system/disk/raid/removeMemeber` | admin | compatibility handler | `raidManager.HandleRemoveDiskFromRAIDVol` |
| `/system/disk/smart/getSMART` | authenticated | compatibility handler | `smartListener.GetSMART` |
| `/system/disk/space/largeFiles` | authenticated | compatibility handler | `lfs.HandleLargeFileList` |
| `/system/disk/space/list` | authenticated | compatibility handler | `diskspace.HandleDiskSpaceList` |
| `/system/disk/space/resolve` | authenticated | compatibility handler | `dc.HandleCapacityResolving` |
| `/system/disk/space/tmp` | authenticated | compatibility handler | `dc.HandleTmpCapacityResolving` |
| `/system/file_system/clearTrash` | authenticated | compatibility handler | `system_fs_clearTrashBin` |
| `/system/file_system/fileOpr` | authenticated | native file api | `system_fs_handleOpr` |
| `/system/file_system/getProperties` | authenticated | native file api | `system_fs_getFileProperties` |
| `/system/file_system/handleCacheRender` | authenticated | compatibility handler | `system_fs_handleCacheRender` |
| `/system/file_system/handleFilePermission` | authenticated | compatibility handler | `system_fs_handleFilePermission` |
| `/system/file_system/handleFolderCache` | authenticated | compatibility handler | `system_fs_handleFolderCache` |
| `/system/file_system/listDir` | authenticated | native file api | `system_fs_handleList` |
| `/system/file_system/listDirHash` | authenticated | native hash | `system_fs_handleDirHash` |
| `/system/file_system/listDrives` | authenticated | native file api | `system_fs_listDrives` |
| `/system/file_system/listRoots` | authenticated | native file api | `system_fs_listRoot` |
| `/system/file_system/listTrash` | authenticated | compatibility handler | `system_fs_scanTrashBin` |
| `/system/file_system/loadThumbnail` | authenticated | compatibility handler | `system_fs_handleThumbnailLoad` |
| `/system/file_system/lowmemUpload` | authenticated | native multipart upload | `system_fs_handleLowMemoryUpload` |
| `/system/file_system/newItem` | authenticated | native file api | `system_fs_handleNewObjects` |
| `/system/file_system/ongoing` | authenticated | compatibility handler | `system_fs_HandleOnGoingTasks` |
| `/system/file_system/preference` | authenticated | compatibility handler | `system_fs_handleUserPreference` |
| `/system/file_system/restoreTrash` | authenticated | compatibility handler | `system_fs_restoreFile` |
| `/system/file_system/search` | authenticated | native file api | `system_fs_handleFileSearch` |
| `/system/file_system/share/checkShared` | authenticated | compatibility handler | `shareManager.HandleShareCheck` |
| `/system/file_system/share/delete` | authenticated | compatibility handler | `shareManager.HandleDeleteShare` |
| `/system/file_system/share/edit` | authenticated | compatibility handler | `shareManager.HandleEditShare` |
| `/system/file_system/share/list` | authenticated | compatibility handler | `shareManager.HandleListAllShares` |
| `/system/file_system/share/new` | authenticated | compatibility handler | `shareManager.HandleCreateNewShare` |
| `/system/file_system/sortMode` | authenticated | compatibility handler | `system_fs_handleFolderSortModePreference` |
| `/system/file_system/upload` | authenticated | native multipart upload | `system_fs_handleUpload` |
| `/system/file_system/validateFileOpr` | authenticated | native validation | `system_fs_validateFileOpr` |
| `/system/file_system/versionHistory` | authenticated | compatibility handler | `system_fs_FileVersionHistory` |
| `/system/file_system/ws/fileOpr` | authenticated | compatibility handler | `system_fs_handleWebSocketOpr` |
| `/system/file_system/ws/listTrash` | authenticated | compatibility handler | `system_fs_WebSocketScanTrashBin` |
| `/system/file_system/zipHandler` | authenticated | compatibility handler | `system_fs_zipHandler` |
| `/system/id/ping` | public | native ping | `systemIdHandlePing` |
| `/system/id/requestInfo` | public | native id info | `systemIdHandleRequest` |
| `/system/info/getArOZInfo` | public | native system info | `infoServer.GetArOZInfo` |
| `/system/info/getCPUinfo` | authenticated | native procfs | `info.GetCPUInfo` |
| `/system/info/getDriveStat` | authenticated | native df | `info.GetDriveStat` |
| `/system/info/getRAMinfo` | authenticated | native procfs | `info.GetRamInfo` |
| `/system/info/getRuntimeInfo` | authenticated | native runtime | `InfoHandleGetRuntimeInfo` |
| `/system/info/getUsageInfo` | authenticated | compatibility handler | `InfoHandleTaskInfo` |
| `/system/info/ifconfig` | authenticated | native sysfs | `info.Ifconfig` |
| `/system/info/license` | public | native license | `systemHandleListLicense` |
| `/system/info/usbPorts` | authenticated | compatibility handler | `info.GetUSB` |
| `/system/info/wallpaper.jpg` | public | compatibility handler | `func(w http.ResponseWriter, r *http.Request` |
| `/system/installer/alpnas/start` | authenticated | compatibility handler | `alpnasHandleInstallerStart` |
| `/system/installer/alpnas/state` | authenticated | compatibility handler | `alpnasHandleInstallerState` |
| `/system/installer/alpnas/targets` | authenticated | compatibility handler | `alpnasHandleInstallerTargets` |
| `/system/iot/execute` | authenticated | compatibility handler | `iotManager.HandleExecute` |
| `/system/iot/icon` | authenticated | compatibility handler | `iotManager.HandleIconLoad` |
| `/system/iot/list` | authenticated | compatibility handler | `iotManager.HandleListing` |
| `/system/iot/listScanner` | admin | compatibility handler | `iotManager.HandleScannerList` |
| `/system/iot/nickname` | authenticated | compatibility handler | `iotManager.HandleNickName` |
| `/system/iot/scan` | authenticated | compatibility handler | `iotManager.HandleScanning` |
| `/system/iot/status` | authenticated | compatibility handler | `iotManager.HandleGetDeviceStatus` |
| `/system/log/list` | admin | compatibility handler | `logViewer.HandleListLog` |
| `/system/log/read` | admin | compatibility handler | `logViewer.HandleReadLog` |
| `/system/module/install` | authenticated | compatibility handler | `HandleModuleInstall` |
| `/system/modules/getDefault` | public | compatibility handler | `func(w http.ResponseWriter, r *http.Request` |
| `/system/modules/getLaunchPara` | public | compatibility handler | `func(w http.ResponseWriter, r *http.Request` |
| `/system/modules/installViaZip` | public | compatibility handler | `func(w http.ResponseWriter, r *http.Request` |
| `/system/modules/list` | public | compatibility handler | `func(w http.ResponseWriter, r *http.Request` |
| `/system/modules/reload` | admin | compatibility handler | `func(w http.ResponseWriter, r *http.Request` |
| `/system/network/connectWifi` | authenticated | compatibility handler | `network_wifi_handleConnect` |
| `/system/network/getNICUsage` | authenticated | compatibility handler | `netstat.HandleGetNetworkInterfaceStats` |
| `/system/network/getNICinfo` | authenticated | compatibility handler | `network.GetNICInfo` |
| `/system/network/getPing` | authenticated | compatibility handler | `network.GetPing` |
| `/system/network/portforward` | authenticated | compatibility handler | `portforward_handleForward` |
| `/system/network/power` | authenticated | compatibility handler | `network_wifi_handleWiFiPower` |
| `/system/network/removeWifi` | authenticated | compatibility handler | `network_wifi_handleWiFiRemove` |
| `/system/network/scanWifi` | authenticated | compatibility handler | `network_wifi_handleScan` |
| `/system/network/server/endpoints` | authenticated | compatibility handler | `NetworkHandleGetFileServerEndpoints` |
| `/system/network/server/list` | authenticated | compatibility handler | `NetworkHandleGetFileServerServiceList` |
| `/system/network/server/status` | authenticated | compatibility handler | `NetworkHandleGetFileServerStatus` |
| `/system/network/server/toggle` | admin | compatibility handler | `NetworkHandleFileServerToggle` |
| `/system/network/webdav/clear` | authenticated | compatibility handler | `WebDAVManager.HandleClearAllPending` |
| `/system/network/webdav/edit` | authenticated | compatibility handler | `WebDAVManager.HandlePermissionEdit` |
| `/system/network/webdav/list` | public | compatibility handler | `WebDAVManager.HandleConnectionList` |
| `/system/network/webdav/status` | authenticated | compatibility handler | `WebDAVManager.HandleStatusChange` |
| `/system/network/wifiinfo` | authenticated | compatibility handler | `network_wifi_handleWiFiInfo` |
| `/system/network/www/toggle` | authenticated | compatibility handler | `userWwwHandler.HandleToggleHomepage` |
| `/system/network/www/webRoot` | authenticated | compatibility handler | `userWwwHandler.HandleSetWebRoot` |
| `/system/permission/delgroup` | admin | compatibility handler | `permissionHandler.HandleGroupRemove` |
| `/system/permission/editgroup` | admin | compatibility handler | `permissionHandler.HandleGroupEdit` |
| `/system/permission/listgroup` | public | compatibility handler | `func(w http.ResponseWriter, r *http.Request` |
| `/system/permission/newgroup` | admin | compatibility handler | `permissionHandler.HandleGroupCreate` |
| `/system/power/accessCheck` | public | compatibility handler | `hardware_power_checkIfHardware` |
| `/system/power/restart` | public | compatibility handler | `hardware_power_restart` |
| `/system/power/shutdown` | public | compatibility handler | `hardware_power_poweroff` |
| `/system/register/cleanUserRegisterInfo` | admin | compatibility handler | `register_handleRegisterCleaning` |
| `/system/register/email` | authenticated | compatibility handler | `registerHandler.HandleEmailChange` |
| `/system/register/getAllowRegistry` | admin | compatibility handler | `register_handleGetAllowRegistry` |
| `/system/register/listUserEmails` | admin | compatibility handler | `register_handleEmailListing` |
| `/system/register/setAllowRegistry` | admin | compatibility handler | `register_handleToggleRegistry` |
| `/system/register/setDefaultGroup` | admin | compatibility handler | `register_handleSetDefaultGroup` |
| `/system/reset/confirmPasswordReset` | public | compatibility handler | `system_resetpw_confirmReset` |
| `/system/reset/validateResetKey` | public | compatibility handler | `system_resetpw_validateResetKeyHandler` |
| `/system/setting/list` | public | native config | `system_setting_handleListing` |
| `/system/storage/ftp/passivemode` | admin | compatibility handler | `FTPManager.HandleFTPPassiveModeSettings` |
| `/system/storage/ftp/setPort` | admin | compatibility handler | `FTPManager.HandleFTPSetPort` |
| `/system/storage/ftp/start` | admin | compatibility handler | `FTPManager.HandleFTPServerStart` |
| `/system/storage/ftp/status` | admin | compatibility handler | `FTPManager.HandleFTPServerStatus` |
| `/system/storage/ftp/stop` | admin | compatibility handler | `FTPManager.HandleFTPServerStop` |
| `/system/storage/ftp/updateGroups` | admin | compatibility handler | `FTPManager.HandleFTPAccessUpdate` |
| `/system/storage/ftp/upnp` | admin | compatibility handler | `FTPManager.HandleFTPUPnP` |
| `/system/storage/pool/bridge` | admin | compatibility handler | `HandleFSHBridging` |
| `/system/storage/pool/checkBridge` | admin | compatibility handler | `HandleFSHBridgeCheck` |
| `/system/storage/pool/edit` | admin | compatibility handler | `HandleFSHEdit` |
| `/system/storage/pool/list` | admin | compatibility handler | `HandleListStoragePools` |
| `/system/storage/pool/listraw` | admin | compatibility handler | `HandleListStoragePoolsConfig` |
| `/system/storage/pool/newHandler` | admin | compatibility handler | `HandleStorageNewFsHandler` |
| `/system/storage/pool/reload` | admin | compatibility handler | `HandleStoragePoolReload` |
| `/system/storage/pool/removeHandler` | admin | compatibility handler | `HandleStoragePoolRemove` |
| `/system/storage/pool/toggle` | admin | compatibility handler | `HandleFSHToggle` |
| `/system/storage/samba/activate` | authenticated | compatibility handler | `func(w http.ResponseWriter, r *http.Request` |
| `/system/storage/samba/add` | admin | compatibility handler | `SambaShareManager.AddSambaShare` |
| `/system/storage/samba/addUser` | admin | compatibility handler | `SambaShareManager.NewSambaUser` |
| `/system/storage/samba/deactivate` | admin | compatibility handler | `SambaShareManager.DeactiveUserAccount` |
| `/system/storage/samba/delUser` | admin | compatibility handler | `SambaShareManager.DelSambaUser` |
| `/system/storage/samba/editPath` | admin | compatibility handler | `SambaShareManager.HandleSharePathChange` |
| `/system/storage/samba/list` | admin | compatibility handler | `SambaShareManager.ListSambaShares` |
| `/system/storage/samba/listUsers` | admin | compatibility handler | `SambaShareManager.ListSambaUsers` |
| `/system/storage/samba/myshare` | admin | compatibility handler | `SambaShareManager.HandleUserSmbStatusList` |
| `/system/storage/samba/myshare/delete` | admin | compatibility handler | `SambaShareManager.DelUserSambaShare` |
| `/system/storage/samba/remove` | admin | compatibility handler | `SambaShareManager.DelSambaShare` |
| `/system/storage/samba/status` | admin | compatibility handler | `SambaShareManager.SmbdStates` |
| `/system/storage/samba/updateShareUsers` | admin | compatibility handler | `SambaShareManager.HandleAccessUserUpdate` |
| `/system/storage/sftp/port` | admin | compatibility handler | `SFTPManager.HandleListeningPort` |
| `/system/storage/sftp/upnp` | admin | compatibility handler | `SFTPManager.HandleToogleUPnP` |
| `/system/storage/sftp/users` | admin | compatibility handler | `SFTPManager.HandleGetConnectedClients` |
| `/system/storage/tftp/defaultUser` | admin | compatibility handler | `TFTPManager.HandleTFTPDefaultUser` |
| `/system/storage/tftp/setPort` | admin | compatibility handler | `TFTPManager.HandleTFTPPort` |
| `/system/storage/tftp/status` | admin | compatibility handler | `TFTPManager.HandleTFTPServerStatus` |
| `/system/subservice/kill` | admin | compatibility handler | `ssRouter.HandleKillSubService` |
| `/system/subservice/list` | admin | compatibility handler | `ssRouter.HandleListing` |
| `/system/subservice/start` | admin | compatibility handler | `ssRouter.HandleStartSubService` |
| `/system/update/checkpending` | admin | compatibility handler | `updates.HandlePendingCheck` |
| `/system/update/checksize` | admin | compatibility handler | `updates.HandleUpdateCheckSize` |
| `/system/update/download` | admin | compatibility handler | `updates.HandleUpdateDownloadRequest` |
| `/system/update/platform` | admin | compatibility handler | `updates.HandleGetUpdatePlatformInfo` |
| `/system/update/restart` | admin | compatibility handler | `func(w http.ResponseWriter, r *http.Request` |
| `/system/users/editUser` | admin | compatibility handler | `user_handleUserEdit` |
| `/system/users/interfaceinfo` | public | compatibility handler | `func(w http.ResponseWriter, r *http.Request` |
| `/system/users/list` | public | compatibility handler | `user_handleList` |
| `/system/users/profilepic` | public | compatibility handler | `func(w http.ResponseWriter, r *http.Request` |
| `/system/users/removeUser` | admin | compatibility handler | `user_handleUserRemove` |
| `/system/users/userinfo` | public | compatibility handler | `func(w http.ResponseWriter, r *http.Request` |
| `/system/ws` | authenticated | compatibility handler | `WebSocketRouter.HandleWebSocketRouting` |
