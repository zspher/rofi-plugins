PKGNAME := env("PKGNAME", "rofi-websearch")
PKGDIR := env("PKGDIR", "")
LIB_NAME := "lib" + snakecase(PKGNAME) + ".so"
PLUGIN_NAME := replace(PKGNAME, "rofi-", "")
PLUGINS_DIR := env("PLUGINS_DIR", `pkg-config --variable pluginsdir rofi || if test -d "/usr/lib64"; then echo "/usr/lib64/rofi"; else echo "/usr/lib/rofi"; fi`)

default:
    just build

build:
    cargo build --release --lib {{if PKGNAME == "" {""} else {"-p " + PKGNAME} }}

install:
    install -Dt "{{ clean(PKGDIR + "/" + PLUGINS_DIR) }}" "target/release/{{ LIB_NAME }}"


run: build
    ROFI_PLUGIN_PATH="target/release" rofi -modes {{ PLUGIN_NAME }},drun,run -show {{ PLUGIN_NAME }} -show-icons -config {{ join(source_directory(),"config.rasi") }}

run-combi: build
    ROFI_PLUGIN_PATH="target/release" rofi -combi-modes {{ PLUGIN_NAME }},drun -show combi  -modes {{ PLUGIN_NAME }},drun,run,combi -show-icons -config {{ join(source_directory(),"config.rasi") }}
