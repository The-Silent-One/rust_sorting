FROM rust:latest

ARG UID
ARG GID

ENV HOME=/home/sam

RUN mkdir ${HOME}
RUN chown -R ${UID}:${GID} ${HOME}

WORKDIR ${HOME}
COPY ./hello_world ./hello_world

USER ${UID}
CMD ["bash -c /bin/bash"]