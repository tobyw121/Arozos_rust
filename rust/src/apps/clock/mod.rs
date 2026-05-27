//! Clock Application
//! 
//! Provides world clock, alarm, stopwatch, and timer functionality.

use axum::{Router, routing::get, response::Html, Json};
use serde::Serialize;
use chrono::{DateTime, Utc};

pub const APP_NAME: &str = "Clock";
pub const APP_VERSION: &str = "1.0.0";
pub const APP_DESCRIPTION: &str = "World clock, alarms, stopwatch, and timer";

#[derive(Debug, Serialize)]
pub struct ClockInfo {
    name: String,
    version: String,
    description: String,
    current_time: String,
}

/// Main clock interface
pub async fn index() -> Html<String> {
    let html = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Clock - arozOS</title>
    <meta charset="utf-8">
    <style>
        body { margin: 0; padding: 20px; font-family: Arial, sans-serif; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; min-height: 100vh; }
        .clock-container { max-width: 900px; margin: 0 auto; }
        .main-clock { text-align: center; padding: 40px; background: rgba(255,255,255,0.1); border-radius: 16px; backdrop-filter: blur(10px); }
        .time-display { font-size: 72px; font-weight: bold; text-shadow: 2px 2px 4px rgba(0,0,0,0.3); }
        .date-display { font-size: 24px; margin-top: 10px; opacity: 0.9; }
        .tabs { display: flex; gap: 10px; margin-top: 30px; }
        .tab { flex: 1; padding: 15px; background: rgba(255,255,255,0.2); border: none; border-radius: 8px; color: white; cursor: pointer; font-size: 16px; }
        .tab:hover { background: rgba(255,255,255,0.3); }
        .tab.active { background: white; color: #667eea; }
        .panel { display: none; margin-top: 30px; background: rgba(255,255,255,0.1); border-radius: 16px; padding: 20px; }
        .panel.active { display: block; }
        .world-city { display: flex; justify-content: space-between; padding: 15px; background: rgba(255,255,255,0.1); margin: 10px 0; border-radius: 8px; }
        .stopwatch-display { font-size: 48px; text-align: center; padding: 30px; }
        .controls { text-align: center; margin-top: 20px; }
        .btn { padding: 12px 24px; margin: 8px; font-size: 16px; border: none; border-radius: 6px; cursor: pointer; background: white; color: #667eea; }
        .btn:hover { opacity: 0.9; }
    </style>
</head>
<body>
    <div class="clock-container">
        <div class="main-clock">
            <div class="time-display" id="timeDisplay">00:00:00</div>
            <div class="date-display" id="dateDisplay">Loading...</div>
        </div>
        
        <div class="tabs">
            <button class="tab active" onclick="showPanel('world')">🌍 World Clock</button>
            <button class="tab" onclick="showPanel('alarm')">⏰ Alarm</button>
            <button class="tab" onclick="showPanel('stopwatch')">⏱️ Stopwatch</button>
            <button class="tab" onclick="showPanel('timer')">⏲️ Timer</button>
        </div>
        
        <div id="world" class="panel active">
            <h2>World Clock</h2>
            <div id="cities"></div>
        </div>
        
        <div id="alarm" class="panel">
            <h2>Alarms</h2>
            <p>Add and manage your alarms here</p>
        </div>
        
        <div id="stopwatch" class="panel">
            <div class="stopwatch-display" id="stopwatchDisplay">00:00:00</div>
            <div class="controls">
                <button class="btn" onclick="startStopwatch()">Start</button>
                <button class="btn" onclick="stopStopwatch()">Stop</button>
                <button class="btn" onclick="resetStopwatch()">Reset</button>
            </div>
        </div>
        
        <div id="timer" class="panel">
            <h2>Timer</h2>
            <p>Set a countdown timer</p>
        </div>
    </div>
    
    <script>
        function updateClock() {
            const now = new Date();
            document.getElementById('timeDisplay').textContent = now.toLocaleTimeString();
            document.getElementById('dateDisplay').textContent = now.toLocaleDateString(undefined, { weekday: 'long', year: 'numeric', month: 'long', day: 'numeric' });
        }
        
        function showPanel(panelId) {
            document.querySelectorAll('.panel').forEach(p => p.classList.remove('active'));
            document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
            document.getElementById(panelId).classList.add('active');
            event.target.classList.add('active');
        }
        
        // Stopwatch
        let stopwatchInterval = null;
        let stopwatchTime = 0;
        
        function startStopwatch() {
            if (stopwatchInterval) return;
            const startTime = Date.now() - stopwatchTime;
            stopwatchInterval = setInterval(() => {
                stopwatchTime = Date.now() - startTime;
                updateStopwatchDisplay();
            }, 10);
        }
        
        function stopStopwatch() {
            if (stopwatchInterval) {
                clearInterval(stopwatchInterval);
                stopwatchInterval = null;
            }
        }
        
        function resetStopwatch() {
            stopStopwatch();
            stopwatchTime = 0;
            updateStopwatchDisplay();
        }
        
        function updateStopwatchDisplay() {
            const totalSeconds = Math.floor(stopwatchTime / 1000);
            const hours = Math.floor(totalSeconds / 3600);
            const minutes = Math.floor((totalSeconds % 3600) / 60);
            const seconds = totalSeconds % 60;
            document.getElementById('stopwatchDisplay').textContent = 
                String(hours).padStart(2, '0') + ':' + String(minutes).padStart(2, '0') + ':' + String(seconds).padStart(2, '0');
        }
        
        // World cities
        const cities = [
            { name: 'New York', timezone: 'America/New_York' },
            { name: 'London', timezone: 'Europe/London' },
            { name: 'Tokyo', timezone: 'Asia/Tokyo' },
            { name: 'Sydney', timezone: 'Australia/Sydney' }
        ];
        
        function updateWorldClocks() {
            const container = document.getElementById('cities');
            container.innerHTML = cities.map(city => {
                const time = new Date().toLocaleTimeString('en-US', { timeZone: city.timezone });
                return `<div class="world-city"><span>${city.name}</span><span>${time}</span></div>`;
            }).join('');
        }
        
        setInterval(updateClock, 1000);
        setInterval(updateWorldClocks, 1000);
        updateClock();
        updateWorldClocks();
    </script>
</body>
</html>"#;
    Html(html.to_string())
}

/// Get clock info with current time
pub async fn api_info() -> Json<ClockInfo> {
    let now: DateTime<Utc> = Utc::now();
    Json(ClockInfo {
        name: APP_NAME.to_string(),
        version: APP_VERSION.to_string(),
        description: APP_DESCRIPTION.to_string(),
        current_time: now.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
    })
}

pub fn routes() -> Router<crate::AppState> {
    Router::new()
        .route("/", get(index))
        .route("/api/info", get(api_info))
}
