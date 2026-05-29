package main

import (
	"bufio"
	"encoding/json"
	"errors"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"sort"
	"strconv"
	"strings"

	fs "imuslab.com/arozos/mod/filesystem"
	module "imuslab.com/arozos/mod/modules"
	prout "imuslab.com/arozos/mod/prouter"
	"imuslab.com/arozos/mod/utils"
)

type alpnasDriveRecord struct {
	Device        string `json:"device"`
	ParentDevice  string `json:"parentDevice,omitempty"`
	Name          string `json:"name"`
	Type          string `json:"type"`
	Size          uint64 `json:"size"`
	Filesystem    string `json:"filesystem,omitempty"`
	Label         string `json:"label,omitempty"`
	UUID          string `json:"uuid,omitempty"`
	Mountpoint    string `json:"mountpoint,omitempty"`
	Mounted       bool   `json:"mounted"`
	ReadOnly      bool   `json:"readOnly"`
	Hotplug       bool   `json:"hotplug"`
	Removable     bool   `json:"removable"`
	Model         string `json:"model,omitempty"`
	Transport     string `json:"transport,omitempty"`
	StorageUUID   string `json:"storageUUID,omitempty"`
	StorageName   string `json:"storageName,omitempty"`
	OpenPath      string `json:"openPath,omitempty"`
	SystemStorage bool   `json:"systemStorage"`
	BootMedium    bool   `json:"bootMedium"`
	Mountable     bool   `json:"mountable"`
	Unmountable   bool   `json:"unmountable"`
	Status        string `json:"status"`
}

type alpnasDriveListResponse struct {
	IsAdmin    bool                `json:"isAdmin"`
	UserGroups []string            `json:"userGroups,omitempty"`
	Devices    []alpnasDriveRecord `json:"devices"`
}

const alpnasStorageStateFile = "/run/alpnas/storage-state.tsv"

func alpnasDriveServiceInit(authRouter *prout.RouterDef) {
	moduleHandler.RegisterModule(module.ModuleInfo{
		Name:        "AlpNAS Drives",
		Group:       "System Tools",
		IconPath:    "SystemAO/system_setting/img/drive.svg",
		Version:     "1.0",
		StartDir:    "SystemAO/disk/alpnas_drives.html",
		SupportFW:   true,
		InitFWSize:  []int{1180, 760},
		LaunchFWDir: "SystemAO/disk/alpnas_drives.html",
		SupportEmb:  false,
	})

	registerSetting(settingModule{
		Name:         "AlpNAS Drives",
		Desc:         "Browse, mount and open local AlpNAS storage devices",
		IconPath:     "SystemAO/system_setting/img/drive.svg",
		Group:        "Disk",
		StartDir:     "SystemAO/disk/alpnas_drives.html",
		RequireAdmin: false,
	})

	authRouter.HandleFunc("/system/disk/alpnas/list", alpnasHandleDriveList)
	authRouter.HandleFunc("/system/disk/alpnas/rescan", alpnasHandleDriveRescan)
	authRouter.HandleFunc("/system/disk/alpnas/mount", alpnasHandleDriveMount)
	authRouter.HandleFunc("/system/disk/alpnas/unmount", alpnasHandleDriveUnmount)
}

func alpnasHandleDriveList(w http.ResponseWriter, r *http.Request) {
	if runtime.GOOS != "linux" {
		utils.SendErrorResponse(w, "AlpNAS drive management is only supported on Linux")
		return
	}

	userinfo, err := userHandler.GetUserInfoFromRequest(w, r)
	if err != nil {
		utils.SendErrorResponse(w, "User not logged in")
		return
	}

	devices, err := alpnasCollectDriveRecords(userinfo.GetAllFileSystemHandler())
	if err != nil {
		utils.SendErrorResponse(w, err.Error())
		return
	}

	js, _ := json.Marshal(alpnasDriveListResponse{
		IsAdmin:    alpnasUserIsAdmin(userinfo),
		UserGroups: userinfo.GetUserPermissionGroupNames(),
		Devices:    devices,
	})
	utils.SendJSONResponse(w, string(js))
}

