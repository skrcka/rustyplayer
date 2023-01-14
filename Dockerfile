FROM skrcka/rust-arm-crosscompile:latest

RUN apt-get update
RUN dpkg --add-architecture armhf && \
    apt-get update && \
    apt-get install --assume-yes libssl-dev:armhf libasound2-dev:armhf crossbuild-essential-armhf libasound2-dev:armhf portaudio19-dev:armhf libpulse-dev:armhf libdbus-1-dev:armhf && \ 
    apt-get install --assume-yes build-essential libasound2-dev portaudio19-dev libpulse-dev libdbus-1-dev 

#ENV LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/lib/arm-linux-gnueabihf/
#ENV LIB_C=/usr/lib/arm-linux-gnueabihf/libc.so.6
ENV PKG_CONFIG_LIBDIR_armv7_unknown_linux_gnueabihf=/usr/lib/arm-linux-gnueabihf/pkgconfig
ENV PKG_CONFIG_ALLOW_CROSS=1