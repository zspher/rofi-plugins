PKGNAME := env("PKGNAME", "rofi-websearch")
PKGDIR := env("PKGDIR", "")
LIB_NAME := "lib" + snakecase(PKGNAME) + ".so"
PLUGIN_NAME := replace(PKGNAME, "rofi-", "")
PLUGINS_DIR := env("PLUGINS_DIR", `pkg-config --variable pluginsdir rofi || if test -d "/usr/lib64"; then echo "/usr/lib64/rofi"; else echo "/usr/lib/rofi"; fi`)

TEST_CONFIG := join(source_directory(),"test/config.rasi")

default:
    just build

build:
    cargo build --release --lib {{ if PKGNAME == "" {""} else {"-p " + PKGNAME} }}

install:
    install -Dt "{{ clean(PKGDIR + "/" + PLUGINS_DIR) }}" "target/release/{{ LIB_NAME }}"


run: build
    ROFI_PLUGIN_PATH="target/release" rofi  -show {{ PLUGIN_NAME }} -config {{ TEST_CONFIG }}

run-combi: build
    ROFI_PLUGIN_PATH="target/release" rofi -combi-modes drun,{{ PLUGIN_NAME }} -show combi -config {{TEST_CONFIG}}
