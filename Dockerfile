FROM python:3.9.1-alpine

ENV VERSION 4.0.9
RUN adduser -D electrum && \
    apk --no-cache add gnupg && \
    apk --no-cache add --virtual build-dependencies gcc musl-dev && \
    wget https://download.electrum.org/${VERSION}/Electrum-${VERSION}.tar.gz && \
    wget https://download.electrum.org/${VERSION}/Electrum-${VERSION}.tar.gz.asc && \
    gpg --keyserver keys.gnupg.net --recv-keys 6694D8DE7BE8EE5631BED9502BD5824B7F9470E6 && \
    gpg --verify Electrum-${VERSION}.tar.gz.asc Electrum-${VERSION}.tar.gz \
    pip3 install Electrum-${VERSION}.tar.gz && \
        rm -f Electrum-${VERSION}.tar.gz && \
        apk del build-dependencies
RUN mkdir -p /data \
            /home/electrum/.electrum/wallets/ \
            /home/electrum/.electrum/testnet/wallets/ \
            /home/electrum/.electrum/regtest/wallets/ \
            /home/electrum/.electrum/simnet/wallets/ && \
            ln -sf /home/electrum/.electrum/ /data && \
            chown -R electrum /home/electrum/.electrum /data
USER electrum
WORKDIR /home/electrum
VOLUME /data
COPY docker-entrypoint.sh /usr/local/bin
ENTRYPOINT ["docker-entrypoint.sh"]
CMD ["electrum"]
