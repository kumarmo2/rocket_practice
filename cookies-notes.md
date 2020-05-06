### NOTES
 * If the `Set-Cookie` response header, doesn't provide the `Expires or Max-Age` directive, it is a session cookie. It will be deleted from the client, once the client shuts down.
 * To help mitigate the `XSS` attack, the `httpOnly` cookies can be used, they are not accessible through javascript and are just sent to the server.
 * `Scope Of Cookies`: 
