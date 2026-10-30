<?php
try {
    function before(void $before) {}
    try {
        function inner(void ...$inner = []) {}
    }
    function after(void $after) {}
}
