from flask import Flask, render_template_string
from flask_socketio import SocketIO
import subprocess
import threading

app = Flask(__name__)
socketio = SocketIO(app)

def stream_logs_to_socket(container_name):
    """Stream Docker container logs to WebSocket."""
    process = subprocess.Popen(['docker', 'logs', '-f', container_name],
                               stdout=subprocess.PIPE,
                               stderr=subprocess.STDOUT,
                               text=True)
    for line in process.stdout:
        socketio.emit('log', {'data': line.strip()}, namespace='/logs')

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
            // Limit the number of displayed log lines to 200
            var logLines = logsElement.innerHTML.split('<br>');
            if (logLines.length > 200) {
                // Remove the oldest log lines beyond the 200th
                logsElement.innerHTML = logLines.slice(logLines.length - 200).join('<br>');
            }
        });
    </script>
</head>
<body>
    <div id="logs"></div>
</body>
</html>
    """)

@socketio.on('connect', namespace='/logs')
def on_connect():
    # Start a new thread for streaming logs to avoid blocking
    threading.Thread(target=stream_logs_to_socket, args=("9d02a3ded338",)).start()

if __name__ == '__main__':
    socketio.run(app, debug=True, host='0.0.0.0')
