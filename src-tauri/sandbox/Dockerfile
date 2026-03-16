FROM ubuntu:22.04

ARG OPENCODE_VERSION=latest

# Avoid interactive prompts during package installation
ENV DEBIAN_FRONTEND=noninteractive

# Install base development tools
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    git \
    ca-certificates \
    build-essential \
    python3 \
    python3-pip \
    nodejs \
    npm \
    jq \
    && rm -rf /var/lib/apt/lists/*

# Install OpenCode CLI
COPY opencode-cli /usr/local/bin/opencode
RUN chmod +x /usr/local/bin/opencode || true

# Create non-root user
RUN groupadd -r sandbox && useradd -r -g sandbox -m -s /bin/bash sandbox

# Create workspace directory
RUN mkdir -p /workspace && chown sandbox:sandbox /workspace

# Create writable directories needed at runtime
RUN mkdir -p /tmp /run /home/sandbox && \
    chown -R sandbox:sandbox /home/sandbox

# Switch to non-root user
USER sandbox
WORKDIR /workspace

# Default: keep container alive for exec commands
CMD ["sleep", "infinity"]
