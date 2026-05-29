package main

import (
	"bufio"
	"encoding/json"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"sort"
	"strconv"
	"strings"

	prout "imuslab.com/arozos/mod/prouter"
	"imuslab.com/arozos/mod/utils"
)

type alpnasInstallerTarget struct {
	Device          string   `json:"device"`
	Name            string   `json:"name"`
	Model           string   `json:"model,omitempty"`
	Transport       string   `json:"transport,omitempty"`
	Size            uint64   `json:"size"`
	Removable       bool     `json:"removable"`
	Hotplug         bool     `json:"hotplug"`
	ReadOnly        bool     `json:"readOnly"`
	FilesystemHints []string `json:"filesystemHints,omitempty"`
	MountedChildren int      `json:"mountedChildren"`
	PartitionCount  int      `json:"partitionCount"`
	BootMedium      bool     `json:"bootMedium"`
}

type alpnasInstallerTargetsResponse struct {
	LiveMode bool                    `json:"liveMode"`
	Targets  []alpnasInstallerTarget `json:"targets"`
}

type alpnasInstallerStateResponse struct {
	Available bool              `json:"available"`
	LiveMode  bool              `json:"liveMode"`
	Status    string            `json:"status"`
	Phase     string            `json:"phase"`
	Message   string            `json:"message"`
	State     map[string]string `json:"state"`
	LogTail   []string          `json:"logTail"`
	Running   bool              `json:"running"`
}

const (
	alpnasInstallerScript    = "/usr/local/bin/alpnas-install-system"
	alpnasInstallerStateFile = "/var/lib/alpnas-installer/state.env"
	alpnasInstallerLogFile   = "/var/lib/alpnas-installer/install.log"
	alpnasInstallerMinBytes  = 2 * 1024 * 1024 * 1024
)

func alpnasInstallerServiceInit(authRouter *prout.RouterDef) {
	if !alpnasInstallerAvailable() {
		return
	}

	registerSetting(settingModule{
		Name:         "AlpNAS Installer",
		Desc:         "Install AlpNAS from the live system onto a local disk",
		IconPath:     "SystemAO/system_setting/img/code.svg",
		Group:        "Advance",
		StartDir:     "SystemAO/installer/alpnas_installer.html",
		RequireAdmin: true,
	})

	authRouter.HandleFunc("/system/installer/alpnas/state", alpnasHandleInstallerState)
	authRouter.HandleFunc("/system/installer/alpnas/targets", alpnasHandleInstallerTargets)
	authRouter.HandleFunc("/system/installer/alpnas/start", alpnasHandleInstallerStart)
}

func alpnasHandleInstallerState(w http.ResponseWriter, r *http.Request) {
	if _, err := alpnasRequireAdmin(w, r); err != nil {
		utils.SendErrorResponse(w, err.Error())
		return
	}

	state := alpnasReadInstallerState()
	status := strings.TrimSpace(state["STATUS"])
	phase := strings.TrimSpace(state["PHASE"])
	message := strings.TrimSpace(state["MESSAGE"])
	if status == "" {
		status = "idle"
	}

	response := alpnasInstallerStateResponse{
		Available: alpnasInstallerAvailable(),
		LiveMode:  alpnasIsLiveMode(),
		Status:    status,
		Phase:     phase,
		Message:   message,
		State:     state,
		LogTail:   alpnasTailLog(alpnasInstallerLogFile, 80),
		Running:   status == "running",
	}

	js, _ := json.Marshal(response)
	utils.SendJSONResponse(w, string(js))
}

func alpnasHandleInstallerTargets(w http.ResponseWriter, r *http.Request) {
	if _, err := alpnasRequireAdmin(w, r); err != nil {
		utils.SendErrorResponse(w, err.Error())
		return
	}

	js, _ := json.Marshal(alpnasInstallerTargetsResponse{
		LiveMode: alpnasIsLiveMode(),
		Targets:  alpnasCollectInstallerTargets(),
	})
	utils.SendJSONResponse(w, string(js))
}

