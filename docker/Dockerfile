FROM erikvl87/languagetool:latest

# Environment variables for configuration
ENV Java_Xms=512m
ENV Java_Xmx=1g
ENV langtool_pipelinePrewarming=true

# Port configuration
EXPOSE 8010

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=60s \
  CMD curl -f http://localhost:8010/v2/languages || exit 1
