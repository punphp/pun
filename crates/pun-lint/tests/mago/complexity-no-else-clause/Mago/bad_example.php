<?php

function process($user) {
    if ($user->isVerified()) {
        // "Happy path" is nested
        $user->login();
    } else {
        // Logic is split across branches
        return;
    }
}
