<?php

// This is considered unsafe:
echo $_GET['name'] ?? '';
-----
4:6: Tainted data passed to a sink function/construct.
