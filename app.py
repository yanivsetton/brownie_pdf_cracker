from flask import Flask, render_template_string
from flask_socketio import SocketIO
import subprocess
import threading
import re

app = Flask(__name__)
# Ensure you're using the proper async mode for your setup, 'threading', 'eventlet', or 'gevent'
socketio = SocketIO(app, async_mode='threading', cors_allowed_origins="*")

def clean_ansi_escape_codes(text):
    """Remove or replace ANSI escape codes from the text."""
    ansi_escape = re.compile(r'\x1B[@-_][0-?]*[ -/]*[@-~]')
    return ansi_escape.sub('', text)

def stream_logs_to_socket(container_name):
    """Stream Docker container logs to WebSocket."""
    process = subprocess.Popen(['docker', 'logs', '-f', container_name],
                               stdout=subprocess.PIPE,
                               stderr=subprocess.STDOUT,
                               text=True)
    for line in process.stdout:
        clean_line = clean_ansi_escape_codes(line.strip())
        print(clean_line)  # Debug: print the cleaned line to the server console
        socketio.emit('log', {'data': clean_line}, namespace='/logs')
        socketio.sleep(0)  # Yield control to the Socket.IO server to handle other clients/events

@app.route('/')
def index():
    return render_template_string("""
<!DOCTYPE html>
<html>
<head>
    <title>Container Logs</title>
    <script type="text/javascript" src="//cdnjs.cloudflare.com/ajax/libs/socket.io/4.0.1/socket.io.js"></script>
    <script type="text/javascript" charset="utf-8">
        var socket = io.connect('http://' + document.domain + ':' + location.port + '/logs');
        
        socket.on('log', function(msg) {
            var logsElement = document.getElementById('logs');
            // Add new log entry
            logsElement.innerHTML += msg.data + '<br>';
            // Auto-scroll to the bottom
            window.scrollTo(0, document.body.scrollHeight);
            // Limit the number of displayed log lines to 10
            var logLines = logsElement.innerHTML.split('<br>');
            if (logLines.length > 10) {
                // Remove the oldest log lines beyond the 10th
                logsElement.innerHTML = logLines.slice(logLines.length - 10).join('<br>');
            }
        });
    </script>
</head>
<body>
    <div id="logs"></div>
</body>
</html>
    """)

def handle_connect():
    """Handle new client connections and start the log streaming in a background thread."""
    # Start a new thread for streaming logs to avoid blocking
    thread = threading.Thread(target=stream_logs_to_socket, args=("pdf_cracker",), daemon=True)
    thread.start()

@socketio.on('connect', namespace='/logs')
def on_connect():
    print('Client connected')
    handle_connect()

if __name__ == '__main__':
    socketio.run(app, debug=True, host='0.0.0.0', allow_unsafe_werkzeug=True)
