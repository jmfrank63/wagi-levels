set dotenv-load

watch level:
    watchexec -e rs -r -w ./{{level}} just run {{level}}

run level: (build level "--target wasm32-wasi")
    wagi -c modules.toml --log-dir ./{{level}}/logs

run-native level:
    cd {{justfile_directory()}}/{{level}}; cargo run

build level target:
    cd {{justfile_directory()}}/{{level}}; cargo build {{target}}