func alpnasHandleInstallerStart(w http.ResponseWriter, r *http.Request) {
	if !alpnasInstallerAvailable() {
		utils.SendErrorResponse(w, "AlpNAS installer is only available in live mode")
		return
	}
	if r.Method != http.MethodPost {
		utils.SendErrorResponse(w, "method_not_allowed")
		return
	}
	if !AuthValidateSecureRequest(w, r, true) {
		return
	}

	disk, err := utils.PostPara(r, "disk")
	if err != nil || strings.TrimSpace(disk) == "" {
		utils.SendErrorResponse(w, "Missing parameter 'disk'")
		return
	}
	hostname, _ := utils.PostPara(r, "hostname")
	timezone, _ := utils.PostPara(r, "timezone")
	hostname = strings.TrimSpace(hostname)
	timezone = strings.TrimSpace(timezone)
	if hostname == "" {
		hostname = "alpnas"
	}
	if timezone == "" {
		timezone = "UTC"
	}

	validTarget := false
	for _, target := range alpnasCollectInstallerTargets() {
		if target.Device == strings.TrimSpace(disk) {
			validTarget = true
			break
		}
	}
	if !validTarget {
		utils.SendErrorResponse(w, "Selected disk is not an installable target")
		return
	}

	state := alpnasReadInstallerState()
	if strings.TrimSpace(state["STATUS"]) == "running" {
		utils.SendErrorResponse(w, "An installation is already running")
		return
	}

	cmd := exec.Command(alpnasInstallerScript, "install", disk, hostname, timezone)
	cmd.Env = alpnasCommandEnv()
	if err := cmd.Start(); err != nil {
		utils.SendErrorResponse(w, "Unable to start installer: "+err.Error())
		return
	}
	go func() {
		_ = cmd.Wait()
	}()

	utils.SendJSONResponse(w, `{"ok":true,"message":"Installation started"}`)
}

func alpnasInstallerAvailable() bool {
	if runtime.GOOS != "linux" {
		return false
	}
	if !alpnasIsLiveMode() {
		return false
	}
	_, err := os.Stat(alpnasInstallerScript)
	return err == nil
}

func alpnasIsLiveMode() bool {
	raw, err := os.ReadFile("/proc/mounts")
	if err == nil {
		for _, line := range strings.Split(string(raw), "\n") {
			fields := strings.Fields(line)
			if len(fields) < 3 {
				continue
			}
			if fields[1] != "/" {
				continue
			}
			switch fields[2] {
			case "overlay", "tmpfs", "squashfs", "aufs":
				return true
			}
			break
		}
	}

	return alpnasCmdlineMatches("apkovl", "modloop", "overlaytmpfs")
}

func alpnasCollectInstallerTargets() []alpnasInstallerTarget {
	targets := []alpnasInstallerTarget{}
	seen := map[string]bool{}

	for _, device := range alpnasInstallerBlockDiskCandidates() {
		device = alpnasCanonicalDevice(strings.TrimSpace(device))
		if device == "" || seen[device] {
			continue
		}
		seen[device] = true

		if alpnasIsReservedDevice(device) {
			continue
		}
		if alpnasInstallerSysfsBool(device, "ro") {
			continue
		}

		size := alpnasInstallerDeviceSize(device)
		if size < alpnasInstallerMinBytes {
			continue
		}

		label := strings.TrimSpace(alpnasReadBlkidValue(device, "LABEL"))
		partLabel := strings.TrimSpace(alpnasReadBlkidValue(device, "PARTLABEL"))
		uuid := strings.TrimSpace(alpnasReadBlkidValue(device, "UUID"))
		partUUID := strings.TrimSpace(alpnasReadBlkidValue(device, "PARTUUID"))
		bootMedium := alpnasInstallerDiskIsBootMedium(device, uuid, partUUID, label, partLabel)
		if bootMedium {
			continue
		}

		targets = append(targets, alpnasInstallerTarget{
			Device:          device,
			Name:            alpnasInstallerDeviceName(device),
			Model:           alpnasInstallerReadSysfs(device, "device/model"),
			Transport:       alpnasInstallerTransport(device),
			Size:            size,
			Removable:       alpnasInstallerSysfsBool(device, "removable"),
			Hotplug:         alpnasInstallerIsHotplug(device),
			ReadOnly:        false,
			FilesystemHints: alpnasInstallerFilesystemHints(device),
			MountedChildren: alpnasInstallerMountedChildren(device),
			PartitionCount:  alpnasInstallerPartitionCount(device),
			BootMedium:      false,
		})
	}

	sort.Slice(targets, func(i, j int) bool {
		if targets[i].Removable != targets[j].Removable {
			return !targets[i].Removable
		}
		if targets[i].Size != targets[j].Size {
			return targets[i].Size > targets[j].Size
		}
		return targets[i].Device < targets[j].Device
	})

	return targets
}

