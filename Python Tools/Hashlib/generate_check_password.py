import uuid
import hashlib
 
def hash_password(password):
    # uuid is used to generate a random number
    salt = uuid.uuid4().hex
    return hashlib.sha256(salt.encode() + password.encode()).hexdigest() + ':' + salt
    
def check_password(hashed_password, user_password):
    password, salt = hashed_password.split(':')
    return password == hashlib.sha256(salt.encode() + user_password.encode()).hexdigest()
 
new_pass = input('Enter your password: ')
hashed_password = hash_password(new_pass)

print('The password hash: ' + hashed_password)
old_pass = input('Enter again the password for checking: ')
if check_password(hashed_password, old_pass):
    print("Password is correct")
else:
    print("Passwords doesn't match")
