FROM gitpod/workspace-full

# Create a place for tools
RUN mkdir ~/tools
ENV PATH="/home/gitpod/tools:${PATH}"
