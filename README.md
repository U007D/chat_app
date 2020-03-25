# Chat App README

A XAND Learning project to create an async chat app to enable users to talk to each other.

## Requirements for this app:

* Users should be able to send messages asynchronously to each other
* Users should see messages they send and receive
* Users should know the author and date/time of each message
* Users should be able to edit messages before sending
* All users see every message

## Getting started

### Linux

Get the dependencies for [iced](https://docs.rs/iced/0.1.0-beta/iced/) to work properly:

```
sudo apt-get install cmake libasound2-dev libexpat1-dev libfreetype6-dev libssl-dev libxcb-composite0-dev libxss-dev libvulkan1 mesa-vulkan-drivers openbox pkg-config vulkan-utils xorg 
sudo add-apt-repository ppa:oibaf/graphics-drivers
sudo apt update && sudo apt upgrade
```

Then, use Rust to build the app:
```
cargo build
```

# Reference

Algorithm [link](https://docs.google.com/document/d/1xXNIU1jg0hHy1M5lFzREX0EXfa6K-rapz4_ziC3wu6g/edit?usp=sharing)
(read-only)