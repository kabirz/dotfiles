#!/bin/bash
# enter bash
# source thisfile
# install_xxx

# install nvim
install_nvim() {
  pushd NvChad || exit
  ln -s "$(pwd)"/custom nvim/lua/custom
  ln -s "$(pwd)"/nvim ~/.config/nvim
  popd || exit
}

# install tmux
install_tmux() {
  git clone https://github.com/tmux-plugins/tpm ~/.tmux/plugins/tpm
  ln -s "$(pwd)"/tmux.conf ~/.tmux.conf
}

# install cargo_config
install_cargo_config() {
  ln -s "$(pwd)"/cargo_config.toml ~/.cargo/config
}

# install alacritty
install_alacritty() {
  ln -s "$(pwd)"/alacritty ~/.config/alacritty
}

# install starship
install_starship() {
  ln -s "$(pwd)"/starship.toml ~/.config/starship.toml
}

# install anaconda
install_anaconda() {
  ln -s "$(pwd)"/anaconda.yml ~/.condarc
}

# install joshuto
install_joshuto() {
  ln -s "$(pwd)"/joshuto ~/.config/joshuto
}