func alpnasHandleDriveRescan(w http.ResponseWriter, r *http.Request) {
	if _, err := alpnasRequireAdmin(w, r); err != nil {
		utils.SendErrorResponse(w, err.Error())
		return
	}

	message, err := alpnasRunDriveCtl("rescan")
	if err != nil {
		utils.SendErrorResponse(w, message)
		return
	}

	if err := alpnasRunStorageSync(); err != nil {
		utils.SendErrorResponse(w, err.Error())
		return
	}

	ReloadAllStoragePools("alpnas drive rescan")
	if message == "" {
		message = "Drive scan completed"
	}
	utils.SendJSONResponse(w, `{"status":"ok","message":"`+alpnasEscapeJSONString(message)+`"}`)
}

func alpnasHandleDriveMount(w http.ResponseWriter, r *http.Request) {
	if _, err := alpnasRequireAdmin(w, r); err != nil {
		utils.SendErrorResponse(w, err.Error())
		return
	}

	dev, err := utils.GetPara(r, "dev")
	if err != nil {
		utils.SendErrorResponse(w, "Missing parameter 'dev'")
		return
	}

	message, err := alpnasRunDriveCtl("mount", dev)
	if err != nil {
		utils.SendErrorResponse(w, message)
		return
	}

	if err := alpnasRunStorageSync(); err != nil {
		utils.SendErrorResponse(w, err.Error())
		return
	}

	ReloadAllStoragePools("alpnas manual mount")
	if message == "" {
		message = "Drive mounted"
	}
	utils.SendJSONResponse(w, `{"status":"ok","message":"`+alpnasEscapeJSONString(message)+`"}`)
}

func alpnasHandleDriveUnmount(w http.ResponseWriter, r *http.Request) {
	if _, err := alpnasRequireAdmin(w, r); err != nil {
		utils.SendErrorResponse(w, err.Error())
		return
	}

	target, _ := utils.GetPara(r, "target")
	if target == "" {
		dev, err := utils.GetPara(r, "dev")
		if err != nil {
			utils.SendErrorResponse(w, "Missing parameter 'target' or 'dev'")
			return
		}
		target = alpnasResolveCurrentMountpoint(dev)
	}

	if target == "" {
		utils.SendErrorResponse(w, "Unable to resolve mount target")
		return
	}

	message, err := alpnasRunDriveCtl("unmount", target)
	if err != nil {
		utils.SendErrorResponse(w, message)
		return
	}

	if err := alpnasRunStorageSync(); err != nil {
		utils.SendErrorResponse(w, err.Error())
		return
	}

	ReloadAllStoragePools("alpnas manual unmount")
	if message == "" {
		message = "Drive unmounted"
	}
	utils.SendJSONResponse(w, `{"status":"ok","message":"`+alpnasEscapeJSONString(message)+`"}`)
}

func alpnasRequireAdmin(w http.ResponseWriter, r *http.Request) (bool, error) {
	if runtime.GOOS != "linux" {
		return false, errors.New("AlpNAS drive management is only supported on Linux")
	}

	userinfo, err := userHandler.GetUserInfoFromRequest(w, r)
	if err != nil {
		return false, errors.New("User not logged in")
	}
	if !alpnasUserIsAdmin(userinfo) {
		return false, errors.New("Administrator privileges required")
	}
	return true, nil
}

func alpnasCollectDriveRecords(handlers []*fs.FileSystemHandler) ([]alpnasDriveRecord, error) {
	results := alpnasCollectStateRecords(handlers)
	if len(results) == 0 {
		results = alpnasCollectFdiskRecords(handlers)
	}
	fallbackRecords := alpnasCollectMountedFallback(handlers)
	if len(results) == 0 {
		results = fallbackRecords
	} else {
		seen := map[string]bool{}
		for _, record := range results {
			key := record.Device + "|" + record.Mountpoint
			seen[key] = true
		}
		for _, record := range fallbackRecords {
			key := record.Device + "|" + record.Mountpoint
			if !seen[key] {
				results = append(results, record)
			}
		}
	}

	sort.Slice(results, func(i, j int) bool {
		if results[i].Mounted != results[j].Mounted {
			return results[i].Mounted
		}
		if results[i].Removable != results[j].Removable {
			return results[i].Removable
		}
		if results[i].Name != results[j].Name {
			return results[i].Name < results[j].Name
		}
		return results[i].Device < results[j].Device
	})

	return results, nil
}

