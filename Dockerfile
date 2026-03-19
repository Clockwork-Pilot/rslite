FROM rust:bookworm

ARG TZ
ENV TZ="$TZ"

ARG CLAUDE_CODE_VERSION=latest
ENV DEVCONTAINER=true

# Install basic development tools and iptables/ipset
RUN apt-get update && apt-get install -y --no-install-recommends \
  less \
  git \
  procps \
  sudo \
  fzf \
  man-db \
  unzip \
  gnupg2 \
  gh \
  iptables \
  ipset \
  iproute2 \
  dnsutils \
  aggregate \
  jq \
  lsb-release \
  nano \
  vim \
  wget \
  curl \
  tcl \
  tcllib \
  tcl8.6-dev \
  gdb \ 
  tcl-dev \
  tcl8.6-tdbc \
  tcl8.6 \
  tcl8.6-tdbc-sqlite3 \
  libicu-dev \
  build-essential \
  && apt-get clean && rm -rf /var/lib/apt/lists/*


RUN curl -fsSL https://github.com/tmux/tmux-builds/releases/download/v3.6a/tmux-3.6a-linux-x86_64.tar.gz \
  | tar -xz -C /usr/local/bin tmux


ARG USERNAME=node

RUN useradd -m -u 1000 $USERNAME
# Persist bash history
# RUN SNIPPET="export PROMPT_COMMAND='history -a' && export HISTFILE=/commandhistory/.bash_history" \
#   && mkdir /commandhistory \
#   && touch /commandhistory/.bash_history \
#   && echo "$SNIPPET" >> /home/$USERNAME/.bashrc \
#   && chown -R $USERNAME /commandhistory


# Create workspace and config directories
RUN mkdir -p /workspace /home/node/ && \
  chown -R node:node /workspace /home/node/


RUN curl -fsSL https://claude.ai/install.sh | bash

RUN mkdir -p /sqlite && \
  chown -R node:node /sqlite
  
USER node
RUN cd /sqlite && \
    wget https://sqlite.org/2026/sqlite-src-3510200.zip && \
    ls /sqlite && \
    unzip /sqlite/sqlite-src-3510200.zip -d /sqlite && \
    rm /sqlite/sqlite-src-3510200.zip && \
    mv /sqlite/sqlite-src-3510200/* /sqlite/ && \
    rm -rf /sqlite/sqlite-src-3510200 && \
    cd /sqlite && \
    ./configure --all --disable-amalgamation && make && rm *.o


# Pull init-firewall.sh directly from the upstream repository
# ARG CLAUDE_CODE_REPO_REF=main
# RUN curl -fsSL \
#   "https://raw.githubusercontent.com/anthropics/claude-code/${CLAUDE_CODE_REPO_REF}/.devcontainer/init-firewall.sh" \
#   -o /workspace/init-firewall.sh

# Set up non-root user


USER root 

RUN chown -R node:node /home/node

RUN ln -s /usr/include/tcl/tcl.h /usr/include/tcl.h \
	&& ln -s /usr/include/tcl/tclOODecls.h /usr/include/tclOODecls.h \
	&& ln -s /usr/include/tcl/tclPlatDecls.h /usr/include/tclPlatDecls.h \
	&& ln -s /usr/include/tcl/tclDecls.h /usr/include/tclDecls.h \
	&& ln -s /usr/include/tcl/tclTomMath.h /usr/include/tclTomMath.h \
	&& ln -s /usr/include/tcl/tclTomMathDecls.h /usr/include/tclTomMathDecls.h \
	&& ln -s /usr/lib/tclConfig.sh /usr/lib64/tclConfig.sh

RUN usermod -aG tty $USERNAME

WORKDIR /workspace
USER node

ENTRYPOINT ["/bin/bash"]