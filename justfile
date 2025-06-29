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
    rofi -plugin-path "target/release" -show {{ PLUGIN_NAME }} -config {{ TEST_CONFIG }}

run-combi: build
    rofi -plugin-path "target/release" -combi-modes {{ PLUGIN_NAME }},drun -show combi -config {{TEST_CONFIG}}
