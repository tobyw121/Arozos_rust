package diskfs

import (
	"bufio"
	"bytes"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"sort"
	"strconv"
	"strings"
	"unicode"
)

type blkidMeta struct {
	Fstype string
	Label  string
	UUID   string
}

type dfStat struct {
	Available int64
	Use       string
}

func listStorageDevicesFromFdisk() ([]BlockDeviceMeta, error) {
	deviceNames, err := fdiskEnumerateDeviceNames()
	if err != nil {
		return nil, err
	}

	mounts := readMounts()
	dfStats := readDFStats()
	blkidCache := map[string]blkidMeta{}
	devices := map[string]*BlockDeviceMeta{}
	order := []string{}

	for _, name := range deviceNames {
		meta, err := buildStorageDeviceMeta(name, mounts, dfStats, blkidCache)
		if err != nil {
			continue
		}

		if meta.Type == "part" {
			parentName := rootDeviceName(meta.Name)
			if parentName == "" || parentName == meta.Name {
				continue
			}
			parent, ok := devices[parentName]
			if !ok {
				parent, err = buildStorageDeviceMeta(parentName, mounts, dfStats, blkidCache)
				if err != nil || parent.Type == "part" {
					continue
				}
				devices[parentName] = parent
				order = append(order, parentName)
			}
			parent.Children = append(parent.Children, partitionMetaFromBlockDevice(*meta))
			continue
		}

		if _, ok := devices[meta.Name]; !ok {
			order = append(order, meta.Name)
		}
		devices[meta.Name] = meta
	}

	results := make([]BlockDeviceMeta, 0, len(order))
	for _, name := range order {
		device := devices[name]
		if device == nil {
			continue
		}
		sort.Slice(device.Children, func(i, j int) bool {
			return device.Children[i].Name < device.Children[j].Name
		})
		results = append(results, *device)
	}

	return results, nil
}

func fdiskEnumerateDeviceNames() ([]string, error) {
	cmd := exec.Command(fdiskBinary(), "-l")
	output, err := cmd.CombinedOutput()
	if err != nil {
		return nil, fmt.Errorf("fdisk error: %v: %s", err, strings.TrimSpace(string(output)))
	}

	results := []string{}
	seen := map[string]bool{}
	scanner := bufio.NewScanner(strings.NewReader(string(output)))
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		if line == "" {
			continue
		}

		if strings.HasPrefix(line, "Disk /dev/") {
			fields := strings.Fields(line)
			if len(fields) < 2 {
				continue
			}
			name := filepath.Base(strings.TrimSuffix(fields[1], ":"))
			if shouldIgnoreStorageDevice(name) || seen[name] {
				continue
			}
			seen[name] = true
			results = append(results, name)
			continue
		}

		if !strings.HasPrefix(line, "/dev/") {
			continue
		}

		fields := strings.Fields(line)
		if len(fields) == 0 {
			continue
		}
		name := filepath.Base(strings.TrimSpace(fields[0]))
		if shouldIgnoreStorageDevice(name) || seen[name] {
			continue
		}
		seen[name] = true
		results = append(results, name)
	}

	return results, nil
}

func buildStorageDeviceMeta(name string, mounts map[string]string, dfStats map[string]dfStat, blkidCache map[string]blkidMeta) (*BlockDeviceMeta, error) {
	name = filepath.Base(strings.TrimSpace(name))
	if name == "" || shouldIgnoreStorageDevice(name) {
		return nil, fmt.Errorf("unsupported device: %s", name)
	}

	sysdir := filepath.Join("/sys/class/block", name)
	if _, err := os.Stat(sysdir); err != nil {
		return nil, err
	}

	devicePath := filepath.Join("/dev", name)
	size := readDeviceSize(sysdir)
	mountpoint := findDeviceMountpoint(devicePath, mounts)
	fsInfo := readBlkidMeta(devicePath, blkidCache)
	stats := dfStats[mountpoint]

	meta := &BlockDeviceMeta{
		Name:       name,
		MajMin:     readTrimmedFile(filepath.Join(sysdir, "dev")),
		Rm:         readBoolFile(filepath.Join(sysdir, "removable")),
		Size:       size,
		Ro:         readBoolFile(filepath.Join(sysdir, "ro")),
		Type:       detectDeviceType(name, sysdir),
		Mountpoint: mountpoint,
		Fstype:     fsInfo.Fstype,
		Label:      fsInfo.Label,
		UUID:       fsInfo.UUID,
		Fsavail:    stats.Available,
		Fsuse:      stats.Use,
		Model:      readDeviceModel(name),
	}

	return meta, nil
}

