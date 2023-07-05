# dotfiles

## get source

```shell
    git clone --recursive https://github.com/kabirz/dotfiles ~/dotfiles
```

## install

```shell
    bash
    source install.sh
```

### zim

```shell
    pushd zimfw
    ./install.sh
    popd
```

### nvim

```shell
    install_nvim
```

### tmux

```shell
    install_tmux
```

then open tmux and input key prefix + I for install tmux plugin
apply network plugin for tmux

```shell
    cd ~/.tmux/plugins/tmux
    git am ~/dotfiles/0001-network-show-ip-address.patch
```

### alacritty

```shell
    install_alacritty
```

### starship

```shell
    install_starship
```

### cargo_config

```shell
    install_cargo_config

```
