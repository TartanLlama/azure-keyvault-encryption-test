#if defined(_MSC_VER)
#define _CRT_SECURE_NO_WARNINGS
#endif

#include <azure/core.hpp>
#include <azure/identity.hpp>
#include <azure/keyvault/keyvault_keys.hpp>
#include <azure/keyvault/keys/cryptography/cryptography_client.hpp>

#include <iomanip>
#include <chrono>
#include <iostream>
#include <memory>
#include <thread>

using namespace Azure::Security::KeyVault::Keys;

int main()
{
   // Set Azure credentials
   auto tenantId = std::getenv("AZURE_TENANT_ID");
   auto clientId = std::getenv("AZURE_CLIENT_ID");
   auto clientSecret = std::getenv("AZURE_CLIENT_SECRET");
   auto credential
      = std::make_shared<Azure::Identity::ClientSecretCredential>(tenantId, clientId, clientSecret);

   std::string rsaKeyName(std::getenv("AZURE_KEY_NAME"));

   // Create a client for retrieving the key
   KeyClient keyClient(std::getenv("AZURE_KEY_VAULT_URI"), credential);

   try
   {
      // Retrieve key from Azure Key Vault
      auto rsaKey = keyClient.GetKey(rsaKeyName);

      // Create client 
      auto cryptoClient = Cryptography::CryptographyClient(rsaKey.Value.Id(), credential);

      // Keep reading messages from stdin and writing the encrypted message on stdout in hex
      std::string message;
      while (std::getline(std::cin, message)) {
         // Encrypt the message with RSA
         std::vector<uint8_t> input(message.begin(), message.end());
         auto res = cryptoClient.Encrypt(Cryptography::EncryptParameters::Rsa15Parameters(input));

         // Write to stdout in hex
         for (auto c : res.Value.Ciphertext) {
            std::cout << std::setw(2) << std::setfill('0') << std::hex << (int)c;
         }
         std::cout << '\n';
      }
   }
   catch (Azure::Core::Credentials::AuthenticationException const& e)
   {
      std::cout << "Authentication Exception happened:" << std::endl << e.what() << std::endl;
      return 1;
   }
   catch (Azure::Core::RequestFailedException const& e)
   {
      std::cout << "Key Vault Client Exception happened:" << std::endl << e.Message << std::endl;
      return 1;
   }

   return 0;
}