# Wrapper Specific Commands
w3 build
w3 codegen

# *Wrapper Commands
pw wrap build
pw wrap codegen

# Plugin Commands
pw plugin ...
pw plugin codegen

# dApp Commadns
pw dapp ...
pw dapp codegen


# Better Wallet Handling
> What does it look like when The Hub has multiple L1 wrappers available for use?

- The plugins need to be able to dynamically handle wallet connection (if it exists)
- The setup of the wallet through the initialization of the plugin is... bad
- Network detection should be able to be done under the covers