func alpnasCollectStateRecords(handlers []*fs.FileSystemHandler) []alpnasDriveRecord {
	raw, err := os.ReadFile(alpnasStorageStateFile)
	if err != nil || len(raw) == 0 {
		return []alpnasDriveRecord{}
	}

	results := []alpnasDriveRecord{}
	scanner := bufio.NewScanner(strings.NewReader(string(raw)))
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		if line == "" || strings.HasPrefix(line, "#") {
			continue
		}

		fields := strings.Split(line, "\t")
		if len(fields) < 20 || fields[0] != "D" {
			continue
		}

		device := alpnasCanonicalDevice(strings.TrimSpace(fields[1]))
		if device == "" || alpnasIsReservedDevice(device) {
			continue
		}

		parent := alpnasCanonicalDevice(strings.TrimSpace(fields[2]))
		deviceType := strings.TrimSpace(fields[3])
		if deviceType == "" {
			deviceType = "disk"
		}

		size := uint64(0)
		if parsed, err := strconv.ParseUint(strings.TrimSpace(fields[4]), 10, 64); err == nil {
			size = parsed
		}

		filesystem := strings.ToLower(strings.TrimSpace(fields[5]))
		label := strings.TrimSpace(fields[6])
		uuid := strings.TrimSpace(fields[7])
		mountpoint := alpnasCleanMountPath(strings.TrimSpace(fields[8]))
		readOnly := alpnasStateBool(fields[10]) || strings.TrimSpace(fields[9]) == "readonly"
		removable := alpnasStateBool(fields[11])
		hotplug := alpnasStateBool(fields[12])
		model := strings.TrimSpace(fields[13])
		transport := strings.TrimSpace(fields[14])
		bootMedium := alpnasStateBool(fields[15])
		systemStorage := alpnasStateBool(fields[16])
		status := strings.TrimSpace(fields[17])
		mountable := alpnasStateBool(fields[18])
		unmountable := alpnasStateBool(fields[19])
		mounted := mountpoint != ""

		if label == "" {
			switch {
			case mounted:
				label = filepath.Base(mountpoint)
			default:
				label = filepath.Base(device)
			}
		}

		openPath, storageUUID, storageName := alpnasResolveOpenPath(mountpoint, handlers)
		if status == "" {
			switch {
			case bootMedium:
				status = "boot"
			case mounted && systemStorage:
				status = "system"
			case mounted:
				status = "mounted"
			case filesystem == "":
				status = "unknown"
			default:
				status = "ready"
			}
		}

		results = append(results, alpnasDriveRecord{
			Device:        device,
			ParentDevice:  parent,
			Name:          label,
			Type:          deviceType,
			Size:          size,
			Filesystem:    filesystem,
			Label:         label,
			UUID:          uuid,
			Mountpoint:    mountpoint,
			Mounted:       mounted,
			ReadOnly:      readOnly,
			Hotplug:       hotplug,
			Removable:     removable,
			Model:         model,
			Transport:     transport,
			StorageUUID:   storageUUID,
			StorageName:   storageName,
			OpenPath:      openPath,
			SystemStorage: systemStorage,
			BootMedium:    bootMedium,
			Mountable:     mountable,
			Unmountable:   unmountable,
			Status:        status,
		})
	}

	return results
}

func alpnasStateBool(raw string) bool {
	switch strings.ToLower(strings.TrimSpace(raw)) {
	case "1", "true", "yes", "y", "readonly":
		return true
	default:
		return false
	}
}