func alpnasInstallerBlockDiskCandidates() []string {
	devices := []string{}
	seen := map[string]bool{}
	add := func(device string) {
		device = alpnasCanonicalDevice(strings.TrimSpace(device))
		if device == "" || !strings.HasPrefix(device, "/dev/") || seen[device] {
			return
		}
		seen[device] = true
		devices = append(devices, device)
	}

	if entries, err := os.ReadDir("/sys/class/block"); err == nil {
		for _, entry := range entries {
			name := strings.TrimSpace(entry.Name())
			if name == "" {
				continue
			}
			device := "/dev/" + name
			if _, err := os.Stat(device); err != nil {
				continue
			}
			if _, err := os.Stat(filepath.Join("/sys/class/block", name, "partition")); err == nil {
				continue
			}
			if alpnasIsReservedDevice(device) {
				continue
			}
			add(device)
		}
	}

	fdiskBin := alpnasLookupBinary("fdisk", "/sbin/fdisk", "/usr/sbin/fdisk", "/usr/bin/fdisk", "/bin/fdisk")
	cmd := exec.Command(fdiskBin, "-l")
	cmd.Env = alpnasCommandEnv()
	if output, err := cmd.Output(); err == nil {
		scanner := bufio.NewScanner(strings.NewReader(string(output)))
		for scanner.Scan() {
			line := strings.TrimSpace(scanner.Text())
			if !strings.HasPrefix(line, "Disk /dev/") {
				continue
			}
			device, _ := alpnasParseFdiskDiskLine(line)
			if alpnasIsReservedDevice(device) {
				continue
			}
			add(device)
		}
	}

	sort.Strings(devices)
	return devices
}

func alpnasInstallerDeviceSize(device string) uint64 {
	base := filepath.Base(alpnasCanonicalDevice(device))
	if base == "" {
		return 0
	}
	raw, err := os.ReadFile(filepath.Join("/sys/class/block", base, "size"))
	if err == nil {
		sectors, parseErr := strconv.ParseUint(strings.TrimSpace(string(raw)), 10, 64)
		if parseErr == nil {
			return sectors * 512
		}
	}

	for _, record := range alpnasCollectFdiskRecords(nil) {
		if alpnasCanonicalDevice(record.Device) == alpnasCanonicalDevice(device) && record.Size > 0 {
			return record.Size
		}
	}
	return 0
}

func alpnasInstallerDiskIsBootMedium(device string, uuid string, partUUID string, label string, partLabel string) bool {
	if alpnasIsBootMedium(device, uuid, partUUID, label, partLabel) {
		return true
	}
	for _, part := range alpnasInstallerPartitionDevices(device) {
		partLabelValue := strings.TrimSpace(alpnasReadBlkidValue(part, "LABEL"))
		partPartLabel := strings.TrimSpace(alpnasReadBlkidValue(part, "PARTLABEL"))
		partUUIDValue := strings.TrimSpace(alpnasReadBlkidValue(part, "UUID"))
		partPartUUID := strings.TrimSpace(alpnasReadBlkidValue(part, "PARTUUID"))
		if alpnasIsBootMedium(part, partUUIDValue, partPartUUID, partLabelValue, partPartLabel) {
			return true
		}
	}
	return false
}

func alpnasInstallerDeviceName(device string) string {
	model := strings.TrimSpace(alpnasInstallerReadSysfs(device, "device/model"))
	if model != "" {
		return model
	}
	base := filepath.Base(strings.TrimSpace(device))
	if base != "" {
		return base
	}
	return device
}

func alpnasInstallerReadSysfs(device string, rel string) string {
	base := filepath.Base(alpnasCanonicalDevice(device))
	if base == "" {
		return ""
	}
	raw, err := os.ReadFile("/sys/class/block/" + base + "/" + rel)
	if err != nil {
		return ""
	}
	return strings.TrimSpace(string(raw))
}

