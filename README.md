# Solana defi execution tools library

A trader once said:
> “slippage is toooo biggg mango-chan, YAMETE!!!”

This is a collection of general purpose tools to trade execution:
- idontsee - Mango Markets Fill-or-Kill guard
  - specify size, direction, and limit fill price, and the tx will fail if real avg fill price is outside the specified bounds.
    Essential tooling for setting up many legged trades atomically.