func alpnasCollectFdiskRecords(handlers []*fs.FileSystemHandler) []alpnasDriveRecord {
	fdiskBin := alpnasLookupBinary("fdisk", "/sbin/fdisk", "/usr/sbin/fdisk", "/usr/bin/fdisk", "/bin/fdisk")
	cmd := exec.Command(fdiskBin, "-l")
	cmd.Env = alpnasCommandEnv()
	output, err := cmd.Output()
	if err != nil {
		return []alpnasDriveRecord{}
	}

	results := []alpnasDriveRecord{}
	seen := map[string]bool{}
	scanner := bufio.NewScanner(strings.NewReader(string(output)))
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		if line == "" {
			continue
		}

		if strings.HasPrefix(line, "Disk /dev/") {
			device, size := alpnasParseFdiskDiskLine(line)
			if device == "" || seen[device] || alpnasIsReservedDevice(device) || alpnasDeviceHasChildPartitions(device) {
				continue
			}
			record := alpnasBuildRecordFromDevice(device, "disk", size, handlers)
			if record.Device == "" {
				continue
			}
			seen[record.Device] = true
			results = append(results, record)
			continue
		}

		if !strings.HasPrefix(line, "/dev/") {
			continue
		}

		device, size := alpnasParseFdiskPartitionLine(line)
		if device == "" || seen[device] || alpnasIsReservedDevice(device) {
			continue
		}
		record := alpnasBuildRecordFromDevice(device, "part", size, handlers)
		if record.Device == "" {
			continue
		}
		seen[record.Device] = true
		results = append(results, record)
	}

	return results
}

func alpnasParseFdiskDiskLine(line string) (string, uint64) {
	fields := strings.Fields(line)
	if len(fields) < 2 {
		return "", 0
	}
	device := strings.TrimSuffix(strings.TrimSpace(fields[1]), ":")
	return device, alpnasExtractLeadingUint64(strings.TrimPrefix(line, "Disk "+device+":"))
}

func alpnasParseFdiskPartitionLine(line string) (string, uint64) {
	fields := strings.Fields(line)
	if len(fields) == 0 {
		return "", 0
	}
	device := strings.TrimSpace(fields[0])
	size := uint64(0)
	for _, field := range fields[1:] {
		if parsed, err := strconv.ParseUint(field, 10, 64); err == nil && parsed > 0 {
			size = parsed
		}
	}
	return device, size
}

func alpnasExtractLeadingUint64(input string) uint64 {
	input = strings.TrimSpace(input)
	for _, field := range strings.Fields(input) {
		if parsed, err := strconv.ParseUint(strings.TrimSuffix(field, ","), 10, 64); err == nil {
			return parsed
		}
	}
	return 0
}

func alpnasBuildRecordFromDevice(device string, defaultType string, size uint64, handlers []*fs.FileSystemHandler) alpnasDriveRecord {
	device = alpnasCanonicalDevice(device)
	if device == "" || !strings.HasPrefix(device, "/dev/") || alpnasIsReservedDevice(device) {
		return alpnasDriveRecord{}
	}

	fstype := strings.ToLower(strings.TrimSpace(alpnasReadBlkidValue(device, "TYPE")))
	label := strings.TrimSpace(alpnasReadBlkidValue(device, "LABEL"))
	partLabel := strings.TrimSpace(alpnasReadBlkidValue(device, "PARTLABEL"))
	if label == "" {
		label = partLabel
	}
	uuid := strings.TrimSpace(alpnasReadBlkidValue(device, "UUID"))
	partUUID := strings.TrimSpace(alpnasReadBlkidValue(device, "PARTUUID"))
	if uuid == "" {
		uuid = partUUID
	}

	mountpoint, readOnly := alpnasLookupMountpoint(device)
	deviceType := defaultType
	if alpnasDeviceHasChildPartitions(device) && strings.TrimSpace(fstype) == "" && mountpoint == "" {
		return alpnasDriveRecord{}
	}

	if deviceType == "" {
		deviceType = "disk"
	}
	if label == "" {
		label = filepath.Base(device)
	}
	openPath, storageUUID, storageName := alpnasResolveOpenPath(mountpoint, handlers)
	systemStorage := alpnasIsSystemMount(mountpoint)
	bootMedium := alpnasIsBootMedium(device, uuid, partUUID, label, partLabel)
	mounted := mountpoint != ""
	reservedFS := alpnasIsReservedFilesystem(fstype)
	mountable := !mounted && !reservedFS && !bootMedium && alpnasCanMountDevice(device, deviceType, fstype)
	unmountable := mounted && !systemStorage && alpnasCanUnmountPath(mountpoint)
	status := "ready"
	switch {
	case bootMedium:
		status = "boot"
	case mounted && systemStorage:
		status = "system"
	case mounted:
		status = "mounted"
	case reservedFS:
		status = "managed"
	case fstype == "":
		status = "unknown"
	}

	return alpnasDriveRecord{
		Device:        device,
		Name:          label,
		Type:          deviceType,
		Size:          size,
		Filesystem:    fstype,
		Label:         label,
		UUID:          uuid,
		Mountpoint:    mountpoint,
		Mounted:       mounted,
		ReadOnly:      readOnly,
		StorageUUID:   storageUUID,
		StorageName:   storageName,
		OpenPath:      openPath,
		SystemStorage: systemStorage,
		BootMedium:    bootMedium,
		Mountable:     mountable,
		Unmountable:   unmountable,
		Status:        status,
	}
}

