<?php

if ($password == $input) {}
if ($a != $user->token) {}
if ($user?->secret === $b) {}
if (User::$apiKey !== $c) {}
if (Config::API_KEY <> $d) {}
if ($x == $obj->getPassword()) {}
if (Foo::getToken() === $y) {}
if ($data['password'] == $z) {}
if (($token = fetch()) == $w) {}
if ($w == ($v = $secret)) {}
if ($token == '') {}
if ('' === $token) {}
if ($token == null) {}
if ($token === 0) {}
if (($token) == ('')) {}
if ($token == 'abc') {}
if ('refresh_token' === $type) {}
if ($type === 'refresh_token') {}
if ($$token == $a) {}
if (${'password'} == $a) {}
if ($obj->{'token'} == $a) {}
if ($obj->$secret == $a) {}
if (Foo::{'token'} == $a) {}
if (PASSWORD == $a) {}
if (\PASSWORD == $a) {}
if ($token < $a) {}
if ($token . 'x') {}
if ($foo == $bar) {}
if ($a == '--password') {}
if (Foo::class == $token) {}
if ($t == $o->m()) {}
if ($get_token == $x) {}
if ($gettoken == $x) {}
if ($x == f()) {}
if ($x == Foo::BAR) {}
if ($x == Foo::bar()) {}
if ($x == $o?->bar) {}
if ($x[$k = $token] == $y) {}
if (${$secret = 1} == $y) {}
-----
3:15: Insecure comparison of sensitive data.
4:8: Insecure comparison of sensitive data.
5:20: Insecure comparison of sensitive data.
6:19: Insecure comparison of sensitive data.
7:21: Insecure comparison of sensitive data.
8:8: Insecure comparison of sensitive data.
9:21: Insecure comparison of sensitive data.
10:23: Insecure comparison of sensitive data.
11:24: Insecure comparison of sensitive data.
12:8: Insecure comparison of sensitive data.
18:12: Insecure comparison of sensitive data.
21:13: Insecure comparison of sensitive data.
22:19: Insecure comparison of sensitive data.
23:21: Insecure comparison of sensitive data.
24:19: Insecure comparison of sensitive data.
25:20: Insecure comparison of sensitive data.
32:16: Insecure comparison of sensitive data.
34:16: Insecure comparison of sensitive data.
35:15: Insecure comparison of sensitive data.
40:21: Insecure comparison of sensitive data.
41:20: Insecure comparison of sensitive data.
