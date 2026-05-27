package agi

import (
	"context"
	"errors"
	"log"
	"os/exec"
	"strings"
	"time"

	"github.com/robertkrimen/otto"
	"imuslab.com/arozos/mod/agi/static"
)

func (g *Gateway) FirewallCtlLibRegister() {
	err := g.RegisterLib("firewallctl", g.injectFirewallCtlFunctions)
	if err != nil {
		log.Fatal(err)
	}
}

func (g *Gateway) injectFirewallCtlFunctions(payload *static.AgiLibInjectionPayload) {
	vm := payload.VM
	u := payload.User

	run := func(timeout time.Duration, args ...string) otto.Value {
		if u == nil || !u.IsAdmin() {
			return jsonValue(vm, map[string]any{
				"ok":    false,
				"error": "admin_required",
			})
		}

		output, err := runFirewallCtl(timeout, args...)
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

	vm.Set("_firewallctl_templates", func(call otto.FunctionCall) otto.Value {
		return run(5*time.Second, "templates", "--json")
	})
	vm.Set("_firewallctl_status", func(call otto.FunctionCall) otto.Value {
		return run(5*time.Second, "status", "--json")
	})
	vm.Set("_firewallctl_apply", func(call otto.FunctionCall) otto.Value {
		templateID, _ := call.Argument(0).ToString()
		return run(15*time.Second, "apply", strings.TrimSpace(templateID), "--json")
	})
	vm.Set("_firewallctl_enable_port", func(call otto.FunctionCall) otto.Value {
		proto, _ := call.Argument(0).ToString()
		port, _ := call.Argument(1).ToString()
		return run(15*time.Second, "enable-port", strings.TrimSpace(proto), strings.TrimSpace(port), "--json")
	})
	vm.Set("_firewallctl_disable_port", func(call otto.FunctionCall) otto.Value {
		proto, _ := call.Argument(0).ToString()
		port, _ := call.Argument(1).ToString()
		return run(15*time.Second, "disable-port", strings.TrimSpace(proto), strings.TrimSpace(port), "--json")
	})

	vm.Run(`
		firewallctl = {
			templates: function() {
				return JSON.parse(_firewallctl_templates());
			},
			status: function() {
				return JSON.parse(_firewallctl_status());
			},
			apply: function(templateID) {
				return JSON.parse(_firewallctl_apply(templateID));
			},
			enablePort: function(proto, port) {
				return JSON.parse(_firewallctl_enable_port(proto, port));
			},
			disablePort: function(proto, port) {
				return JSON.parse(_firewallctl_disable_port(proto, port));
			}
		};
	`)
}

func runFirewallCtl(timeout time.Duration, args ...string) (string, error) {
	ctx, cancel := context.WithTimeout(context.Background(), timeout)
	defer cancel()

	cmd := exec.CommandContext(ctx, "/usr/local/bin/alpnas-firewall", args...)
	raw, err := cmd.CombinedOutput()
	output := strings.TrimSpace(string(raw))

	if errors.Is(ctx.Err(), context.DeadlineExceeded) {
		return "", errors.New("firewallctl_timeout")
	}

	if err != nil {
		if output != "" {
			return "", errors.New(output)
		}
		return "", err
	}

	return output, nil
}
