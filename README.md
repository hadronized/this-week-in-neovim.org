Notice: A continuation of this project now lives at https://dotfyle.com/this-week-in-neovim.\
See https://dotfyle.com/this-week-in-neovim/46#update-twin for a longer explanation.

---

# This Week In Neovim

This repository holds the source code of https://this-week-in-neovim.org (this domain does not belong to this project anymore. Use at your own risk.).

<!-- vim-markdown-toc GFM -->

* [Architecture](#architecture)
* [How does it run in production](#how-does-it-run-in-production)
* [Automatic updates every Monday](#automatic-updates-every-monday)
* [Licences](#licences)

<!-- vim-markdown-toc -->

## Architecture

The website is composed of two main pieces:

- [back](./back), the backend binary. It has different roles:
  - As a web server, it responds to requests, such as `/`, `/latest`, weekly news, RSS feeds, etc. and serves the right
    content. Served content is cached with a TTL (currently 1 day but that might change).
  - It runs in a dedicated thread a _notify_ file watcher, connected to a directory which contents is the
    [contents repository]. If a new weekly is added, it automatically
    loads it.
- [twin](./twin), the Rust library for representing weekly news, parsing, converting Markdown to HTMl, etc. etc.

## How does it run in production

1. The backend is compiled with `cargo build --release` and pushed to a remote production server.
2. On that server, a clone of this repository runs `git fetch` and `git rebase origin/master`
  (that repository _always_ stays on `master`).
3. Then, a docker image is built by running `docker build .`. See the [Dockerfile][./Dockerfile]. That process copies
   the binary into the image as long as the [server configuration](./back/config.toml), [static files](./static). A
   directory is created to be able to read from the [contents repository].
4. Once the image is built, `docker tag` is run twice: once to create the `twin:M.N.P` SemVer image, and another time to
   create `twin:latest`.
5. A docker-compose project is restarted with `twin:latest` and the webapp runs.

## Automatic updates every Monday

On every Monday, a [git script](./run/twin-refresh) is run by a [systemd unit](./run/twin-refresh.service), according to
a [timer](./run/twin-refresh.timer). That script simply `git pull --rebase` in the right host directory (which is
mounted in the docker container). For short: thanks to the notify thread and mounting the volume in the docker
container, there is no service interruption to do to release a new weekly: the only thing is to merge to `master` on the
[contents repository] before every Morning CET. The refresh date time is currently set on **Monday 9:00 AM CET**.

## Licences

The source code of the website itself (i.e. [this very repository](https://github.com/phaazon/this-week-in-neovim.org))
is licensed with the [BSD-3 New Clause](./LICENSE).

However, the actual content this is published (the “weekly news”) is licensed with CC-BY-SA-4.0. The license can be
found [here](https://github.com/phaazon/this-week-in-neovim-contents).

[contents repository]: https://github.com/phaazon/this-week-in-neovim-contents
