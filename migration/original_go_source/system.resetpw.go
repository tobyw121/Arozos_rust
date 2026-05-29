package main

import (
	"errors"
	"fmt"
	"html"
	"log"
	"net/http"
	"os"
	"path/filepath"
	"strings"

	auth "imuslab.com/arozos/mod/auth"
	fs "imuslab.com/arozos/mod/filesystem"
	"imuslab.com/arozos/mod/utils"
)

/*
	Password Reset Module

	This module exists to serve the password restart page with security check
*/

func system_resetpw_init() {
	http.HandleFunc("/system/reset/validateResetKey", system_resetpw_validateResetKeyHandler)
	http.HandleFunc("/system/reset/confirmPasswordReset", system_resetpw_confirmReset)
	http.HandleFunc("/reset.system", system_resetpw_alpnasResetSystem)
}

// Validate if the ysername and rkey is valid
func system_resetpw_validateResetKeyHandler(w http.ResponseWriter, r *http.Request) {
	username, err := utils.PostPara(r, "username")
	if err != nil {
		utils.SendErrorResponse(w, "Invalid username or key")
		return
	}
	rkey, err := utils.PostPara(r, "rkey")
	if err != nil {
		utils.SendErrorResponse(w, "Invalid username or key")
		return
	}

	if username == "" || rkey == "" {
		utils.SendErrorResponse(w, "Invalid username or rkey")
		return
	}

	//Check if the pair is valid
	err = system_resetpw_validateResetKey(username, rkey)
	if err != nil {
		utils.SendErrorResponse(w, err.Error())
		return
	}

	utils.SendOK(w)

}

func system_resetpw_confirmReset(w http.ResponseWriter, r *http.Request) {
	username, _ := utils.PostPara(r, "username")
	rkey, _ := utils.PostPara(r, "rkey")
	newpw, _ := utils.PostPara(r, "pw")
	if username == "" || rkey == "" || newpw == "" {
		utils.SendErrorResponse(w, "Internal Server Error")
		return
	}

	//Check user exists
	if !authAgent.UserExists(username) {
		utils.SendErrorResponse(w, "Username not exists")
		return
	}

	//Validate rkey
	err := system_resetpw_validateResetKey(username, rkey)
	if err != nil {
		utils.SendErrorResponse(w, err.Error())
		return
	}

	//OK to procced
	newHashedPassword := auth.Hash(newpw)
	err = sysdb.Write("auth", "passhash/"+username, newHashedPassword)
	if err != nil {
		utils.SendErrorResponse(w, err.Error())
		return
	}

	utils.SendOK(w)

}

func system_resetpw_validateResetKey(username string, key string) error {
	//Get current password from db
	passwordInDB := ""
	err := sysdb.Read("auth", "passhash/"+username, &passwordInDB)
	if err != nil {
		return err
	}

	//Get hashed user key
	hashedKey := auth.Hash(key)
	if passwordInDB != hashedKey {
		return errors.New("Invalid Password Reset Key")
	}

	return nil
}

func system_resetpw_handlePasswordReset(w http.ResponseWriter, r *http.Request) {
	//Check if the user click on this link with reset password key string. If not, ask the user to input one
	acc, err := utils.GetPara(r, "acc")
	if err != nil || acc == "" {
		system_resetpw_serveIdEnterInterface(w, r)
		return
	}

	resetkey, err := utils.GetPara(r, "rkey")
	if err != nil || resetkey == "" {
		system_resetpw_serveIdEnterInterface(w, r)
		return
	}

	//Check if the code is valid
	err = system_resetpw_validateResetKey(acc, resetkey)
	if err != nil {
		utils.SendErrorResponse(w, "Invalid username or resetKey")
		return
	}

	//OK. Create the New Password Entering UI
	vendorIconSrc := filepath.Join(vendorResRoot, "vendor_icon.png")
	if !fs.FileExists(vendorIconSrc) {
		vendorIconSrc = "./web/img/public/vendor_icon.png"
	}
	imageBase64, _ := utils.LoadImageAsBase64(vendorIconSrc)
	template, err := utils.Templateload("system/reset/resetPasswordTemplate.html", map[string]string{
		"vendor_logo": imageBase64,
		"host_name":   *host_name,
		"username":    acc,
		"rkey":        resetkey,
	})
	if err != nil {
		log.Fatal(err)
	}
	w.Header().Set("Content-Type", "text/html; charset=UTF-8")
	w.Write([]byte(template))
}

