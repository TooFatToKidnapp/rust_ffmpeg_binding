#!/bin/bash

set -ex

source /vagrant/.ssh_env

apt update
apt upgrade -y
apt install curl clang git libavformat-dev libavcodec-dev libavutil-dev pkg-config libavfilter-dev libavdevice-dev -y


# echo "PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig:$PKG_CONFIG_PATH" >> /home/vagrant/.bashrc
# source /home/vagrant/.bachrc


echo -e "nameserver 8.8.8.8\nnameserver 1.1.1.1" | sudo tee /etc/resolv.conf

echo $SSH_KEY >> /home/vagrant/.ssh/authorized_keys
