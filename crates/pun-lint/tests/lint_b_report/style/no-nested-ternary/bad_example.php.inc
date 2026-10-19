<?php

$allowed = $user->isAdmin() ? true : ($user->isEditor() ? true : false);
-----
3:39: Nested ternary expressions are confusing due to PHP's operator associativity rules.
