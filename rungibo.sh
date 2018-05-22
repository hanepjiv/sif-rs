#! /bin/env sh

GIBO=gibo

${GIBO} -u | exit 1
${GIBO} \
    Archives \
    Emacs \
    Linux \
    Mercurial \
    TortoiseGit \
    Vim \
    VisualStudio \
    Windows \
    Xcode \
    macOS \
    Android \
    Rust \
    > .gitignore
