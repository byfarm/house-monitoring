
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

exec *ARGS:
    docker compose exec -it {{ARGS}}

serial:
    stty -F /dev/ttyUSB0 raw 9600
    cat /dev/ttyUSB0
