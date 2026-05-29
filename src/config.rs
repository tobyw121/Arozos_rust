use clap::Parser;
/// Runtime configuration translated from the original Go flag set.
#[derive(Debug, Clone, Parser)]
#[command(name = "arozos-rs", about = "Rust migration of ArozOS/AlpNAS") ]
pub struct Config {
    /// Root directory containing copied ArozOS web assets.
    #[arg(long = "web-root", default_value = "resources/web")]
    pub web_root: String,
    /// Root directory containing copied ArozOS system data/templates.
    #[arg(long = "system-root", default_value = "resources/system")]
    pub system_root: String,
    /// Listening host for HTTP server
    #[arg(long = "host", default_value = "")]
    pub listen_host: String,
    /// Listening port for HTTP server
    #[arg(long = "port", default_value_t = 8080)]
    pub listen_port: u16,
    /// Listening port for HTTPS server
    #[arg(long = "tls_port", alias = "https-port", default_value_t = 8443)]
    pub tls_listen_port: u16,
    /// Show system build version
    #[arg(long = "version", default_value_t = false)]
    pub show_version: bool,
    /// Default name for this host
    #[arg(long = "hostname", default_value = "AlpNAS")]
    pub host_name: String,
    /// System UUID for clustering and distributed computing. Only need to config once for first time startup. Leave empty for auto generation.
    #[arg(long = "uuid", default_value = "")]
    pub system_uuid: String,
    /// Disable subservices completely
    #[arg(long = "disable_subservice", default_value_t = false)]
    pub disable_subservices: bool,
    /// Enable uPNP service, recommended for host under NAT router
    #[arg(long = "allow_upnp", default_value_t = false)]
    pub allow_upnp: bool,
    /// Enable SSDP service, disable this if you do not want your device to be scanned by Windows's Network Neighborhood Page
    #[arg(long = "allow_ssdp", default_value_t = true)]
    pub allow_ssdp: bool,
    /// Enable MDNS service. Allow device to be scanned by nearby ArOZ Hosts
    #[arg(long = "allow_mdns", default_value_t = true)]
    pub allow_mdns: bool,
    /// Force MAC address to be used for discovery services. If not set, it will use the first NIC
    #[arg(long = "force_mac", default_value = "")]
    pub force_mac: String,
    /// Disable IP resolving if the system is running under reverse proxy environment
    #[arg(long = "disable_ip_resolver", default_value_t = false)]
    pub disable_ip_resolve_services: bool,
    /// Enable gzip compress on file server
    #[arg(long = "gzip", default_value_t = true)]
    pub enable_gzip: bool,
    /// Enable native HTTPS serving using Rust/rustls.
    #[arg(long = "tls", alias = "https", default_value_t = false)]
    pub use_tls: bool,
    /// Disable HTTP server, require tls=true
    #[arg(long = "disable_http", default_value_t = false)]
    pub disable_http: bool,
    /// TLS certificate file (.crt/.pem). Auto-created when auto_tls is enabled.
    #[arg(long = "cert", alias = "tls-cert", default_value = "resources/system/rust-port-data/tls/arozos-rust.crt")]
    pub tls_cert: String,
    /// TLS key file (.key/.pem). Auto-created when auto_tls is enabled.
    #[arg(long = "key", alias = "tls-key", default_value = "resources/system/rust-port-data/tls/arozos-rust.key")]
    pub tls_key: String,
    /// Automatically create a local self-signed HTTPS certificate if no certificate exists.
    #[arg(long = "auto_tls", alias = "auto-https-cert", default_value_t = false)]
    pub auto_tls: bool,
    /// Subject alternative names for the auto-created certificate. Repeat the flag for multiple names.
    #[arg(long = "cert-san", alias = "tls-san")]
    pub tls_subject_alt_names: Vec<String>,
    /// Redirect HTTP requests to the native HTTPS listener.
    #[arg(long = "redirect_http_to_https", alias = "redirect-http-to-https", default_value_t = false)]
    pub redirect_http_to_https: bool,
    /// Session key, must be 16, 24 or 32 bytes long (AES-128, AES-192 or AES-256). Leave empty for auto generated.
    #[arg(long = "session_key", default_value = "")]
    pub session_key: String,
    /// Enable hardware management functions in system
    #[arg(long = "enable_hwman", default_value_t = true)]
    pub allow_hardware_management: bool,
    /// Enable power management of the host system
    #[arg(long = "enable_pwman", default_value_t = true)]
    pub allow_power_management: bool,
    /// Path for the wpa_supplicant config
    #[arg(long = "wpa_supplicant_config", default_value = "/etc/wpa_supplicant/wpa_supplicant.conf")]
    pub wpa_supplicant_path: String,
    /// The default wireless interface for connecting to an AP
    #[arg(long = "wlan_interface_name", default_value = "wlan0")]
    pub wan_interface_name: String,
    /// Skip mdadm reload config during startup, might result in werid RAID device ID in some Linux distro
    #[arg(long = "skip_mdadm_reload", default_value_t = false)]
    pub skip_mdadm_reload: bool,
    /// Maxmium upload size in MB. Must not exceed the available ram on your system
    #[arg(long = "max_upload_size", default_value_t = 8192)]
    pub max_upload: usize,
    /// Upload buffer memory in MB. Any file larger than this size will be buffered to disk (slower).
    #[arg(long = "upload_buf", default_value_t = 25)]
    pub upload_buf: usize,
    /// File location of the storage config file
    #[arg(long = "storage_config", default_value = "./system/storage.json")]
    pub storage_config_file: String,
    /// Temporary storage, can be access via tmp:/. A tmp/ folder will be created in this path. Recommend fast storage devices like SSD
    #[arg(long = "tmp", default_value = "./")]
    pub tmp_directory: String,
    /// User root directories
    #[arg(long = "root", default_value = "./files/")]
    pub root_directory: String,
    /// Amount of buffer memory for IO operations
    #[arg(long = "iobuf", default_value_t = 1024)]
    pub file_opr_buff: usize,
    /// Enable directory listing
    #[arg(long = "dir_list", default_value_t = true)]
    pub enable_dir_listing: bool,
    /// Enable file upload buffering to run in async mode (Faster upload, require RAM >= 8GB)
    #[arg(long = "upload_async", default_value_t = false)]
    pub enable_asyncfileupload: bool,
    /// Maxmium buffer pool size (in MB) for buffer required file system abstractions
    #[arg(long = "buffpool_size", default_value_t = 1024)]
    pub bufferpoolsize: usize,
    /// Maxmium buffer file size (in MB) for buffer required file system abstractions
    #[arg(long = "bufffile_size", default_value_t = 25)]
    pub bufferfilemaxsize: usize,
    /// Enable buffer pool for buffer required file system abstractions
    #[arg(long = "enable_buffpool", default_value_t = true)]
    pub enable_buffering: bool,
    /// Allow compatibility to ArOZ Online Beta Clusters
    #[arg(long = "beta_scan", default_value_t = false)]
    pub enable_beta_scanning_support: bool,
    /// Enable the debugging console.
    #[arg(long = "console", default_value_t = false)]
    pub enable_console: bool,
    /// Enable logging to file for debug purpose
    #[arg(long = "logging", default_value_t = true)]
    pub enable_logging: bool,
    /// Enable public register interface for account creation
    #[arg(long = "public_reg", default_value_t = false)]
    pub allow_public_registry: bool,
    /// Allow RESTFUL login redirection that allow machines like billboards to login to the system on boot
    #[arg(long = "allow_autologin", default_value_t = true)]
    pub allow_autologin: bool,
    /// Allow the system to install package using Advanced Package Tool (aka apt or apt-get)
    #[arg(long = "allow_pkg_install", default_value_t = true)]
    pub allow_package_autoinstall: bool,
    /// Enable user homepage. Accessible via /www/{username}/
    #[arg(long = "homepage", default_value_t = true)]
    pub allow_homepage: bool,
    /// Nightly tasks execution time. Default 3 = 3 am in the morning
    #[arg(long = "ntt", default_value_t = 3)]
    pub nightlytaskruntime: usize,
    /// Time before tmp file will be deleted in seconds. Default 86400 seconds = 24 hours
    #[arg(long = "tmp_time", default_value_t = 86400)]
    pub maxtempfilekeeptime: usize,
    /// Enable cluster operations within LAN. Require allow_mdns=true flag
    #[arg(long = "allow_cluster", default_value_t = true)]
    pub allow_clustering: bool,
    /// Enable IoT related APIs and scanner. Require MDNS enabled
    #[arg(long = "allow_iot", default_value_t = true)]
    pub allow_iot: bool,
}

impl Config {
    pub fn bind_addr(&self) -> String { format!("{}:{}", self.listen_host, self.listen_port) }
}
