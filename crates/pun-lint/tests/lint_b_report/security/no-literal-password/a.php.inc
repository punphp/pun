<?php

$password = 'secret';
$token = 123;
$apiKey = '';
$secret = 1.5;
$api_key = getenv('X');
$x = ['password' => 'abc', 'name' => 'x', 'token' => '', 'secret' => 42, $k => 'v', 'a', "api_key" => 'z'];
const PASSWORD = 'x';
const TOKEN = PASSWORD;
const NAME = 'x';
class A {
    const SECRET = 'abc';
    const OTHER = 'abc';
    public $password = 'x';
    public $name = 'x';
    public $token;
    public function f($password = 'x', $token = null, $name = 'x', $secret) {}
}
f(password: 'x', name: 'y', token: $t);
$user->password = 'x';
User::$token = 9;
$data['secret'] = 'abc';
$a = $b = 'x';
$getPassword = 'q';
$c = ['--password' => 'x'];
$c = ['$password' => 'x', '$get_token' => 'y'];
-----
3:1: Literal passwords or sensitive data should not be stored in code.
4:1: Literal passwords or sensitive data should not be stored in code.
8:7: Literal passwords or sensitive data should not be stored in code.
8:58: Literal passwords or sensitive data should not be stored in code.
8:90: Literal passwords or sensitive data should not be stored in code.
9:7: Literal passwords or sensitive data should not be stored in code.
13:11: Literal passwords or sensitive data should not be stored in code.
15:12: Literal passwords or sensitive data should not be stored in code.
18:23: Literal passwords or sensitive data should not be stored in code.
20:3: Literal passwords or sensitive data should not be stored in code.
21:8: Literal passwords or sensitive data should not be stored in code.
22:7: Literal passwords or sensitive data should not be stored in code.
23:7: Literal passwords or sensitive data should not be stored in code.
25:1: Literal passwords or sensitive data should not be stored in code.
27:7: Literal passwords or sensitive data should not be stored in code.
27:27: Literal passwords or sensitive data should not be stored in code.