func alpnasResolveOpenPath(mountpoint string, handlers []*fs.FileSystemHandler) (string, string, string) {
	target := filepath.Clean(filepath.ToSlash(strings.TrimSpace(mountpoint)))
	if target == "." || target == "" || target == "/" {
		return "", "", ""
	}

	var bestHandler *fs.FileSystemHandler
	bestLength := -1
	for _, handler := range handlers {
		if handler == nil || handler.Closed {
			continue
		}
		base := filepath.Clean(filepath.ToSlash(strings.TrimSpace(handler.Path)))
		if base == "." || base == "" {
			continue
		}
		if target != base && !strings.HasPrefix(target+"/", base+"/") {
			continue
		}
		if len(base) > bestLength {
			bestLength = len(base)
			bestHandler = handler
		}
	}

	if bestHandler == nil {
		return "", "", ""
	}

	rel, err := filepath.Rel(filepath.Clean(filepath.ToSlash(bestHandler.Path)), target)
	if err != nil {
		return bestHandler.UUID + ":/", bestHandler.UUID, bestHandler.Name
	}

	rel = filepath.ToSlash(rel)
	if rel == "." || rel == "" {
		return bestHandler.UUID + ":/", bestHandler.UUID, bestHandler.Name
	}

	return bestHandler.UUID + ":/" + strings.TrimPrefix(rel, "/"), bestHandler.UUID, bestHandler.Name
}

func alpnasCleanMountPath(input string) string {
	input = strings.TrimSpace(input)
	if input == "" {
		return ""
	}
	return filepath.Clean(filepath.ToSlash(input))
}

func alpnasCanMountDevice(device string, deviceType string, filesystem string) bool {
	if !strings.HasPrefix(device, "/dev/") {
		return false
	}
	if strings.TrimSpace(filesystem) == "" {
		return false
	}
	if alpnasIsReservedFilesystem(filesystem) {
		return false
	}

	switch strings.ToLower(strings.TrimSpace(deviceType)) {
	case "part", "lvm", "crypt", "raid", "md", "mpath":
		return true
	case "disk":
		return !alpnasDeviceHasChildPartitions(device)
	case "rom":
		return true
	case "mounted":
		return true
	default:
		return true
	}
}

func alpnasCanUnmountPath(mountpoint string) bool {
	if mountpoint == "" {
		return false
	}
	return strings.HasPrefix(mountpoint, "/media/") || strings.HasPrefix(mountpoint, "/mnt/")
}

func alpnasIsSystemMount(mountpoint string) bool {
	switch {
	case mountpoint == "/", mountpoint == "/boot", mountpoint == "/boot/efi":
		return true
	case mountpoint == "/tank", mountpoint == "/tank/apps", mountpoint == "/tank/data":
		return true
	default:
		return false
	}
}

