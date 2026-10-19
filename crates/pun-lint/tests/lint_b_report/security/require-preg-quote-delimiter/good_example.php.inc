<?php

// The delimiter is provided, ensuring it gets escaped if necessary.
$pattern = '/' . preg_quote( $user_input, '/' ) . '/';
-----
