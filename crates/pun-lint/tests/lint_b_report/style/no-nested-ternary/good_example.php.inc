<?php

if ($user->isAdmin()) {
    $allowed = true;
} else {
    $allowed = $user->isEditor();
}
-----