func partitionMetaFromBlockDevice(device BlockDeviceMeta) PartitionMeta {
	return PartitionMeta{
		Name:       device.Name,
		MajMin:     device.MajMin,
		Rm:         device.Rm,
		Size:       device.Size,
		Ro:         device.Ro,
		Type:       device.Type,
		Mountpoint: device.Mountpoint,
		Fstype:     device.Fstype,
		Label:      device.Label,
		UUID:       device.UUID,
		Fsavail:    device.Fsavail,
		Fsuse:      device.Fsuse,
	}
}

func fdiskBinary() string {
	candidates := []string{"/sbin/fdisk", "/usr/sbin/fdisk", "/usr/bin/fdisk", "/bin/fdisk"}
	for _, candidate := range candidates {
		if _, err := os.Stat(candidate); err == nil {
			return candidate
		}
	}
	return "fdisk"
}

func shouldIgnoreStorageDevice(name string) bool {
	switch {
	case name == "":
		return true
	case strings.HasPrefix(name, "loop"),
		strings.HasPrefix(name, "ram"),
		strings.HasPrefix(name, "zram"),
		strings.HasPrefix(name, "fd"),
		strings.Contains(name, "mmcblk") && (strings.Contains(name, "boot") || strings.Contains(name, "rpmb")):
		return true
	default:
		return false
	}
}

func detectDeviceType(name string, sysdir string) string {
	if readTrimmedFile(filepath.Join(sysdir, "partition")) != "" {
		return "part"
	}
	switch {
	case strings.HasPrefix(name, "sr"):
		return "rom"
	case strings.HasPrefix(name, "md"):
		return "raid"
	case strings.HasPrefix(name, "dm-"):
		return "lvm"
	default:
		return "disk"
	}
}

func rootDeviceName(name string) string {
	name = filepath.Base(strings.TrimSpace(name))
	switch {
	case strings.HasPrefix(name, "nvme"), strings.HasPrefix(name, "mmcblk"), strings.HasPrefix(name, "md"), strings.HasPrefix(name, "dm-"):
		if idx := strings.LastIndex(name, "p"); idx > 0 && hasOnlyDigits(name[idx+1:]) {
			return name[:idx]
		}
	}

	end := len(name)
	for end > 0 && unicode.IsDigit(rune(name[end-1])) {
		end--
	}
	if end == len(name) {
		return name
	}
	return name[:end]
}

func hasOnlyDigits(input string) bool {
	if input == "" {
		return false
	}
	for _, r := range input {
		if !unicode.IsDigit(r) {
			return false
		}
	}
	return true
}

func readMounts() map[string]string {
	results := map[string]string{}
	raw, err := os.ReadFile("/proc/mounts")
	if err != nil {
		return results
	}

	for _, line := range strings.Split(string(raw), "\n") {
		fields := strings.Fields(line)
		if len(fields) < 2 {
			continue
		}
		source := canonicalDevice(fields[0])
		if !strings.HasPrefix(source, "/dev/") {
			continue
		}
		results[source] = strings.ReplaceAll(fields[1], `\040`, " ")
	}

	return results
}

