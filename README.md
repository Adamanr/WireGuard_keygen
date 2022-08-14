<h1>WireGuard-KeyGen</h1>

<b>Ru</b>
WireGuard-KeyGen - API разработанная на языке Rust для взаимодействия с WireGuard 

<b>Функционал API</b> 
1. Принимает json запрос с полем "ip" на /add_key
2. Создаёт приватные и публичные ключи WireGuard 
3. Отправляет ответом два ключа 
*Удаление ключа
1. Принимает json запрос с полем "key" на /delete_key
2. Удаляет в wg0.conf заданный ключ с его allowed-ips 


<b>En</b>
WireGuard-KeyGen - API developed in the Rust language for interacting with WireGuard

<b>API functionality</b>
1. Accepts a json request with the "ip" field on /add_key
2. Creates WireGuard private and public keys
3. Sends two keys in response
*Delete keys
1. Accepts a json request with the "key" field to /delete_key
2. Deletes the specified key from its allowed-ips in wg 0.conf