func system_resetpw_serveIdEnterInterface(w http.ResponseWriter, r *http.Request) {
	//Reset Key or Username not found, Serve entering interface
	imgsrc := filepath.Join(vendorResRoot, "vendor_icon.png")
	if !fs.FileExists(imgsrc) {
		imgsrc = "./web/img/public/vendor_icon.png"
	}
	imageBase64, _ := utils.LoadImageAsBase64(imgsrc)
	template, err := utils.Templateload("system/reset/resetCodeTemplate.html", map[string]string{
		"vendor_logo": imageBase64,
		"host_name":   *host_name,
	})
	if err != nil {
		log.Fatal(err)
	}
	w.Header().Set("Content-Type", "text/html; charset=UTF-8")
	w.Write([]byte(template))
}

// system_resetpw_alpnasResetSystem is an AlpNAS recovery endpoint.
// It is intentionally separate from ArozOS' legacy reset.html flow because the
// NAS must remain recoverable when the user database or browser session becomes
// inconsistent. The endpoint is unauthenticated but requires a boot-generated
// one-time console token stored outside the ArozOS data directory.
func system_resetpw_alpnasResetSystem(w http.ResponseWriter, r *http.Request) {
	const resetTokenPath = "/run/alpnas/auth/reset.token"

	tokenBytes, err := os.ReadFile(resetTokenPath)
	if err != nil {
		http.Error(w, "AlpNAS reset token is not available. Run 'alpnas-auth-reset-token show' on the console.", http.StatusServiceUnavailable)
		return
	}
	expectedToken := strings.TrimSpace(string(tokenBytes))
	if expectedToken == "" {
		http.Error(w, "AlpNAS reset token is empty. Run 'alpnas-auth-reset-token rotate' on the console.", http.StatusServiceUnavailable)
		return
	}

	if r.Method == http.MethodPost {
		_ = r.ParseForm()
		suppliedToken := strings.TrimSpace(r.FormValue("token"))
		username := strings.TrimSpace(r.FormValue("username"))
		newPassword := r.FormValue("password")
		confirmPassword := r.FormValue("confirm")

		if suppliedToken == "" || suppliedToken != expectedToken {
			system_resetpw_renderAlpnasResetForm(w, suppliedToken, username, "Ungültiger Reset-Token.")
			return
		}
		if username == "" || !authAgent.UserExists(username) {
			system_resetpw_renderAlpnasResetForm(w, suppliedToken, username, "Benutzer existiert nicht.")
			return
		}
		if len(newPassword) < 8 {
			system_resetpw_renderAlpnasResetForm(w, suppliedToken, username, "Das neue Passwort muss mindestens 8 Zeichen haben.")
			return
		}
		if newPassword != confirmPassword {
			system_resetpw_renderAlpnasResetForm(w, suppliedToken, username, "Die Passwörter stimmen nicht überein.")
			return
		}

		newHashedPassword := auth.Hash(newPassword)
		if err := sysdb.Write("auth", "passhash/"+username, newHashedPassword); err != nil {
			system_resetpw_renderAlpnasResetForm(w, suppliedToken, username, err.Error())
			return
		}
		authAgent.RemoveAutologinTokenByUsername(username)
		_ = os.Remove(resetTokenPath)

		w.Header().Set("Content-Type", "text/html; charset=UTF-8")
		fmt.Fprintf(w, `<!doctype html><html><head><meta charset="utf-8"><title>AlpNAS Passwort zurückgesetzt</title>
<style>body{font-family:system-ui,sans-serif;margin:3rem;background:#f6f8fb;color:#17212b}.card{max-width:640px;background:#fff;border:1px solid #dbe3ee;border-radius:14px;padding:2rem;box-shadow:0 10px 30px rgba(0,0,0,.08)}a{color:#0b76d1}</style></head><body><div class="card"><h1>Passwort zurückgesetzt</h1><p>Das Passwort für <b>%s</b> wurde geändert. Der Reset-Token wurde ungültig gemacht.</p><p><a href="/login.html">Zur Anmeldung</a></p></div></body></html>`, html.EscapeString(username))
		return
	}

	queryToken := strings.TrimSpace(r.URL.Query().Get("token"))
	username := ""
	users := authAgent.ListUsers()
	if len(users) == 1 {
		username = users[0]
	}
	system_resetpw_renderAlpnasResetForm(w, queryToken, username, "")
}

