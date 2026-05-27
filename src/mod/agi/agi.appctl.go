package agi

import (
	"context"
	"encoding/json"
	"errors"
	"log"
	"os/exec"
	"strings"
	"time"

	"github.com/robertkrimen/otto"
	"imuslab.com/arozos/mod/agi/static"
)

func (g *Gateway) AppCtlLibRegister() {
	err := g.RegisterLib("appctl", g.injectAppCtlFunctions)
	if err != nil {
		log.Fatal(err)
	}
}

func (g *Gateway) injectAppCtlFunctions(payload *static.AgiLibInjectionPayload) {
	vm := payload.VM
	u := payload.User

	run := func(timeout time.Duration, args ...string) otto.Value {
		if u == nil || !u.IsAdmin() {
			return jsonValue(vm, map[string]any{
				"ok":    false,
				"error": "admin_required",
			})
		}

		output, err := runAppCtl(timeout, args...)
		if err != nil {
			return jsonValue(vm, map[string]any{
				"ok":    false,
				"error": err.Error(),
			})
		}

		if strings.TrimSpace(output) == "" {
			output = `{"ok":true}`
		}

		value, convErr := vm.ToValue(output)
		if convErr != nil {
			return jsonValue(vm, map[string]any{
				"ok":    false,
				"error": convErr.Error(),
			})
		}

		return value
	}

	vm.Set("_appctl_catalog", func(call otto.FunctionCall) otto.Value {
		return run(5*time.Second, "catalog")
	})

	vm.Set("_appctl_status", func(call otto.FunctionCall) otto.Value {
		appID, _ := call.Argument(0).ToString()
		return run(5*time.Second, "status", strings.TrimSpace(appID))
	})

	vm.Set("_appctl_install", func(call otto.FunctionCall) otto.Value {
		appID, _ := call.Argument(0).ToString()
		return run(2*time.Minute, "install", strings.TrimSpace(appID))
	})

	vm.Set("_appctl_action", func(call otto.FunctionCall) otto.Value {
		appID, _ := call.Argument(0).ToString()
		action, _ := call.Argument(1).ToString()
		return run(2*time.Minute, "action", strings.TrimSpace(appID), strings.TrimSpace(action))
	})

	vm.Run(`
		appctl = {
			catalog: function() {
				return JSON.parse(_appctl_catalog());
			},
			status: function(app) {
				return JSON.parse(_appctl_status(app));
			},
			install: function(app) {
				return JSON.parse(_appctl_install(app));
			},
			action: function(app, action) {
				return JSON.parse(_appctl_action(app, action));
			}
		};
	`)
}

func runAppCtl(timeout time.Duration, args ...string) (string, error) {
	ctx, cancel := context.WithTimeout(context.Background(), timeout)
	defer cancel()

	cmd := exec.CommandContext(ctx, "/usr/local/bin/alpnas-appctl", args...)
	raw, err := cmd.CombinedOutput()
	output := extractAppCtlJSON(strings.TrimSpace(string(raw)))

	if errors.Is(ctx.Err(), context.DeadlineExceeded) {
		return "", errors.New("appctl_timeout")
	}

	if err != nil {
		if output != "" && json.Valid([]byte(output)) {
			return output, nil
		}
		if output != "" {
			return "", errors.New(output)
		}
		return "", err
	}

	if output == "" {
		return output, nil
	}
	if !json.Valid([]byte(output)) {
		body, _ := json.Marshal(map[string]any{
			"ok":     false,
			"error":  "invalid_appctl_output",
			"detail": output,
		})
		return string(body), nil
	}

	return output, nil
}

func extractAppCtlJSON(output string) string {
	output = strings.TrimSpace(output)
	if output == "" {
		return ""
	}
	if json.Valid([]byte(output)) {
		return output
	}

	// Some OpenRC helpers and daemon start attempts may write status text around
	// the appctl JSON. Prefer a complete JSON line from the end of the output.
	lines := strings.Split(output, "\n")
	for i := len(lines) - 1; i >= 0; i-- {
		candidate := strings.TrimSpace(lines[i])
		if candidate == "" {
			continue
		}
		if json.Valid([]byte(candidate)) {
			return candidate
		}
	}

	// Fall back to scanning for a valid JSON object embedded in noisy output.
	for start := strings.Index(output, "{"); start >= 0 && start < len(output); {
		for end := strings.LastIndex(output, "}"); end > start; end = strings.LastIndex(output[:end], "}") {
			candidate := strings.TrimSpace(output[start : end+1])
			if json.Valid([]byte(candidate)) {
				return candidate
			}
		}
		next := strings.Index(output[start+1:], "{")
		if next < 0 {
			break
		}
		start += next + 1
	}

	return output
}

func jsonValue(vm *otto.Otto, payload any) otto.Value {
	raw, _ := json.Marshal(payload)
	value, err := vm.ToValue(string(raw))
	if err != nil {
		return otto.FalseValue()
	}
	return value
}
