FROM busybox:1.36.1-uclibc
WORKDIR /app
COPY new_contents/ .
STOPSIGNAL SIGKILL
CMD ["httpd", "-f"]
