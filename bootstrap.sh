#!/usr/bin/env bash

export DEBIAN_FRONTEND=noninteractive
sudo apt-get -yq install curl gcc vim git device-tree-compiler g++-arm-linux-gnueabihf
echo 'curl https://sh.rustup.rs -sSf | sh -s -- -y;' | su vagrant

cd /vagrant/st7701s
/home/vagrant/.cargo/bin/rustup target add armv7-unknown-linux-gnueabihf
