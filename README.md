# Azure Keyvault Encryption/Decryption in C++ and Rust

This repo has programs which retrieve a key from an Azure Key Vault and then encrypt or decrypt messages with it.

The encryption program is written in C++ and is in the `encrypt` folder. The decryption program is written in Rust and is in the `decrypt` folder.

## Building

### `encrypt`

Install [vcpkg](https://vcpkg.io/), then:

```
$ vcpkg install --triplet=x64-windows-static --feature-flags=manifests
```

Then build with CMake using your prefered method.


### `decrypt`

```
$ cargo build
```

## Running

The following environment flags are expected to be set for both `encrypt` and `decrypt`:

- `AZURE_CLIENT_ID`: Azure ActiveDirectory application registration client ID
- `AZURE_TENANT_ID`: Azure ActiveDirectory application registration tenant ID
- `AZURE_KEY_VAULT_URI`: The URI of the Azure Key Vault
- `AZURE_CLIENT_SECRET`: The generate Azure Key Vault secret used to decrypt the messages


### `encrypt`

```
$ ./azure_encrypt
```

### `decrypt`

```
$ cargo run
```
