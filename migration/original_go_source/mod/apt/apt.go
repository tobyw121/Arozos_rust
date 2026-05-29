package apt

import (
	"encoding/json"
	"errors"
	"log"
	"net/http"
	"os"
	"os/exec"
	"runtime"
	"strings"
)

/*
	Pacakge management tool for Linux OS with APT

	ONLY USABLE under Linux environment
*/

type AptPackageManager struct {
	AllowAutoInstall bool
}

func NewPackageManager(autoInstall bool) *AptPackageManager {
	return &AptPackageManager{
		AllowAutoInstall: autoInstall,
	}
}

// Install the given package if not exists. Set mustComply to true for "panic on failed to install"
func (a *AptPackageManager) InstallIfNotExists(pkgname string, mustComply bool) error {
	pkgname = strings.ReplaceAll(pkgname, "&", "")
	pkgname = strings.ReplaceAll(pkgname, "|", "")

	if !a.AllowAutoInstall {
		return errors.New("package auto install is disabled")
	}

	installed, err := PackageExists(pkgname)
	if err != nil {
		log.Println(err.Error())
	}
	if installed {
		return nil
	}

	installCmd := []string{"apt-get", "install", "-y", pkgname}
	if runtime.GOOS == "linux" {
		if _, err := exec.LookPath("apk"); err == nil {
			installCmd = []string{"apk", "add", pkgname}
		} else if _, err := exec.LookPath("apt-get"); err == nil {
			installCmd = []string{"apt-get", "install", "-y", pkgname}
		}
	}

	log.Println("Installing package " + pkgname + "...")
	cmd := exec.Command(installCmd[0], installCmd[1:]...)
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	err = cmd.Run()
	if err != nil {
		log.Println("Installation failed on package: " + pkgname)
		if mustComply {
			os.Exit(1)
		}
		return err
	}
	return nil
}

func PackageExists(pkgname string) (bool, error) {
	if runtime.GOOS == "windows" {
		cmd := exec.Command("where.exe", pkgname, "2>", "nul")
		_, err := cmd.CombinedOutput()
		if err != nil {
			return false, errors.New("Package " + pkgname + " not found in Windows %PATH%.")
		}
		return true, nil
	} else if runtime.GOOS == "darwin" {
		cmd := exec.Command("whereis", pkgname)
		out, err := cmd.CombinedOutput()
		if err != nil {
			return false, errors.New("Package " + pkgname + " not found in MacOS ENV variable.")
		}

		if strings.TrimSpace(string(out)) == "" {
			cmd := exec.Command("bash", "-c", "brew list | grep "+pkgname)
			out, err = cmd.CombinedOutput()
			if err != nil {
				return false, errors.New("Package " + pkgname + " not found in MacOS ENV variable.")
			}
		}
		if strings.TrimSpace(string(out)) != "" {
			return true, nil
		}
		return false, errors.New("Package " + pkgname + " not installed on this Mac")
	} else if runtime.GOOS == "linux" {
		cmd := exec.Command("which", pkgname)
		out, _ := cmd.CombinedOutput()
		if len(string(out)) > 1 {
			return true, nil
		}
		return false, errors.New("Package " + pkgname + " not installed on this Linux Host")
	}
	return false, errors.New("unsupported Platform")
}

func HandlePackageListRequest(w http.ResponseWriter, r *http.Request) {
	if runtime.GOOS == "windows" {
		w.Header().Set("Content-Type", "application/json")
		w.Write([]byte("{\"error\":\"Function disabled on Windows\"}"))
		return
	}

	listCmd := []string{"apt", "list", "--installed"}
	mode := "apt"
	if runtime.GOOS == "linux" {
		if _, err := exec.LookPath("apk"); err == nil {
			listCmd = []string{"apk", "info", "-vv"}
			mode = "apk"
		}
	}

	cmd := exec.Command(listCmd[0], listCmd[1:]...)
	out, err := cmd.CombinedOutput()
	if err != nil {
		w.Header().Set("Content-Type", "application/json")
		w.Write([]byte("{\"error\":\"" + err.Error() + "\"}"))
		return
	}

	results := [][]string{}
	for _, line := range strings.Split(string(out), "\n") {
		if line == "" {
			continue
		}

		if mode == "apk" {
			fields := strings.Fields(line)
			if len(fields) == 0 {
				continue
			}
			pkg := fields[0]
			version := "installed"
			if idx := strings.LastIndex(pkg, "-"); idx > 0 {
				version = pkg[idx+1:]
			}
			results = append(results, []string{pkg, version})
			continue
		}

		packageInfo := strings.Split(line, "/")
		packageName := packageInfo[0]
		if len(packageInfo) >= 2 {
			parts := strings.Split(packageInfo[1], ",")
			if len(parts) < 2 {
				continue
			}
			packageVersion := parts[1]
			if len(packageVersion) >= 4 && packageVersion[:3] == "now" {
				packageVersion = packageVersion[4:]
			}
			if strings.Contains(packageVersion, "[installed") && packageVersion[len(packageVersion)-1:] != "]" {
				packageVersion = packageVersion + ",automatic]"
			}
			results = append(results, []string{packageName, packageVersion})
		}
	}

	jsonString, _ := json.Marshal(results)
	w.Header().Set("Content-Type", "application/json")
	w.Write(jsonString)
}
