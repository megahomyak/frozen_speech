FROM alpine:3.20.3 AS compiler
WORKDIR /app
RUN apk add --no-cache python3=3.12.3
COPY contents .
RUN python -B compile.py

FROM busybox:1.36.1-uclibc AS build-env
WORKDIR /app
CMD ["httpd", "-f"]
STOPSIGNAL SIGKILL

FROM build-env AS production
COPY --from=compiler contents/compiled .

FROM build-env AS development
