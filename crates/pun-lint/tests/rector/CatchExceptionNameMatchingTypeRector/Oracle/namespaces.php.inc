<?php

namespace App {
    try {
    } catch (\Exception $e) {
    }
}

namespace {
    try {
    } catch (Exception $e) {
    }
    try {
    } catch (Exception) {
        echo $e;
    }
    try {
    } catch (Error $e) {
    }
    try {
    } catch (RuntimeException) {
        echo $e;
    }
}
-----
<?php

namespace App {
    try {
    } catch (\Exception $exception) {
    }
}

namespace {
    try {
    } catch (Exception $exception) {
    }
    try {
    } catch (Exception) {
        echo $e;
    }
    try {
    } catch (Error $error) {
    }
    try {
    } catch (RuntimeException) {
        echo $error;
    }
}
