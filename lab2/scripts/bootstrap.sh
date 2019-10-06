#!/bin/bash

sh scripts/conan-install.sh

sh scripts/build-fmt.sh
sh scripts/build-spdlog.sh
