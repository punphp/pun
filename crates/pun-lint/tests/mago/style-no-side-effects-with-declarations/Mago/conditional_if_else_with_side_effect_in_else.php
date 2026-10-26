<?php

if (defined('USE_X')) {
    class Foo {}
} else {
    echo 'fallback';
}

class Bar {}
