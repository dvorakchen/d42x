import aesjs from "aes-js";

const AES_KEY = import.meta.env.VITE_AES_KEY ?? "";
const AES_IV = import.meta.env.VITE_AES_IV ?? "";

/**
 * decrpt a cipher text
 * @param {string} ciphertext cipher text hex format
 * @returns plain text
 */
export function decrypt(ciphertext: string) {
  const aes = getDecrypter();

  let bytes = aesjs.utils.hex.toBytes(ciphertext);
  let plaintext = aes.decrypt(bytes);
  plaintext = aesjs.padding.pkcs7.strip(plaintext);

  return aesjs.utils.utf8.fromBytes(plaintext);
}

function getDecrypter() {
  return new aesjs.ModeOfOperation.cbc(
    aesjs.utils.utf8.toBytes(AES_KEY),
    aesjs.utils.utf8.toBytes(AES_IV)
  );
}
