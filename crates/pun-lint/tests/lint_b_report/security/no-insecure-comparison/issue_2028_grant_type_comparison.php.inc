<?php

declare(strict_types=1);

namespace App;

$grantType = 'someValue';
if ($grantType === 'refresh_token') {
    echo 'Refresh Token selected';
}
-----
