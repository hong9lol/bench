#!/bin/bash

# Isolation CPU Core #1 
echo "Updating GRUB to isolate CPU core 1..."
sudo sed -i 's/GRUB_CMDLINE_LINUX_DEFAULT="[^"]*/& isolcpus=1/' /etc/default/grub

# GRUB Update
echo "Updating GRUB and applying CPU isolation..."
sudo update-grub

# install utils
echo "Setting CPU frequency scaling to performance mode..."
sudo apt-get update
sudo apt-get install -y cpufrequtils

sudo cpufreq-set -r -g performance
echo "CPU frequency scaling set to performance mode to stop frequency scaling"

# for perf
sudo apt-get update
sudo apt-get install linux-tools-common linux-tools-generic linux-tools-$(uname -r)

sudo reboot

# Run program with core #1
#taskset -c 1 ./your_executable
