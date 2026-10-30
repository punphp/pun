<?php

final class NullableParam
{
    public function go(): void
    {
        $this->run(['a']);
        $this->run(null);
    }

    private function run(?array $items): void
    {
    }
}
