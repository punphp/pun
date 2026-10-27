<?php

final class Rounds
{
    public function lines($output)
    {
        if (is_string($output)) {
            return 'a';
        }
        if (is_array($output)) {
            return array_map(fn ($line): string => rtrim($line, "\n")."\n", $output);
        }

        return '';
    }
}
