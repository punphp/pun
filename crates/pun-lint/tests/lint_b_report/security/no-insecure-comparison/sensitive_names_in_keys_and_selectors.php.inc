<?php

$data['refresh_token'] === $provided;
$provided === $data[('password')];
$object->password === $provided;
$object?->password === $provided;
$object->{'password'} === $provided;
$object?->{'password'} === $provided;
$object->getToken() === $provided;
$object->{'getToken'}() === $provided;
Credentials::getToken() === $provided;
Credentials::$password === $provided;
Credentials::PASSWORD === $provided;
${'password'} === $provided;
-----
3:24: Insecure comparison of sensitive data.
4:11: Insecure comparison of sensitive data.
5:19: Insecure comparison of sensitive data.
6:20: Insecure comparison of sensitive data.
7:23: Insecure comparison of sensitive data.
8:24: Insecure comparison of sensitive data.
9:21: Insecure comparison of sensitive data.
10:25: Insecure comparison of sensitive data.
11:25: Insecure comparison of sensitive data.
12:24: Insecure comparison of sensitive data.
13:23: Insecure comparison of sensitive data.
14:15: Insecure comparison of sensitive data.
