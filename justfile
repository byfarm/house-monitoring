
default:
    just --choose

compile:
    cd arduino/ && arduino-cli compile arduino.ino --fqbn esp32:esp32:esp32

upload:
    cd arduino/ && arduino-cli upload -p /dev/ttyUSB0 --fqbn esp32:esp32:esp32 .

update:
    just compile && just upload

new-location LOCATION:
    cd arduino/ && sed "s/\$lkjhasf/{{LOCATION}}/" params_template.h > params.h
