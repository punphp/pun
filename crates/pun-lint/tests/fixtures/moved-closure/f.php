<?php

final class Rounds
{
    public function lines($output)
    {
        if (is_string($output)) {
            return 'a';
        } elseif (is_array($output)) {
            return array_map(fn ($line) => rtrim($line, "\n")."\n", $output);
        }

        return '';
    }
}
