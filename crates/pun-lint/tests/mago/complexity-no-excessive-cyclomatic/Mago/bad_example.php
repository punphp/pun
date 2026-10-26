<?php

function validateUser($user) {
    if (!isset($user['email'])) {
        return false;
    }

    if (!filter_var($user['email'], FILTER_VALIDATE_EMAIL)) {
        return false;
    }

    if (!isset($user['age'])) {
        return false;
    }

    if ($user['age'] < 18) {
        return false;
    }

    if ($user['age'] > 120) {
        return false;
    }

    if (!isset($user['name'])) {
        return false;
    }

    if (strlen($user['name']) < 2) {
        return false;
    }

    if (!isset($user['country'])) {
        return false;
    }

    if (!in_array($user['country'], ['US', 'UK', 'CA'])) {
        return false;
    }

    if (isset($user['phone'])) {
        if (!preg_match('/^\d{10}$/', $user['phone'])) {
            return false;
        }
    }

    if (isset($user['preferences'])) {
        if (is_array($user['preferences'])) {
            foreach ($user['preferences'] as $key => $value) {
                if ($key === 'newsletter') {
                    if ($value !== true && $value !== false) {
                        return false;
                    }
                }
            }
        }
    }

    if (isset($user['address'])) {
        if (!isset($user['address']['street'])) {
            return false;
        }
        if (!isset($user['address']['city'])) {
            return false;
        }
    }

    return true;
}
