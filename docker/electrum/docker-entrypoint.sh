#!/usr/bin/env sh
set -ex

# Network switch
if [ "$ELECTRUM_TESTNET" = true ] || [ "$ELECTRUM_NETWORK" = "testnet" ]; then
  FLAGS='--testnet'
elif [ "$ELECTRUM_NETWORK" = "regtest" ]; then
  FLAGS='--regtest'
elif [ "$ELECTRUM_NETWORK" = "simnet" ]; then
  FLAGS='--simnet'
fi

# Graceful shutdown
trap 'pkill -TERM -P1; electrum daemon stop; exit 0' SIGTERM

# Set config
electrum setconfig rpcuser ${ELECTRUM_USER} $FLAGS --offline
electrum setconfig rpcpassword ${ELECTRUM_PASSWORD} $FLAGS --offline
electrum setconfig rpchost 0.0.0.0 $FLAGS --offline
electrum setconfig rpcport 7000 $FLAGS --offline

# XXX: Check load wallet or create

# Run application
electrum daemon -d $FLAGS

# Wait forever
while true; do
  tail -f /dev/null &
  wait ${!}
done
