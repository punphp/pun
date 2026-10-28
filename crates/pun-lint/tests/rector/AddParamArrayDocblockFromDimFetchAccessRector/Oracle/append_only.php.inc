<?php

final class AppendOnly
{
    private function process(array $data): void
    {
        $data[] = 1;
    }
}
