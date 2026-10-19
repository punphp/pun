<?php

// Using a safe alternative to FFI
$data = 'some data';
$hash = hash('sha256', $data);
-----
