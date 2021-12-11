# Change lights over MQuicTT

In this toy usecase for [MQuicTT](https://github.com/mquictt/mquictt), we explore the use of a CLI application to control a "light" panel that displays the changed settings received over the _"bedroom/light"_ topic. To demo, please use the following commands:
1. Start the MQuicTT broker: 
```sh
cargo run --bin broker
```
2. Start the light-side client(This will be running on our embedded device):
```sh
cargo run --bin light-side
```
3. Start the app-side client(This will be running on our PC/control device):
```sh
cargo run --bin app-side
```

Now you will be able to control the settings of your light by setting the color and by brightness from your command line :D
