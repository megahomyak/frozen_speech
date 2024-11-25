FROM busybox:1.36.1-uclibc AS build
WORKDIR /app
CMD ["httpd", "-f"]
STOPSIGNAL SIGKILL

FROM build AS production
COPY contents/compiled .

FROM build AS development
