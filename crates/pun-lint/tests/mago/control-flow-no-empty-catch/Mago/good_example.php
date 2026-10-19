<?php

try {
    // some code that might throw an exception
} catch(Exception $e) {
    // Handle the error, log it, or re-throw it.
    error_log($e->getMessage());
}
