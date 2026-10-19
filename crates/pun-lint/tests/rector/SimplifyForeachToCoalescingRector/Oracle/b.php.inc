<?php

final class CommentOnlyLoop
{
    public function find(string $needle)
    {
        $items = ['a' => 1];
        foreach ($items as $key => $value) {
            // nothing yet
        }

        return null;
    }

    public function findInIf(string $needle)
    {
        $items = ['a' => 1];
        foreach ($items as $key => $value) {
            if ($key === $needle) {
                // nothing yet
            }
        }

        return null;
    }
}
