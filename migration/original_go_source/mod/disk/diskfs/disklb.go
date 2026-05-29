package diskfs

import "fmt"

// Get disk model name by disk name (sdX, not /dev/sdX), return the model name (if any) and expected size (not actual)
// return device labeled size, model and error if any
func GetDiskModelByName(name string) (string, string, error) {
	storageMeta, err := ListAllStorageDevices()
	if err != nil {
		return "", "", err
	}

	for _, device := range storageMeta.Blockdevices {
		if device.Name == name {
			return formatByteCountIEC(device.Size), device.Model, nil
		}
		for _, child := range device.Children {
			if child.Name == name {
				return formatByteCountIEC(child.Size), device.Model, nil
			}
		}
	}
	return "", "", fmt.Errorf("disk not found: %s", name)
}
