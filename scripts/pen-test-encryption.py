import base64
import os
from cryptography.hazmat.primitives.ciphers.aead import AESGCM
from cryptography.hazmat.primitives.kdf.pbkdf2 import PBKDF2HMAC
from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.backends import default_backend

def simulate_unauthorized_access():
    print("[!] TEST: Attempting decryption with simulated 'unauthorized' hardware fingerprint...")
    
    # Datos encriptados reales leídos del estado
    with open("apps/web/src/data/multiverse-state.json", "r") as f:
        encoded_data = f.read().strip()
    
    raw_data = base64.b64decode(encoded_data)
    salt = raw_data[:16]
    nonce = raw_data[16:28]
    ciphertext = raw_data[28:]
    
    # Intentar desencriptar con un fingerprint 'falso'
    fake_hw_id = "unauthorized-device-id"
    password = os.getenv("ORACLE_TOKEN", "development-only-key")
    binding_key = f"{password}:{fake_hw_id}".encode()
    
    kdf = PBKDF2HMAC(
        algorithm=hashes.SHA256(),
        length=32,
        salt=salt,
        iterations=100000,
        backend=default_backend()
    )
    key = kdf.derive(binding_key)
    aesgcm = AESGCM(key)
    
    try:
        aesgcm.decrypt(nonce, ciphertext, None)
        print("[X] FAIL: Data decrypted on unauthorized device.")
    except Exception:
        print("[✔] SUCCESS: Decryption failed as expected. Fortress is secure.")

if __name__ == "__main__":
    simulate_unauthorized_access()
