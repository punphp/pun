<?php

// If $user_input contains '/', the regex will be invalid.
$pattern = '/' . preg_quote( $user_input ) . '/';
-----
4:18: Missing delimiter argument in `preg_quote()` call
