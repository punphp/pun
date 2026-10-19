<?php

function example(): int {
    try {
        return get_value();
    } finally {
        // no control flow statements
    }
}
