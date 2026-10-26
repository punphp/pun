<?php

function get_difference(string $a, string $b): string {
    // ...
}

function get_difference_case_insensitive(string $a, string $b): string {
    // ...
}

final class Connection
{
    private bool $secure;

    // The boolean is only stored, never used to branch behaviour.
    public function configure(bool $secure): void
    {
        $this->secure = $secure;
    }
}
