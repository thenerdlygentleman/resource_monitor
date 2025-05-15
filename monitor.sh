#!/bin/bash

ns="\033[0m"
green="\033[32m"
blue="\033[34m"
bg_white="\033[47m"

print_value() {
  value_name=$1
  value=$2
  echo -e "$blue$value_name:\t$green$value$ns"
}

monitor() {
  while true
    do
      thermal_zone0=$(echo $(($(cat /sys/class/thermal/thermal_zone0/temp) / 1000)))
      thermal_zone1=$(echo $(($(cat /sys/class/thermal/thermal_zone1/temp) / 1000)))
      total_memory=$(free -m | grep Mem | awk '{print $2}')
      free_memory=$(free -m | grep Mem | awk '{print $4}')
      free_memory_percentage=$(echo $(($free_memory / $total_memory * 100)))

      clear
      echo -e "${bg_white}${blue}$(date)${ns}"
      print_value "thermal_zone0 / °C" "$thermal_zone0"
      print_value "thermal_zone1 / °C" "$thermal_zone1"
      print_value "total_memory / MiB" "$total_memory"
      print_value "free_memory / MiB" "$free_memory"
      print_value "free_memory / %" "$free_memory_percentage"
      sleep 1
  done
}

monitor
