# Description

Нужно реализовать сервис по регистрации пользователя
- хеш регистрации (хэш всех данных) отправляется в блокчейн 
- у пользователя сохраняется закрытый ключ под паролем

## Fields

- public_key
- username
- photo_ipfs
- password_encrypted_by_secret_key
- custom_data - произвольные данные

## API

- get_user_by_public_key
- get_user_by_username
- get_all_user
- set_user - запись пользователя в блокчейн
    - на клиенте пароль\секретный ключ сохраняется на локальной машине пользователя