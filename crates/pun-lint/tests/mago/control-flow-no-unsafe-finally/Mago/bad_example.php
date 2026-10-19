<?php

function example(): int {
    try {
        return get_value();
    } finally {
        return 42; // Unsafe control flow statement in finally block
    }
}
