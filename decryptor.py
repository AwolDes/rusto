from base64 import b64decode

from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes
from cryptography.hazmat.backends import default_backend

key = b64decode(b'NvDy+u51EfMC+amJzoJO+w==')
nonce = b64decode(b'd5+SLyfPGUSeug50nK1WGA==')
ct = b64decode(b'vTVxjyUms4Z4jex/OcMcQlY=')
backend = default_backend()
cipher = Cipher(algorithms.AES(key), modes.CTR(nonce), backend=backend)
decryptor = cipher.decryptor()
print(decryptor.update(ct) + decryptor.finalize())
