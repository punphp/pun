<?php

if ($storedToken == $userToken) {
    // Vulnerable to timing attacks
}
-----
3:18: Insecure comparison of sensitive data.