func alpnasIsReservedFilesystem(fstype string) bool {
	switch strings.ToLower(strings.TrimSpace(fstype)) {
	case "", "swap", "lvm2_member", "linux_raid_member", "zfs_member", "crypto_luks", "bcache":
		return fstype != ""
	default:
		return false
	}
}

func alpnasReadBlkidValue(device string, key string) string {
	device = strings.TrimSpace(device)
	key = strings.TrimSpace(key)
	if device == "" || key == "" {
		return ""
	}

	blkidBin := alpnasLookupBinary("blkid", "/sbin/blkid", "/usr/sbin/blkid", "/usr/bin/blkid", "/bin/blkid")
	cmd := exec.Command(blkidBin, "-c", "/dev/null", "-o", "value", "-s", key, device)
	cmd.Env = alpnasCommandEnv()
	output, err := cmd.Output()
	if err == nil {
		value := strings.TrimSpace(string(output))
		if value != "" && !strings.HasPrefix(value, device+":") {
			return value
		}
	}

	cmd = exec.Command(blkidBin, "-c", "/dev/null", device)
	cmd.Env = alpnasCommandEnv()
	output, err = cmd.Output()
	if err != nil {
		return ""
	}

	line := strings.TrimSpace(string(output))
	if line == "" {
		return ""
	}

	quotedPrefix := key + "=\""
	if idx := strings.Index(line, quotedPrefix); idx >= 0 {
		rest := line[idx+len(quotedPrefix):]
		if end := strings.Index(rest, "\""); end >= 0 {
			return rest[:end]
		}
	}

	plainPrefix := key + "="
	if idx := strings.Index(line, plainPrefix); idx >= 0 {
		rest := line[idx+len(plainPrefix):]
		for i, r := range rest {
			if r == ' ' || r == '\t' {
				return rest[:i]
			}
		}
		return rest
	}

	return ""
}

func alpnasCmdlineMatches(values ...string) bool {
	raw, err := os.ReadFile("/proc/cmdline")
	if err != nil {
		return false
	}
	cmdline := strings.ToLower(string(raw))
	for _, value := range values {
		value = strings.ToLower(strings.TrimSpace(value))
		if value == "" {
			continue
		}
		if strings.Contains(cmdline, value) {
			return true
		}
	}
	return false
}

func alpnasLookupMountpoint(device string) (string, bool) {
	raw, err := os.ReadFile("/proc/mounts")
	if err != nil {
		return "", false
	}

	device = alpnasCanonicalDevice(device)
	lines := strings.Split(string(raw), "\n")
	for _, line := range lines {
		fields := strings.Fields(line)
		if len(fields) < 4 {
			continue
		}
		source := alpnasCanonicalDevice(fields[0])
		if source != device {
			continue
		}
		return alpnasCleanMountPath(strings.ReplaceAll(fields[1], `\040`, " ")), strings.Contains(","+fields[3]+",", ",ro,")
	}

	return "", false
}

func alpnasIsBootMedium(device string, uuid string, partUUID string, label string, partLabel string) bool {
	canonical := alpnasCanonicalDevice(device)
	deviceRoot := alpnasDeviceRootDisk(canonical)
	for _, source := range alpnasBootSources() {
		if canonical == source || deviceRoot == alpnasDeviceRootDisk(source) {
			return true
		}
	}

	lowerLabels := []string{
		strings.ToLower(strings.TrimSpace(label)),
		strings.ToLower(strings.TrimSpace(partLabel)),
	}
	if alpnasCmdlineMatches(
		filepath.Base(canonical),
		canonical,
		deviceRoot,
		"uuid="+strings.TrimSpace(uuid),
		"partuuid="+strings.TrimSpace(partUUID),
		"label="+strings.TrimSpace(label),
		"label="+strings.TrimSpace(partLabel),
		"apkovl="+filepath.Base(canonical),
	) {
		return true
	}

	for _, value := range lowerLabels {
		switch value {
		case "ventoy", "vtoyefi":
			if alpnasCmdlineMatches("ventoy", "vtoyefi") {
				return true
			}
		}
		if strings.HasPrefix(value, "alpine") || strings.Contains(value, "alpnas") {
			if alpnasCmdlineMatches(value, "apkovl", "modloop") {
				return true
			}
		}
	}

	return false
}

