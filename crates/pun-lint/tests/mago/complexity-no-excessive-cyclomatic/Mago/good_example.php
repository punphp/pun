<?php

function validateUser($user) {
    if (!isValidEmail($user['email'])) {
        return false;
    }

    if (!isValidAge($user['age'])) {
        return false;
    }

    if (!hasRequiredFields($user)) {
        return false;
    }

    return true;
}

function isValidEmail($email) {
    return filter_var($email, FILTER_VALIDATE_EMAIL) !== false;
}

function isValidAge($age) {
    return $age >= 18 && $age <= 120;
}

function hasRequiredFields($user) {
    return isset($user['name']) && isset($user['email']);
}
