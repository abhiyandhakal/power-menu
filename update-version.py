#!/usr/bin/env python3
import os
import sys
import json
import re

# check if version is provided
if len(sys.argv) < 2:
    print('Version is required')
    sys.exit(1)

# check if version is valid
version = sys.argv[1]

version_pattern = r'^\d+\.\d+\.\d+$'

if not re.match(version_pattern, version):
    print('Version is invalid')
    sys.exit(1)

# PACKAGE.JSON
with open('package.json', 'r') as f:
    package_json = json.load(f)

package_json['version'] = version

with open('package.json', 'w') as f:
    json.dump(package_json, f, indent=2)

# TAURI.CONF.JSON
tauri_conf_path = os.path.join('src-tauri', 'tauri.conf.json')
with open(tauri_conf_path, 'r') as f:
    tauri_conf_json = json.load(f)

tauri_conf_json['package']['version'] = version

with open(tauri_conf_path, 'w') as f:
    json.dump(tauri_conf_json, f, indent=2)

# CARGO.TOML
cargo_toml_path = os.path.join('src-tauri', 'Cargo.toml')
with open(cargo_toml_path, 'r') as f:
    cargo_toml = f.read()

cargo_toml = re.sub(r'version = "\d+\.\d+\.\d+"',
                    f'version = "{version}"', cargo_toml)

with open(cargo_toml_path, 'w') as f:
    f.write(cargo_toml)
