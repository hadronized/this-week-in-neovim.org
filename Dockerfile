FROM archlinux:latest

# Paths.
RUN mkdir -p /usr/local/bin/twin
RUN mkdir -p /usr/share/twin/static
RUN mkdir -p /var/lib/twin/contents

# Resources.
ADD ./target/release/this-week-in-neovim-backend /usr/local/bin/twin/this-week-in-neovim-backend
ADD ./back/config.toml /usr/share/twin
ADD ./static /usr/share/twin/static

ENV TWIN_CONFIG=/usr/share/twin/config.toml
ENTRYPOINT /usr/local/bin/twin/this-week-in-neovim-backend
