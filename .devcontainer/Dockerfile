FROM docker.io/library/rust:trixie

# Setup dev user
ARG USERNAME=dev
ARG USER_UID=10001
ARG USER_GID=$USER_UID

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/home/${USERNAME}" \
    --shell "/bin/bash" \
    --uid "${USER_UID}" \
    "${USERNAME}"


USER $USERNAME

COPY --chown=$USERNAME bashrc /home/$USERNAME/.bashrc
COPY --chown=$USERNAME vimrc /home/$USERNAME/.vimrc

WORKDIR /home/$USERNAME
