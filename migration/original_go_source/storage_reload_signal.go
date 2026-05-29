package main

import (
	"sync"

	storagefs "imuslab.com/arozos/mod/storage"
)

var storageReloadMu sync.Mutex

// ReloadAllStoragePools refreshes the base pool and all permission-group pools.
// This is used by the ALPNAS integration to hot-reload drive discovery without
// a full process restart.
func ReloadAllStoragePools(reason string) {
	storageReloadMu.Lock()
	defer storageReloadMu.Unlock()

	if systemWideLogger == nil {
		return
	}
	systemWideLogger.PrintAndLog("Storage", "Reloading all storage pools ("+reason+")", nil)

	if baseStoragePool != nil {
		baseStoragePool.Close()
	}
	emptyPool := storagefs.StoragePool{}
	baseStoragePool = &emptyPool

	if err := LoadBaseStoragePool(); err != nil {
		systemWideLogger.PrintAndLog("Storage", err.Error(), err)
	} else if userHandler != nil {
		userHandler.UpdateStoragePool(baseStoragePool)
	}

	if permissionHandler == nil {
		return
	}
	for _, pg := range permissionHandler.PermissionGroups {
		systemWideLogger.PrintAndLog("Storage", "Reloading Storage Pool for: "+pg.Name, nil)
		if pg.StoragePool != nil {
			pg.StoragePool.Close()
		}
		newEmptyPool := storagefs.StoragePool{}
		pg.StoragePool = &newEmptyPool
		LoadStoragePoolForGroup(pg)
	}

	BridgeStoragePoolInit()
}
