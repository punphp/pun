<?php

$storedToken == $userToken;
$storedToken != $userToken;
$storedToken === $userToken;
$storedToken !== $userToken;
$password === 'hardcoded';
'hardcoded' !== ($password);
($password = 'hardcoded') === $provided;
$provided === ($value = $password);
-----
3:14: Insecure comparison of sensitive data.
4:14: Insecure comparison of sensitive data.
5:14: Insecure comparison of sensitive data.
6:14: Insecure comparison of sensitive data.
7:11: Insecure comparison of sensitive data.
8:13: Insecure comparison of sensitive data.
9:27: Insecure comparison of sensitive data.
10:11: Insecure comparison of sensitive data.
