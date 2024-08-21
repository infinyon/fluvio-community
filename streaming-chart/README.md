Quick proof of concept for a webserver serving a websocket to a live chart for
apache echarts.

Before starting, create a fluvio cluster and `fluvio topic create foo`

Run the code w/ `cargo run`

The server has endpoints:

  http://127.0.0.1:3000/chart
  This is a static page that requests data from:

  http://127.0.0.1:3000/ws/foo
  The `ws/foo` handler will stream data from the fluvio cluster `foo` topic.

To load a sample `fluvio produce foo -f sine_wave.txt` and plot it real-time.

