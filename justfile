set shell := ["powershell.exe", "-c"]

default:
    cargo run

list:
    cargo run list

add:
    cargo run add

remove:
    cargo run remove

update:
    cargo run update

help:
    cargo run -- --help



