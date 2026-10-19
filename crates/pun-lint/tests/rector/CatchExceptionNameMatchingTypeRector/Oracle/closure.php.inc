<?php

namespace App;

use Vendor\Errors\Timeout;

try {
} catch (Timeout $e) {
    report($e);
}

$c = function () {
    try {
    } catch (\Vendor\Errors\Timeout $t) {
    }
    try {
    } catch (Timeout) {
    }
    try {
    } catch (Timeout $e) {
    }
    try {
    } catch (RuntimeException $timeout) {
    }
    x($timeout);
    try {
    } catch (RuntimeException) {
    }
    try {
    } catch (Timeout $e) {
    }
};
-----
<?php

namespace App;

use Vendor\Errors\Timeout;

try {
} catch (Timeout $timeout) {
    report($timeout);
}

$c = function () {
    try {
    } catch (\Vendor\Errors\Timeout $timeout) {
    }
    try {
    } catch (Timeout) {
    }
    try {
    } catch (Timeout $timeout) {
    }
    try {
    } catch (RuntimeException $runtimeException) {
    }
    x($runtimeException);
    try {
    } catch (RuntimeException) {
    }
    try {
    } catch (Timeout $timeout) {
    }
};