func alpnasInstallerSysfsBool(device string, rel string) bool {
	switch strings.TrimSpace(alpnasInstallerReadSysfs(device, rel)) {
	case "1", "y", "Y", "yes", "true", "on":
		return true
	default:
		return false
	}
}

func alpnasInstallerTransport(device string) string {
	if value := strings.TrimSpace(alpnasInstallerReadSysfs(device, "device/transport")); value != "" {
		return value
	}
	base := filepath.Base(alpnasCanonicalDevice(device))
	switch {
	case strings.HasPrefix(base, "nvme"):
		return "nvme"
	case strings.HasPrefix(base, "sd"):
		if link, err := filepath.EvalSymlinks("/sys/class/block/" + base); err == nil {
			lower := strings.ToLower(link)
			switch {
			case strings.Contains(lower, "/usb"):
				return "usb"
			case strings.Contains(lower, "/ata"):
				return "ata"
			case strings.Contains(lower, "/sas"):
				return "sas"
			}
		}
	}
	return ""
}

func alpnasInstallerIsHotplug(device string) bool {
	if alpnasInstallerSysfsBool(device, "removable") {
		return true
	}
	switch alpnasInstallerTransport(device) {
	case "usb", "firewire":
		return true
	default:
		return false
	}
}

func alpnasInstallerPartitionDevices(device string) []string {
	device = alpnasCanonicalDevice(device)
	base := filepath.Base(device)
	sysdir := filepath.Join("/sys/class/block", base)
	entries, err := os.ReadDir(sysdir)
	if err != nil {
		return []string{}
	}

	results := []string{}
	for _, entry := range entries {
		name := entry.Name()
		if name == base {
			continue
		}
		if strings.HasPrefix(name, base+"p") || strings.HasPrefix(name, base) {
			part := "/dev/" + name
			if _, err := os.Stat(part); err == nil {
				results = append(results, alpnasCanonicalDevice(part))
			}
		}
	}
	sort.Strings(results)
	return results
}

func alpnasInstallerPartitionCount(device string) int {
	return len(alpnasInstallerPartitionDevices(device))
}

func alpnasInstallerMountedChildren(device string) int {
	count := 0
	for _, part := range alpnasInstallerPartitionDevices(device) {
		if mountpoint, _ := alpnasLookupMountpoint(part); mountpoint != "" {
			count++
		}
	}
	return count
}

func alpnasInstallerFilesystemHints(device string) []string {
	results := []string{}
	for _, part := range alpnasInstallerPartitionDevices(device) {
		fsType := strings.ToLower(strings.TrimSpace(alpnasReadBlkidValue(part, "TYPE")))
		if fsType == "" || utils.StringInArray(results, fsType) {
			continue
		}
		results = append(results, fsType)
	}
	sort.Strings(results)
	return results
}

func alpnasReadInstallerState() map[string]string {
	state := map[string]string{}
	raw, err := os.ReadFile(alpnasInstallerStateFile)
	if err != nil {
		return state
	}

	scanner := bufio.NewScanner(strings.NewReader(string(raw)))
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		if line == "" || strings.HasPrefix(line, "#") {
			continue
		}
		key, value, ok := strings.Cut(line, "=")
		if !ok {
			continue
		}
		state[strings.TrimSpace(key)] = strings.TrimSpace(value)
	}

	return state
}

func alpnasTailLog(path string, maxLines int) []string {
	raw, err := os.ReadFile(path)
	if err != nil {
		return []string{}
	}

	lines := []string{}
	scanner := bufio.NewScanner(strings.NewReader(string(raw)))
	for scanner.Scan() {
		text := strings.TrimSpace(scanner.Text())
		if text == "" {
			continue
		}
		lines = append(lines, text)
	}

	if len(lines) > maxLines {
		lines = lines[len(lines)-maxLines:]
	}
	return lines
}

func alpnasFormatInstallerBytes(bytes uint64) string {
	if bytes == 0 {
		return "0 B"
	}

	units := []string{"B", "KB", "MB", "GB", "TB", "PB"}
	size := float64(bytes)
	index := 0
	for size >= 1024 && index < len(units)-1 {
		size /= 1024
		index++
	}

	precision := 2
	if size >= 100 {
		precision = 0
	} else if size >= 10 {
		precision = 1
	}

	return strconv.FormatFloat(size, 'f', precision, 64) + " " + units[index]
}