func alpnasRunDriveCtl(args ...string) (string, error) {
	binaryPath := "/usr/local/bin/alpnas-drivectl"
	if _, err := os.Stat(binaryPath); err != nil {
		return "alpnas-drivectl is not available on this system image", err
	}

	cmd := exec.Command(binaryPath, args...)
	cmd.Env = alpnasCommandEnv()
	output, err := cmd.CombinedOutput()
	message := strings.TrimSpace(string(output))
	if err != nil {
		if message == "" {
			message = err.Error()
		}
		return message, err
	}

	return message, nil
}

func alpnasRunStorageSync() error {
	binaryPath := "/usr/local/bin/alpnas-arozos-storage-sync"
	if _, err := os.Stat(binaryPath); err != nil {
		return errors.New("alpnas-arozos-storage-sync is not available on this system image")
	}

	cmd := exec.Command(binaryPath)
	cmd.Env = append(alpnasCommandEnv(), "ALPNAS_AROZOS_STORAGE_SYNC_SKIP_SIGNAL=1")
	output, err := cmd.CombinedOutput()
	if err != nil {
		message := strings.TrimSpace(string(output))
		if message == "" {
			message = err.Error()
		}
		return errors.New(message)
	}

	return nil
}

func alpnasResolveCurrentMountpoint(device string) string {
	device = strings.TrimSpace(device)
	if device == "" {
		return ""
	}

	raw, err := os.ReadFile("/proc/mounts")
	if err != nil {
		return ""
	}

	lines := strings.Split(string(raw), "\n")
	for _, line := range lines {
		fields := strings.Fields(line)
		if len(fields) < 2 {
			continue
		}
		if fields[0] != device {
			continue
		}
		return strings.ReplaceAll(fields[1], `\040`, " ")
	}

	return ""
}

func alpnasUserIsAdmin(userinfo interface {
	IsAdmin() bool
	GetUserPermissionGroupNames() []string
}) bool {
	if userinfo.IsAdmin() {
		return true
	}
	for _, name := range userinfo.GetUserPermissionGroupNames() {
		switch strings.ToLower(strings.TrimSpace(name)) {
		case "administrator", "administrators", "administratoren", "admin":
			return true
		}
	}
	return false
}

func alpnasCommandEnv() []string {
	return append(os.Environ(), "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin")
}

func alpnasLookupBinary(name string, candidates ...string) string {
	for _, candidate := range candidates {
		if candidate == "" {
			continue
		}
		if strings.Contains(candidate, "/") {
			if _, err := os.Stat(candidate); err == nil {
				return candidate
			}
			continue
		}
		if resolved, err := exec.LookPath(candidate); err == nil {
			return resolved
		}
	}
	return name
}

func alpnasSupportsLegacyDiskManager() bool {
	return true
}

func alpnasCanonicalDevice(device string) string {
	device = strings.TrimSpace(device)
	if device == "" {
		return ""
	}
	if resolved, err := filepath.EvalSymlinks(device); err == nil && resolved != "" {
		return resolved
	}
	return device
}

func alpnasIsReservedDevice(device string) bool {
	device = alpnasCanonicalDevice(device)
	switch {
	case strings.HasPrefix(device, "/dev/loop"),
		strings.HasPrefix(device, "/dev/ram"),
		strings.HasPrefix(device, "/dev/zram"),
		strings.HasPrefix(device, "/dev/fd"),
		strings.Contains(device, "mmcblk") && (strings.Contains(device, "boot") || strings.Contains(device, "rpmb")):
		return true
	default:
		return false
	}
}