func system_resetpw_renderAlpnasResetForm(w http.ResponseWriter, token string, username string, message string) {
	w.Header().Set("Content-Type", "text/html; charset=UTF-8")
	escapedToken := html.EscapeString(token)
	escapedUser := html.EscapeString(username)
	escapedMessage := html.EscapeString(message)
	msgBlock := ""
	if escapedMessage != "" {
		msgBlock = `<div class="msg">` + escapedMessage + `</div>`
	}
	fmt.Fprintf(w, `<!doctype html><html><head><meta charset="utf-8"><title>AlpNAS Passwort Reset</title>
<meta name="viewport" content="width=device-width, initial-scale=1">
<style>
body{font-family:system-ui,-apple-system,BlinkMacSystemFont,"Segoe UI",sans-serif;margin:0;background:#eef3f8;color:#142232;display:flex;min-height:100vh;align-items:center;justify-content:center}.card{width:min(620px,calc(100vw - 32px));background:#fff;border:1px solid #d9e3ee;border-radius:18px;padding:28px;box-shadow:0 18px 50px rgba(15,35,60,.12)}h1{margin:0 0 8px}.hint{color:#5f6f82;margin:0 0 22px}.row{margin:14px 0}label{display:block;font-weight:700;margin-bottom:6px}input{box-sizing:border-box;width:100%%;border:1px solid #cbd7e3;border-radius:10px;padding:12px;font-size:16px}button{border:0;background:#0f766e;color:#fff;border-radius:10px;padding:12px 18px;font-weight:700;font-size:16px;cursor:pointer}.msg{background:#fff4e5;border:1px solid #ffd19b;color:#7a3d00;border-radius:10px;padding:12px;margin:0 0 18px}.danger{background:#fff1f2;border:1px solid #fecdd3;color:#881337;border-radius:10px;padding:12px;margin-top:18px;font-size:14px}code{background:#edf2f7;padding:2px 5px;border-radius:5px}</style></head><body><form class="card" method="post" action="/reset.system"><h1>AlpNAS Passwort Reset</h1><p class="hint">Token auf der AlpNAS-Konsole mit <code>alpnas-auth-reset-token show</code> anzeigen. Nach erfolgreichem Reset wird der Token gelöscht.</p>%s<div class="row"><label>Reset-Token</label><input name="token" value="%s" autocomplete="off" required></div><div class="row"><label>Benutzername</label><input name="username" value="%s" autocomplete="username" required></div><div class="row"><label>Neues Passwort</label><input type="password" name="password" autocomplete="new-password" required minlength="8"></div><div class="row"><label>Neues Passwort bestätigen</label><input type="password" name="confirm" autocomplete="new-password" required minlength="8"></div><button type="submit">Passwort zurücksetzen</button><div class="danger">Diese Seite funktioniert nur mit dem lokalen Boot-Token. Sie ist für Notfall-Wiederherstellung gedacht.</div></form></body></html>`, msgBlock, escapedToken, escapedUser)
}
