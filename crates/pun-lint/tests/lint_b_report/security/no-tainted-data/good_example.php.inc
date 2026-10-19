<?php

// Properly escape data before using a sink like `echo`
echo htmlspecialchars($_GET['name'] ?? '', ENT_QUOTES, 'UTF-8');
-----
