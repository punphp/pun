<?php

if (defined('USE_X')) {
    class Foo implements X {}
} else {
    class Foo implements Y {}
}

class Bar {}