func readDFStats() map[string]dfStat {
	results := map[string]dfStat{}
	cmd := exec.Command("df", "-B1", "-P")
	output, err := cmd.Output()
	if err != nil {
		return results
	}

	scanner := bufio.NewScanner(bytes.NewReader(output))
	firstLine := true
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		if line == "" {
			continue
		}
		if firstLine {
			firstLine = false
			continue
		}

		fields := strings.Fields(line)
		if len(fields) < 6 {
			continue
		}

		available, _ := strconv.ParseInt(fields[3], 10, 64)
		mountpoint := strings.Join(fields[5:], " ")
		results[mountpoint] = dfStat{
			Available: available,
			Use:       fields[4],
		}
	}

	return results
}

func readBlkidMeta(devicePath string, cache map[string]blkidMeta) blkidMeta {
	devicePath = canonicalDevice(devicePath)
	if cache != nil {
		if cached, ok := cache[devicePath]; ok {
			return cached
		}
	}

	meta := blkidMeta{}
	cmd := exec.Command("blkid", "-c", "/dev/null", devicePath)
	output, err := cmd.Output()
	if err == nil {
		line := strings.TrimSpace(string(output))
		meta = blkidMeta{
			Fstype: parseBlkidField(line, "TYPE"),
			Label:  parseBlkidField(line, "LABEL"),
			UUID:   parseBlkidField(line, "UUID"),
		}
	}

	if cache != nil {
		cache[devicePath] = meta
	}
	return meta
}

func parseBlkidField(line string, key string) string {
	prefix := key + "=\""
	idx := strings.Index(line, prefix)
	if idx < 0 {
		return ""
	}
	rest := line[idx+len(prefix):]
	end := strings.Index(rest, "\"")
	if end < 0 {
		return ""
	}
	return rest[:end]
}

func findDeviceMountpoint(devicePath string, mounts map[string]string) string {
	if len(mounts) == 0 {
		return ""
	}
	return mounts[canonicalDevice(devicePath)]
}

func canonicalDevice(devicePath string) string {
	devicePath = strings.TrimSpace(devicePath)
	if devicePath == "" {
		return ""
	}
	if !strings.HasPrefix(devicePath, "/dev/") {
		devicePath = filepath.Join("/dev", filepath.Base(devicePath))
	}
	if resolved, err := filepath.EvalSymlinks(devicePath); err == nil && resolved != "" {
		return resolved
	}
	return devicePath
}

func readTrimmedFile(filename string) string {
	raw, err := os.ReadFile(filename)
	if err != nil {
		return ""
	}
	return strings.TrimSpace(string(raw))
}

func readBoolFile(filename string) bool {
	switch readTrimmedFile(filename) {
	case "1", "y", "Y", "true", "TRUE", "yes", "YES":
		return true
	default:
		return false
	}
}

func readDeviceSize(sysdir string) int64 {
	sectors, err := strconv.ParseInt(readTrimmedFile(filepath.Join(sysdir, "size")), 10, 64)
	if err != nil || sectors < 0 {
		return 0
	}
	return sectors * 512
}

func readDeviceModel(name string) string {
	sysdir := filepath.Join("/sys/class/block", name)
	model := readTrimmedFile(filepath.Join(sysdir, "device/model"))
	if model != "" {
		return model
	}

	if dmName := readTrimmedFile(filepath.Join(sysdir, "dm/name")); dmName != "" {
		return dmName
	}

	return readTrimmedFile(filepath.Join(sysdir, "device/vendor"))
}

func formatByteCountIEC(size int64) string {
	if size < 1024 {
		return fmt.Sprintf("%d B", size)
	}

	units := []string{"KiB", "MiB", "GiB", "TiB", "PiB"}
	value := float64(size)
	unitIndex := -1
	for value >= 1024 && unitIndex < len(units)-1 {
		value /= 1024
		unitIndex++
	}
	return fmt.Sprintf("%.1f %s", value, units[unitIndex])
}
