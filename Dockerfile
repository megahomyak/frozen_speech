FROM busybox:1.36.1-uclibc
WORKDIR /app
COPY contents/ .
STOPSIGNAL SIGKILL
CMD ["httpd", "-f"]
