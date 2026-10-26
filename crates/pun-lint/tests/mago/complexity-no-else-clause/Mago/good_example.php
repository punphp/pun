<?php

function process($user) {
    if (!$user->isVerified()) {
        return; // Early return
    }

    // "Happy path" continues here
    $user->login();
}
