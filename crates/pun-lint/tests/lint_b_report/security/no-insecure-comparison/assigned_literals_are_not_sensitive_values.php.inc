<?php

($grantType = 'refresh_token') === $expected;
$expected !== ($grantType = ('refresh_token'));
($grantType = $other = 'password') == $expected;
-----
