## Desktop Chat Software
- how to run
  - tty1: cargo run --bin server
  - tty2: cargo run --bin client Bruce
  - tty3: Cargo run --bin client Lucy 
  - tty4: telnet localhost 6142
  - tty5: telnet localhost 6142
- Screenshot
  - <img src="client.png" alt="drawing" style="width:200px;height: 300px"/>
- TODO
   - STEP2
    - Personalize settings for each client and save these settings locally.
    - Log recording.
  - STEP3
    - Add a feature to add friends.
    - Individual chat mode.
    - Group chat mode.
  - STEP4
    - Enable sending emoji images.
    - Enable sending individual files.
    - Enable sending folders.
- DONE
  - STEP0
    - [x] Preliminary framework established, capable of properly rendering the client desktop application.
  - STEP1
    - [x] Implement message sending by clicking the send button in a single client window; the current chat window should display messages sent by multiple users.
    - [x] Open multiple client windows, each automatically named.
    - [x] Enable message sending between clients and display the messages in the chat windows.
    - [x] Optimize the chat display page, with messages from others shown on the left and messages sent by oneself on the right.


## References
- [example-projects](https://github.com/DioxusLabs/example-projects)