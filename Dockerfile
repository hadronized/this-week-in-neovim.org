FROM archlinux:latest

# Paths.
RUN mkdir -p /usr/local/bin/twin
RUN mkdir -p /usr/share/twin/
RUN mkdir -p /var/lib/twin/contents

# Binary.
ADD ./target/release/this-week-in-neovim-backend /usr/local/bin/twin/this-week-in-neovim-backend
ADD ./back/config.toml /usr/share/twin

# Read-only frontend; it’s basically just the index.html, index.js and the WASM module.
ADD ./front/dist /usr/share/twin/frontend

ENV TWIN_CONFIG=/usr/share/twin/config.toml
ENTRYPOINT /usr/local/bin/twin/this-week-in-neovim-backend