func alpnasDeviceHasChildPartitions(device string) bool {
	device = alpnasCanonicalDevice(device)
	base := filepath.Base(device)
	sysdir := filepath.Join("/sys/class/block", base)
	entries, err := os.ReadDir(sysdir)
	if err != nil {
		return false
	}

	for _, entry := range entries {
		name := entry.Name()
		if name == base {
			continue
		}
		if strings.HasPrefix(name, base+"p") || strings.HasPrefix(name, base) {
			return true
		}
	}

	return false
}

func alpnasDeviceRootDisk(device string) string {
	device = filepath.Base(alpnasCanonicalDevice(device))
	switch {
	case strings.HasPrefix(device, "nvme") && strings.Contains(device, "p"):
		return "/dev/" + strings.Split(device, "p")[0]
	case strings.HasPrefix(device, "mmcblk") && strings.Contains(device, "p"):
		return "/dev/" + strings.Split(device, "p")[0]
	default:
		return "/dev/" + strings.TrimRight(device, "0123456789")
	}
}

func alpnasBootSources() []string {
	sources := []string{}
	add := func(value string) {
		value = alpnasCanonicalDevice(strings.TrimSpace(value))
		if value == "" || !strings.HasPrefix(value, "/dev/") {
			return
		}
		if !utils.StringInArray(sources, value) {
			sources = append(sources, value)
		}
	}

	raw, err := os.ReadFile("/proc/mounts")
	if err == nil {
		lines := strings.Split(string(raw), "\n")
		for _, line := range lines {
			fields := strings.Fields(line)
			if len(fields) < 2 {
				continue
			}
			target := strings.ReplaceAll(fields[1], `\040`, " ")
			switch target {
			case "/boot", "/boot/efi":
				add(fields[0])
			}
		}
	}

	return sources
}

func alpnasCollectMountedFallback(handlers []*fs.FileSystemHandler) []alpnasDriveRecord {
	file, err := os.Open("/proc/mounts")
	if err != nil {
		return []alpnasDriveRecord{}
	}
	defer file.Close()

	results := []alpnasDriveRecord{}
	seen := map[string]bool{}
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		fields := strings.Fields(scanner.Text())
		if len(fields) < 4 {
			continue
		}
		device := strings.TrimSpace(fields[0])
		mountpoint := strings.ReplaceAll(fields[1], `\040`, " ")
		fstype := strings.ToLower(strings.TrimSpace(fields[2]))
		opts := strings.TrimSpace(fields[3])
		if !strings.HasPrefix(device, "/dev/") || seen[device] {
			continue
		}
		if alpnasIsReservedFilesystem(fstype) {
			continue
		}
		seen[device] = true
		uuid := strings.TrimSpace(alpnasReadBlkidValue(device, "UUID"))
		partUUID := strings.TrimSpace(alpnasReadBlkidValue(device, "PARTUUID"))
		label := strings.TrimSpace(alpnasReadBlkidValue(device, "LABEL"))
		partLabel := strings.TrimSpace(alpnasReadBlkidValue(device, "PARTLABEL"))
		bootMedium := alpnasIsBootMedium(device, uuid, partUUID, label, partLabel)

		openPath, storageUUID, storageName := alpnasResolveOpenPath(mountpoint, handlers)
		status := "mounted"
		if bootMedium {
			status = "boot"
		}
		results = append(results, alpnasDriveRecord{
			Device:        device,
			Name:          filepath.Base(mountpoint),
			Type:          "mounted",
			Filesystem:    fstype,
			Mountpoint:    alpnasCleanMountPath(mountpoint),
			Mounted:       true,
			ReadOnly:      strings.Contains(","+opts+",", ",ro,"),
			StorageUUID:   storageUUID,
			StorageName:   storageName,
			OpenPath:      openPath,
			SystemStorage: alpnasIsSystemMount(mountpoint),
			BootMedium:    bootMedium,
			Unmountable:   alpnasCanUnmountPath(mountpoint) && !alpnasIsSystemMount(mountpoint),
			Status:        status,
		})
	}

	return results
}

func alpnasEscapeJSONString(input string) string {
	raw, _ := json.Marshal(input)
	quoted := string(raw)
	return strings.TrimSuffix(strings.TrimPrefix(quoted, `"`), `"`)
}
